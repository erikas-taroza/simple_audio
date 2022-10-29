use std::sync::RwLock;

use symphonia::{core::{formats::{FormatOptions, FormatReader, SeekTo, SeekMode}, meta::MetadataOptions, io::{MediaSourceStream, MediaSource}, probe::Hint, units::Time}, default};

use crate::src::progress_state_stream::*;

use super::{cpal_output::CpalOutput, controls::*};

static OUTPUT:RwLock<Option<CpalOutput>> = RwLock::new(None);

#[derive(Clone, Copy)]
pub struct Decoder
{
    stop:bool
}

impl Decoder
{
    pub const fn new() -> Self { Decoder { stop: false } }

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
        //let mut cpal_output:Option<CpalOutput> = None;

        // Used only for outputting the current position and duration.
        let timebase = track.codec_params.time_base.unwrap();
        let duration = track.codec_params.n_frames.map(|frames| track.codec_params.start_ts + frames).unwrap();

        loop
        {
            // If we break at the end of the loop,
            // we give the chance for a new cpal_output to be created.
            // So when we want to stop the current stream, in this case, it would just stop the next stream.
            // When we break here, the cpal_output of the next song is not made.

            // That is why cpal_output is set to none here. Because a new song means cpal_output is reinitialized.
            // to None above;
            //if self.stop { break; }

            if self.stop
            {
                {
                    let cpal_output = &*OUTPUT.write().unwrap();
                    let _ = *cpal_output.as_ref().unwrap();
                    let _ = reader;
                }

                let mut cpal_output = OUTPUT.write().unwrap();
                *cpal_output = None;
                break;
            }

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

            *SEEK_TS.write().unwrap() = None;

            // Decode the next packet.
            let packet = match reader.next_packet()
            {
                Ok(packet) => packet,
                Err(_err) => break
            };

            if packet.track_id() != track_id { continue; }

            match decoder.decode(&packet)
            {
                Err(err) => panic!("ERR: Failed to decode sound. {err}"),
                Ok(decoded) => {
                    if packet.ts() >= seek_ts
                    {
                        // Update the progress stream with calculated times.
                        update_progress_state_stream(ProgressState {
                            position: timebase.calc_time(packet.ts()).seconds,
                            duration: timebase.calc_time(duration).seconds
                        });

                        //self.write_decoded(&mut cpal_output, decoded);

                        let cpal_output = &mut *OUTPUT.write().unwrap();

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

            // println!("{}", cpal_output.is_some());
            // drop(cpal_output.as_ref().unwrap());
        }

        self.stop = false;
        println!("Finished");
    }

    // fn write_decoded(&self, cpal_output:&mut Option<CpalOutput>, decoded:AudioBufferRef)
    // {
    //     if cpal_output.is_none()
    //     {
    //         let spec = *decoded.spec();
    //         let duration = decoded.capacity() as u64;
    //         cpal_output.replace(CpalOutput::build_stream(spec, duration));
    //     }

    //     if let Some(cpal_output) = cpal_output
    //     {
    //         cpal_output.write(decoded);
    //     }
    // }

    pub fn stop(&mut self)
    {
        self.stop = true;
    }
}