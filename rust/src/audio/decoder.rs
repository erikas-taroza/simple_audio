use cpal::traits::StreamTrait;
use rb::RbConsumer;
use symphonia::{core::{formats::{FormatOptions, FormatReader, SeekTo, SeekMode}, meta::MetadataOptions, io::{MediaSourceStream, MediaSource}, probe::Hint, units::Time}, default};

use crate::utils::{progress_state_stream::*, playback_state_stream::update_playback_state_stream};

use super::{cpal_output::CpalOutput, controls::*};

#[derive(Default)]
pub struct Decoder;

impl Decoder
{
    pub fn open_stream(&mut self, source:Box<dyn MediaSource>)
    {
        let mss = MediaSourceStream::new(source, Default::default());

        let format_options = FormatOptions { enable_gapless: true, ..Default::default() };
        let metadata_options:MetadataOptions = Default::default();

        match default::get_probe().format(&Hint::new(), mss, &format_options, &metadata_options)
        {
            Err(err) => panic!("ERR: Failed to probe source. {err}"),
            Ok(mut probed) => self.decode_loop(&mut probed.format)
        }
    }

    fn decode_loop(&mut self, reader:&mut Box<dyn FormatReader>)
    {
        let track = reader.default_track().unwrap();
        let track_id = track.id;

        let mut decoder = default::get_codecs().make(&track.codec_params, &Default::default()).unwrap();
        let mut cpal_output:Option<CpalOutput> = None;

        // Used only for outputting the current position and duration.
        let timebase = track.codec_params.time_base.unwrap();
        let duration = track.codec_params.n_frames.map(|frames| track.codec_params.start_ts + frames).unwrap();
        let duration = timebase.calc_time(duration).seconds;

        let mut lock = PROGRESS.write().unwrap();
        *lock = ProgressState { position: 0, duration };
        drop(lock);

        // Clone a receiver to listen for the stop signal.
        let lock = TXRX.read().unwrap();
        let rx = lock.as_ref().unwrap().1.clone();
        drop(lock);

        loop
        {
            // Poll the status of the RX in lib.rs.
            // If the value is ThreadMessage::Quit, that means we want to stop this stream.
            // Breaking the loop drops everything which stops the cpal stream.
            match rx.try_recv()
            {
                Err(_) => (),
                Ok(message) => match message
                {
                    ThreadMessage::Play => {
                        if let Some(cpal_output) = cpal_output.as_ref()
                        { cpal_output.stream.play().unwrap(); } 
                    },
                    ThreadMessage::Pause => {
                        if let Some(cpal_output) = cpal_output.as_ref()
                        { cpal_output.stream.pause().unwrap(); } 
                    },
                    ThreadMessage::Quit => break,
                }
            }

            if !IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst)
            { continue; }

            // Seeking.
            let seek_ts:u64 = if let Some(seek_ts) = *SEEK_TS.read().unwrap()
            {
                let seek_to = SeekTo::Time { time: Time::from(seek_ts), track_id: Some(track_id) };
                match reader.seek(SeekMode::Accurate, seek_to)
                {
                    Ok(seeked_to) => seeked_to.required_ts,
                    Err(_) => 0
                }
            } else { 0 };

            // Clean up seek stuff.
            if SEEK_TS.read().unwrap().is_some()
            {
                *SEEK_TS.write().unwrap() = None;
                decoder.reset();
                // Clear the ring buffer which prevents the writer
                // from blocking.
                if let Some(cpal_output) = cpal_output.as_ref()
                { let _ = cpal_output.ring_buffer_reader.skip(usize::MAX); }
                continue;
            }

            // Decode the next packet.
            let packet = match reader.next_packet()
            {
                Ok(packet) => packet,
                // An error occurs when the stream ends
                // so we handle sending the DONE update here.
                Err(_) => {
                    update_playback_state_stream(crate::utils::playback_state::PlaybackState::Done);
                    update_progress_state_stream(ProgressState { position: 0, duration: 0 });
                    *PROGRESS.write().unwrap() = ProgressState { position: 0, duration: 0 };
                    IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
                    crate::metadata::set_playback_state(crate::utils::playback_state::PlaybackState::Done);
                    break;
                }
            };

            if packet.track_id() != track_id { continue; }

            match decoder.decode(&packet)
            {
                Err(err) => println!("WARN: Failed to decode sound. {err}"),
                Ok(decoded) => {
                    if packet.ts() < seek_ts { continue; }
                    
                    // Update the progress stream with calculated times.
                    let progress = ProgressState {
                        position: timebase.calc_time(packet.ts()).seconds,
                        duration
                    };

                    update_progress_state_stream(progress);
                    *PROGRESS.write().unwrap() = progress;

                    // Write the decoded packet to CPAL.
                    if cpal_output.is_none()
                    {
                        let spec = *decoded.spec();
                        let duration = decoded.capacity() as u64;
                        cpal_output.replace(CpalOutput::build_stream(spec, duration));
                    }

                    cpal_output.as_mut().unwrap().write(decoded);
                }
            }
        }

        if let Some(cpal_ouput) = cpal_output
        { cpal_ouput.stream.pause().unwrap(); }
    }
}