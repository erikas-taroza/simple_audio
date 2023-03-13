// This file is a part of simple_audio
// Copyright (c) 2022-2023 Erikas Taroza <erikastaroza@gmail.com>
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of 
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program.
// If not, see <https://www.gnu.org/licenses/>.

use anyhow::{Context, anyhow};
use cpal::traits::StreamTrait;
use crossbeam::channel::Receiver;
use symphonia::{core::{formats::{FormatOptions, FormatReader, SeekTo, SeekMode}, meta::MetadataOptions, io::{MediaSourceStream, MediaSource}, probe::Hint, units::{Time, TimeBase}}, default};

use crate::utils::{progress_state_stream::*, playback_state_stream::update_playback_state_stream, types::*};

use super::{cpal_output::CpalOutput, controls::*};

pub struct Decoder
{
    rx: Receiver<ThreadMessage>,
    cpal_output: Option<CpalOutput>,
    playback_config: Option<PlaybackConfig>
}

impl Decoder
{
    /// Creates a new decoder.
    pub fn new() -> Self
    {
        let rx = TXRX.read().unwrap().1.clone();

        Decoder {
            rx,
            cpal_output: None,
            playback_config: None
        }
    }

    /// Starts decoding in an infinite loop.
    /// 
    /// Listens for any incoming `ThreadMessage`s.
    pub fn start(&mut self)
    {
        let mut state = DecoderState::Idle;

        loop
        {
            // Poll the status of the RX in lib.rs.
            // If the player is paused, then block this thread until a message comes in
            // to save the CPU.
            let recv: Option<ThreadMessage> = if state.is_waiting()
                // This will always be None on the first iteration
                // which is good because we don't need to block
                // on the first iteration.
                && self.cpal_output.is_some()
            {
                self.rx.recv().ok()
            } else {
                self.rx.try_recv().ok()
            };

            // TODO: Error handling for the thread messages.
            match recv
            {
                None => (),
                Some(message) => match message
                {
                    ThreadMessage::Open(source) => {
                        if let Ok(result) = self.open(source) {
                            let track = result.default_track()
                                .context("Cannot start playback. There are no tracks present in the file.").unwrap();
                            let track_id = track.id;

                            let decoder = default::get_codecs().make(&track.codec_params, &Default::default())
                                .unwrap();

                            // Used only for outputting the current position and duration.
                            let timebase = track.codec_params.time_base.unwrap();
                            let ts = track.codec_params.n_frames.map(|frames| track.codec_params.start_ts + frames).unwrap();
                            let duration = timebase.calc_time(ts).seconds;

                            self.playback_config = Some(PlaybackConfig {
                                reader: result,
                                decoder,
                                track_id,
                                timebase,
                                duration,
                            });
                        }
                    },
                    ThreadMessage::Play => {
                        state = DecoderState::Playing;

                        // Windows handles play/pause differently.
                        if cfg!(not(target_os = "windows")) {
                            if let Some(cpal_output) = &self.cpal_output {
                                let _ = cpal_output.stream.play();
                            }
                        }
                    },
                    ThreadMessage::Pause => {
                        state = DecoderState::Paused;

                        // Windows handles play/pause differently.
                        if cfg!(not(target_os = "windows")) {
                            if let Some(cpal_output) = &self.cpal_output {
                                let _ = cpal_output.stream.pause();
                            }
                        }
                    }
                    ThreadMessage::Stop => {
                        self.cpal_output = None;
                        self.playback_config = None;
                        state = DecoderState::Idle;
                    },
                    // When the device is changed/disconnected,
                    // then we should reestablish a connection.
                    // To make a new connection, dispose of the current cpal_output
                    // and pause playback. Once the user is ready, they can start
                    // playback themselves.
                    ThreadMessage::DeviceChanged => {
                        self.cpal_output = None;
                        // This method sends a `ThreadMessage::Pause` but it is
                        // ignored because `cpal_output` is `None`.
                        crate::Player::internal_pause();
                    },
                }
            }

            if !IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst)
            { continue; }

            let result = self.do_playback();

            // The playback has finished.
            // TODO: Handle stop and finish playback.
            if result.is_err() {
                self.finish_playback();
                state = DecoderState::Idle;
            }
        }
    }

    fn do_playback(&mut self) -> anyhow::Result<()>
    {
        if let Some(playback_config) = self.playback_config.as_mut()
        {
            let seek_ts: u64 = if let Some(seek_ts) = *SEEK_TS.read().unwrap()
            {
                let seek_to = SeekTo::Time { time: Time::from(seek_ts), track_id: Some(playback_config.track_id) };
                match playback_config.reader.seek(SeekMode::Coarse, seek_to)
                {
                    Ok(seeked_to) => seeked_to.required_ts,
                    Err(_) => 0
                }
            } else { 0 };

            // Clean up seek stuff.
            if SEEK_TS.read().unwrap().is_some()
            {
                *SEEK_TS.write().unwrap() = None;
                playback_config.decoder.reset();
                // Clear the ring buffer which prevents the writer
                // from blocking.
                if let Some(cpal_output) = self.cpal_output.as_ref()
                { cpal_output.ring_buffer_reader.skip_all(); }
                return Ok(());
            }

            // Decode the next packet.
            let packet = match playback_config.reader.next_packet()
            {
                Ok(packet) => packet,
                // An error occurs when the stream
                // has reached the end of the audio.
                Err(_) => {
                    if IS_LOOPING.load(std::sync::atomic::Ordering::SeqCst)
                    {
                        *SEEK_TS.write().unwrap() = Some(0);
                        crate::utils::callback_stream::update_callback_stream(Callback::PlaybackLooped);
                        return Ok(());
                    }

                    return Err(anyhow!("Finished playback."));
                }
            };

            if packet.track_id() != playback_config.track_id { return Ok(()); }

            let decoded = playback_config.decoder.decode(&packet)
                .context("Could not decode audio packet.")?;

            if packet.ts() < seek_ts { return Ok(()); }
                
            // Update the progress stream with calculated times.
            let progress = ProgressState {
                position: playback_config.timebase.calc_time(packet.ts()).seconds,
                duration: playback_config.duration
            };

            update_progress_state_stream(progress);
            *PROGRESS.write().unwrap() = progress;

            // Write the decoded packet to CPAL.
            if self.cpal_output.is_none()
            {
                let spec = *decoded.spec();
                let duration = decoded.capacity() as u64;
                self.cpal_output.replace(CpalOutput::new(spec, duration)?);
            }

            self.cpal_output.as_mut().unwrap().write(decoded);
        }

        Ok(())
    }

    fn finish_playback(&mut self)
    {
        // Tell the main thread that everything is done here.
        // tx.send(true).unwrap();

        // This code is only ran if the stream
        // reached the end of the audio.
        // if !has_reached_end { return Ok(()); }
        
        // Flushing isn't needed when the user deliberately
        // stops the stream because they no longer care about the remaining samples.
        if let Some(cpal_output) = self.cpal_output.as_mut()
        { cpal_output.flush(); }

        // Send the done message once cpal finishes flushing.
        // There may be samples left over and we don't want to
        // start playing another file before they are read.
        update_playback_state_stream(PlaybackState::Done);
        update_progress_state_stream(ProgressState { position: 0, duration: 0 });
        *PROGRESS.write().unwrap() = ProgressState { position: 0, duration: 0 };
        IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
        crate::metadata::set_playback_state(PlaybackState::Done);
    }

    fn open(&self, source: Box<dyn MediaSource>) -> anyhow::Result<Box<dyn FormatReader>>
    {
        let mss = MediaSourceStream::new(source, Default::default());
        let format_options = FormatOptions { enable_gapless: true, ..Default::default() };
        let metadata_options: MetadataOptions = Default::default();

        let probed = default::get_probe().format(
            &Hint::new(),
            mss,
            &format_options,
            &metadata_options
        ).context("Failed to create format reader.")?;

        Ok(probed.format)
    }

    // pub fn decode_(&mut self, source: Box<dyn MediaSource>) -> anyhow::Result<()>
    // {
    //     let mss = MediaSourceStream::new(source, Default::default());

    //     let format_options = FormatOptions { enable_gapless: true, ..Default::default() };
    //     let metadata_options: MetadataOptions = Default::default();

    //     let mut probed = default::get_probe().format(
    //         &Hint::new(),
    //         mss,
    //         &format_options,
    //         &metadata_options
    //     ).context("Failed to create decoder.")?;

    //     self.decode_loop_(&mut probed.format)?;

    //     Ok(())
    // }

    // fn decode_loop_(&mut self, reader: &mut Box<dyn FormatReader>) -> anyhow::Result<()>
    // {
    //     let track = reader.default_track()
    //         .context("Cannot start playback. There are no tracks present in the file.")?;
    //     let track_id = track.id;

    //     let mut decoder = default::get_codecs().make(&track.codec_params, &Default::default())?;
    //     let mut cpal_output: Option<CpalOutput> = None;

    //     // Used only for outputting the current position and duration.
    //     let timebase = track.codec_params.time_base.unwrap();
    //     let ts = track.codec_params.n_frames.map(|frames| track.codec_params.start_ts + frames).unwrap();
    //     let duration = timebase.calc_time(ts).seconds;

    //     let mut lock = PROGRESS.write().unwrap();
    //     *lock = ProgressState { position: 0, duration };
    //     drop(lock);

    //     // Clone a receiver to listen for the stop signal.
    //     let rx = TXRX.read().unwrap().1.clone();

    //     let lock = TXRX2.read().unwrap();
    //     let tx = lock.as_ref().unwrap().0.clone();
    //     drop(lock);

    //     let mut has_reached_end = false;

    //     loop
    //     {
    //         // Poll the status of the RX in lib.rs.
    //         // If the player is paused, then block this thread until a message comes in
    //         // to save the CPU.
    //         let recv: Option<ThreadMessage> = if !IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst)
    //             // This will always be None on the first iteration
    //             // which is good because we don't need to block
    //             // on the first iteration.
    //             && cpal_output.is_some()
    //         {
    //             rx.recv().ok()
    //         } else {
    //             rx.try_recv().ok()
    //         };

    //         match recv
    //         {
    //             None => (),
    //             Some(message) => match message
    //             {
    //                 ThreadMessage::Play if cfg!(not(target_os = "windows")) => {
    //                     if let Some(cpal_output) = cpal_output.as_ref()
    //                     { cpal_output.stream.play()?; }
    //                 },
    //                 ThreadMessage::Pause if cfg!(not(target_os = "windows")) => {
    //                     if let Some(cpal_output) = cpal_output.as_ref()
    //                     { cpal_output.stream.pause()?; }
    //                 },
    //                 ThreadMessage::Stop => break,
    //                 // When the device is changed/disconnected,
    //                 // then we should reestablish a connection.
    //                 // To make a new connection, dispose of the current cpal_output
    //                 // and pause playback. Once the user is ready, they can start
    //                 // playback themselves.
    //                 ThreadMessage::DeviceChanged => {
    //                     cpal_output = None;
    //                     // This method sends a `ThreadMessage::Pause` but it is
    //                     // ignored because `cpal_output` is `None`.
    //                     crate::Player::internal_pause();
    //                 }
    //                 _ => ()
    //             }
    //         }

    //         if !IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst)
    //         { continue; }

    //         // Seeking.
    //         let seek_ts: u64 = if let Some(seek_ts) = *SEEK_TS.read().unwrap()
    //         {
    //             let seek_to = SeekTo::Time { time: Time::from(seek_ts), track_id: Some(track_id) };
    //             match reader.seek(SeekMode::Coarse, seek_to)
    //             {
    //                 Ok(seeked_to) => seeked_to.required_ts,
    //                 Err(_) => 0
    //             }
    //         } else { 0 };

    //         // Clean up seek stuff.
    //         if SEEK_TS.read().unwrap().is_some()
    //         {
    //             *SEEK_TS.write().unwrap() = None;
    //             decoder.reset();
    //             // Clear the ring buffer which prevents the writer
    //             // from blocking.
    //             if let Some(cpal_output) = cpal_output.as_ref()
    //             { cpal_output.ring_buffer_reader.skip_all(); }
    //             continue;
    //         }

    //         // Decode the next packet.
    //         let packet = match reader.next_packet()
    //         {
    //             Ok(packet) => packet,
    //             // An error occurs when the stream
    //             // has reached the end of the audio.
    //             Err(_) => {
    //                 if IS_LOOPING.load(std::sync::atomic::Ordering::SeqCst)
    //                 {
    //                     *SEEK_TS.write().unwrap() = Some(0);
    //                     crate::utils::callback_stream::update_callback_stream(Callback::PlaybackLooped);
    //                     continue;
    //                 }

    //                 has_reached_end = true;
    //                 break;
    //             }
    //         };

    //         if packet.track_id() != track_id { continue; }

    //         let decoded = decoder.decode(&packet)
    //             .context("Could not decode audio packet.")?;

    //         if packet.ts() < seek_ts { continue; }
                
    //         // Update the progress stream with calculated times.
    //         let progress = ProgressState {
    //             position: timebase.calc_time(packet.ts()).seconds,
    //             duration
    //         };

    //         update_progress_state_stream(progress);
    //         *PROGRESS.write().unwrap() = progress;

    //         // Write the decoded packet to CPAL.
    //         if cpal_output.is_none()
    //         {
    //             let spec = *decoded.spec();
    //             let duration = decoded.capacity() as u64;
    //             cpal_output.replace(CpalOutput::new(spec, duration)?);
    //         }

    //         cpal_output.as_mut().unwrap().write(decoded);
    //     }

    //     // Tell the main thread that everything is done here.
    //     tx.send(true).unwrap();

    //     // This code is only ran if the stream
    //     // reached the end of the audio.
    //     if !has_reached_end { return Ok(()); }
        
    //     // Flushing isn't needed when the user deliberately
    //     // stops the stream because they no longer care about the remaining samples.
    //     if let Some(cpal_output) = cpal_output.as_mut()
    //     { cpal_output.flush(); }

    //     // Send the done message once cpal finishes flushing.
    //     // There may be samples left over and we don't want to
    //     // start playing another file before they are read.
    //     update_playback_state_stream(PlaybackState::Done);
    //     update_progress_state_stream(ProgressState { position: 0, duration: 0 });
    //     *PROGRESS.write().unwrap() = ProgressState { position: 0, duration: 0 };
    //     IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
    //     crate::metadata::set_playback_state(PlaybackState::Done);

    //     Ok(())
    // }
}

enum DecoderState
{
    Playing,
    Paused,
    Idle
}

impl DecoderState
{
    fn is_waiting(&self) -> bool {
        if let DecoderState::Idle = self {
            return true;
        }

        false
    }
}

struct PlaybackConfig {
    reader: Box<dyn FormatReader>,
    track_id: u32,
    decoder: Box<dyn symphonia::core::codecs::Decoder>,
    timebase: TimeBase,
    duration: u64
}