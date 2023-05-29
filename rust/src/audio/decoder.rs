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

use std::thread::{self, JoinHandle};

use anyhow::{anyhow, Context};
use cpal::traits::StreamTrait;
use crossbeam::channel::Receiver;
use symphonia::{
    core::{
        audio::{AsAudioBufferRef, AudioBuffer},
        formats::{FormatOptions, FormatReader, SeekMode, SeekTo},
        io::{MediaSource, MediaSourceStream},
        meta::MetadataOptions,
        probe::Hint,
        units::{Time, TimeBase},
    },
    default,
};

use crate::{
    media_controllers,
    utils::{
        callback_stream::update_callback_stream,
        playback_state_stream::update_playback_state_stream, progress_state_stream::*, types::*,
    },
};

use super::{controls::*, cpal_output::CpalOutput};

pub struct Decoder
{
    thread_killer: Receiver<bool>,
    controls: Controls,
    state: DecoderState,
    cpal_output: Option<CpalOutput>,
    playback: Option<Playback>,
    preload_playback: Option<(Playback, CpalOutput)>,
    /// The `JoinHandle` for the thread that preloads a file.
    preload_thread: Option<JoinHandle<anyhow::Result<(Playback, CpalOutput)>>>,
}

impl Decoder
{
    /// Creates a new decoder.
    pub fn new(controls: Controls) -> Self
    {
        let thread_killer = THREAD_KILLER.read().unwrap().1.clone();

        Decoder {
            thread_killer,
            controls,
            state: DecoderState::Idle,
            cpal_output: None,
            playback: None,
            preload_playback: None,
            preload_thread: None,
        }
    }

    /// Starts decoding in an infinite loop.
    /// Listens for any incoming `ThreadMessage`s.
    ///
    /// If playing, then the decoder decodes packets
    /// until the file is done playing.
    ///
    /// If stopped, the decoder goes into an idle state
    /// where it waits for a message to come.
    pub fn start(mut self)
    {
        loop {
            // Check if the preload thread is done.
            if self.poll_preload_thread().is_err() {
                update_callback_stream(Callback::DecodeError);
            }

            // Check for incoming `ThreadMessage`s.
            match self.listen_for_message() {
                Ok(should_break) => {
                    if should_break {
                        break;
                    }
                }
                Err(_) => {
                    update_callback_stream(Callback::DecodeError);
                }
            }

            // Decode and output the samples.
            match self.do_playback() {
                Ok(playback_complete) => {
                    if playback_complete {
                        self.state = DecoderState::Idle;
                        self.finish_playback();
                    }
                }
                Err(_) => {
                    update_callback_stream(Callback::DecodeError);
                }
            }
        }
    }

    /// Listens for any incoming messages.
    ///
    /// Blocks if the `self.state` is `Idle` or `Paused`.
    ///
    /// Returns true if this thread should be stopped.
    /// Returns false otherwise.
    fn listen_for_message(&mut self) -> anyhow::Result<bool>
    {
        if self.thread_killer.try_recv().is_ok() {
            return Ok(true);
        }

        // If the player is paused, then block this thread until a message comes in
        // to save the CPU.
        let recv: Option<PlayerEvent> = if self.state.is_idle() || self.state.is_paused() {
            self.controls.event_handler().1.recv().ok()
        }
        else {
            self.controls.event_handler().1.try_recv().ok()
        };

        match recv {
            None => (),
            Some(message) => match message {
                PlayerEvent::Open(source) => {
                    self.cpal_output = None;
                    self.playback = Some(Self::open(source)?);
                }
                PlayerEvent::Play => {
                    self.state = DecoderState::Playing;

                    // Windows handles play/pause differently.
                    #[cfg(not(target_os = "windows"))]
                    if let Some(cpal_output) = &self.cpal_output {
                        cpal_output.stream.play()?;
                    }
                }
                PlayerEvent::Pause => {
                    self.state = DecoderState::Paused;

                    // Windows handles play/pause differently.
                    #[cfg(not(target_os = "windows"))]
                    if let Some(cpal_output) = &self.cpal_output {
                        cpal_output.stream.pause()?;
                    }
                }
                PlayerEvent::Stop => {
                    self.state = DecoderState::Idle;
                    self.cpal_output = None;
                    self.playback = None;
                }
                // When the device is changed/disconnected,
                // then we should reestablish a connection.
                // To make a new connection, dispose of the current cpal_output
                // and pause playback. Once the user is ready, they can start
                // playback themselves.
                PlayerEvent::DeviceChanged => {
                    self.cpal_output = None;
                    crate::Player::internal_pause(&self.controls);

                    // The device change will also affect the preloaded playback.
                    if self.preload_playback.is_some() {
                        let (playback, cpal_output) = self.preload_playback.take().unwrap();

                        self.preload_playback.replace((
                            playback,
                            CpalOutput::new(
                                self.controls.clone(),
                                cpal_output.spec,
                                cpal_output.duration,
                            )?,
                        ));
                    }
                }
                PlayerEvent::Preload(source) => {
                    self.preload_playback = None;
                    self.controls.set_is_file_preloaded(false);
                    let handle = self.preload(source);
                    self.preload_thread = Some(handle);
                }
                PlayerEvent::PlayPreload => {
                    if self.preload_playback.is_none() {
                        return Ok(false);
                    }

                    let (playback, cpal_output) = self.preload_playback.take().unwrap();
                    self.playback = Some(playback);
                    self.cpal_output = Some(cpal_output);
                    self.controls.set_is_file_preloaded(false);

                    crate::Player::internal_play(&self.controls);
                }
            },
        }

        Ok(false)
    }

    /// Decodes a packet and writes to `cpal_output`.
    ///
    /// Returns `true` when the playback is complete.
    /// Returns `false` otherwise.
    fn do_playback(&mut self) -> anyhow::Result<bool>
    {
        // Nothing to do.
        if self.playback.is_none() || self.state.is_idle() || self.state.is_paused() {
            return Ok(false);
        }

        let playback = self.playback.as_mut().unwrap();

        // If there is audio already decoded from preloading,
        // then output that instead.
        if let Some(preload) = playback.preload.take() {
            // Write the decoded packet to CPAL.
            if self.cpal_output.is_none() {
                let spec = *preload.spec();
                let duration = preload.capacity() as u64;
                self.cpal_output
                    .replace(CpalOutput::new(self.controls.clone(), spec, duration)?);
            }

            let buffer_ref = preload.as_audio_buffer_ref();
            self.cpal_output.as_mut().unwrap().write(buffer_ref);

            return Ok(false);
        }

        if let Some(seek_ts) = *self.controls.seek_ts() {
            let seek_to = SeekTo::Time {
                time: Time::from(seek_ts),
                track_id: Some(playback.track_id),
            };
            playback.reader.seek(SeekMode::Coarse, seek_to)?;
        }

        // Clean up seek stuff.
        if self.controls.seek_ts().is_some() {
            self.controls.set_seek_ts(None);
            playback.decoder.reset();
            // Clear the ring buffer which prevents the writer
            // from blocking.
            if let Some(cpal_output) = self.cpal_output.as_ref() {
                cpal_output.ring_buffer_reader.skip_all();
            }
            return Ok(false);
        }

        // Decode the next packet.
        let packet = match playback.reader.next_packet() {
            Ok(packet) => packet,
            // An error occurs when the stream
            // has reached the end of the audio.
            Err(_) => {
                if self.controls.is_looping() {
                    self.controls.set_seek_ts(Some(0));
                    crate::utils::callback_stream::update_callback_stream(Callback::PlaybackLooped);
                    return Ok(false);
                }

                return Ok(true);
            }
        };

        if packet.track_id() != playback.track_id {
            return Ok(false);
        }

        let decoded = playback
            .decoder
            .decode(&packet)
            .context("Could not decode audio packet.")?;

        let position = if let Some(timebase) = playback.timebase {
            timebase.calc_time(packet.ts()).seconds
        }
        else {
            0
        };

        // Update the progress stream with calculated times.
        let progress = ProgressState {
            position,
            duration: playback.duration,
        };

        Self::notify_media_controllers_with_progress(&self.controls.progress(), &progress);
        update_progress_state_stream(progress);
        self.controls.set_progress(progress);

        // Write the decoded packet to CPAL.
        if self.cpal_output.is_none() {
            let spec = *decoded.spec();
            let duration = decoded.capacity() as u64;
            self.cpal_output
                .replace(CpalOutput::new(self.controls.clone(), spec, duration)?);
        }

        self.cpal_output.as_mut().unwrap().write(decoded);

        Ok(false)
    }

    /// Called when the file is finished playing.
    ///
    /// Flushes `cpal_output` and sends a `Done` message to Dart.
    fn finish_playback(&mut self)
    {
        if let Some(cpal_output) = self.cpal_output.as_mut() {
            cpal_output.flush();
        }

        // Send the done message once cpal finishes flushing.
        // There may be samples left over and we don't want to
        // start playing another file before they are read.
        update_playback_state_stream(PlaybackState::Done);

        let progress_state = ProgressState {
            position: 0,
            duration: 0,
        };

        update_progress_state_stream(progress_state);
        self.controls.set_progress(progress_state);

        self.controls.set_is_playing(false);
        self.controls.set_is_stopped(true);
        crate::media_controllers::set_playback_state(PlaybackState::Done);
    }

    /// Opens the given source for playback. Returns a `Playback`
    /// for the source.
    fn open(source: Box<dyn MediaSource>) -> anyhow::Result<Playback>
    {
        let mss = MediaSourceStream::new(source, Default::default());
        let format_options = FormatOptions {
            enable_gapless: true,
            ..Default::default()
        };
        let metadata_options: MetadataOptions = Default::default();

        let probed = default::get_probe()
            .format(&Hint::new(), mss, &format_options, &metadata_options)
            .context("Failed to create format reader.")?;

        let reader = probed.format;

        let track = reader
            .default_track()
            .context("Cannot start playback. There are no tracks present in the file.")?;
        let track_id = track.id;

        let decoder = default::get_codecs().make(&track.codec_params, &Default::default())?;

        // Used only for outputting the current position and duration.
        let timebase = track.codec_params.time_base.or_else(|| {
            track
                .codec_params
                .sample_rate
                .map(|sample_rate| TimeBase::new(1, sample_rate))
        });
        let ts = track
            .codec_params
            .n_frames
            .map(|frames| track.codec_params.start_ts + frames);

        let duration = match (timebase, ts) {
            (Some(timebase), Some(ts)) => timebase.calc_time(ts).seconds,
            _ => 0,
        };

        Ok(Playback {
            reader,
            decoder,
            track_id,
            timebase,
            duration,
            preload: None,
        })
    }

    /// Spawns a thread that decodes the first packet of the source.
    ///
    /// Returns a preloaded `Playback` and `CpalOutput` when complete.
    fn preload(
        &self,
        source: Box<dyn MediaSource>,
    ) -> JoinHandle<anyhow::Result<(Playback, CpalOutput)>>
    {
        let controls = self.controls.clone();
        thread::spawn(move || {
            let mut playback = Self::open(source)?;
            // Preload
            let packet = playback.reader.next_packet()?;
            let buf_ref = playback.decoder.decode(&packet)?;

            let spec = *buf_ref.spec();
            let duration = buf_ref.capacity() as u64;

            let mut buf = AudioBuffer::new(duration, spec);
            buf_ref.convert(&mut buf);
            playback.preload = Some(buf);

            let cpal_output = CpalOutput::new(controls, spec, duration)?;
            cpal_output.stream.pause()?;

            Ok((playback, cpal_output))
        })
    }

    /// Polls the `preload_thread`.
    ///
    /// If it is finished, the preloaded file
    /// is then placed in `preload_playback`.
    fn poll_preload_thread(&mut self) -> anyhow::Result<()>
    {
        if self.preload_thread.is_none() || !self.preload_thread.as_ref().unwrap().is_finished() {
            return Ok(());
        }

        let handle = self.preload_thread.take().unwrap();
        let result = handle
            .join()
            .unwrap_or(Err(anyhow!("Could not join preload thread.")))?;

        self.preload_playback.replace(result);
        self.controls.set_is_file_preloaded(true);

        Ok(())
    }

    fn notify_media_controllers_with_progress(
        curr_progress: &ProgressState,
        new_progress: &ProgressState,
    )
    {
        // Notify OS media controllers about the new duration.
        if curr_progress.duration == 0 {
            update_callback_stream(Callback::DurationCalculated);
            media_controllers::set_duration(new_progress.duration);
        }

        // Notify OS media controllers about the new position.
        if curr_progress.position < new_progress.position {
            media_controllers::set_position(new_progress.position);
        }
    }
}

enum DecoderState
{
    Playing,
    Paused,
    Idle,
}

impl DecoderState
{
    fn is_idle(&self) -> bool
    {
        if let DecoderState::Idle = self {
            return true;
        }

        false
    }

    fn is_paused(&self) -> bool
    {
        if let DecoderState::Paused = self {
            return true;
        }

        false
    }
}

/// Holds the items related to playback.
///
/// Ex: The Symphonia decoder, timebase, duration.
struct Playback
{
    reader: Box<dyn FormatReader>,
    track_id: u32,
    decoder: Box<dyn symphonia::core::codecs::Decoder>,
    timebase: Option<TimeBase>,
    duration: u64,
    /// A buffer of already decoded samples.
    preload: Option<AudioBuffer<f32>>,
}
