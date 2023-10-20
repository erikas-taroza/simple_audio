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

use std::{
    sync::{atomic::AtomicBool, Arc},
    thread::{self, JoinHandle},
};

use anyhow::{anyhow, Context};
use crossbeam::channel::Receiver;
use lazy_static::lazy_static;
use symphonia::{
    core::{
        audio::{AsAudioBufferRef, AudioBuffer},
        formats::{FormatOptions, FormatReader, SeekMode, SeekTo},
        io::{MediaSource, MediaSourceStream},
        meta::MetadataOptions,
        probe::Hint,
        units::{Time, TimeBase},
    },
    default::{self, register_enabled_codecs},
};
use symphonia_core::codecs::CodecRegistry;

use crate::{
    api::Player,
    audio::opus::OpusDecoder,
    media_controllers,
    utils::{
        callback_stream::update_callback_stream, error::Error,
        playback_state_stream::update_playback_state_stream, progress_state_stream::*, types::*,
    },
};

use super::{
    controls::*,
    cpal_output::{CpalOutput, OutputWriter},
};

lazy_static! {
    static ref CODEC_REGISTRY: CodecRegistry = {
        let mut registry = CodecRegistry::new();
        register_enabled_codecs(&mut registry);
        registry.register_all::<OpusDecoder>();
        registry
    };
}

pub struct Decoder
{
    thread_killer: Receiver<bool>,
    controls: Controls,
    state: DecoderState,
    cpal_output: CpalOutput,
    output_writer: Option<OutputWriter>,
    playback: Option<Playback>,
    preload_playback: Option<Playback>,
    /// The `JoinHandle` for the thread that preloads a file.
    preload_thread: Option<JoinHandle<anyhow::Result<Playback>>>,
}

impl Decoder
{
    /// Creates a new decoder.
    pub fn new(controls: Controls) -> Self
    {
        let thread_killer = THREAD_KILLER.get().unwrap().read().unwrap().1.clone();
        let cpal_output = CpalOutput::new(controls.clone()).unwrap();

        Decoder {
            thread_killer,
            controls,
            state: DecoderState::Idle,
            cpal_output,
            output_writer: None,
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
            if let Err(err) = self.poll_preload_thread() {
                update_callback_stream(Callback::Error(Error::Decode {
                    message: err.to_string(),
                }));
            }

            // Check for incoming `ThreadMessage`s.
            match self.listen_for_message() {
                Ok(should_break) => {
                    if should_break {
                        break;
                    }
                }
                Err(err) => {
                    update_callback_stream(Callback::Error(Error::Decode {
                        message: err.to_string(),
                    }));
                }
            }

            // Decode and output the samples.
            match self.do_playback() {
                Ok(playback_complete) => {
                    if playback_complete {
                        self.finish_playback();
                    }
                }
                Err(err) => {
                    update_callback_stream(Callback::Error(Error::Decode {
                        message: err.to_string(),
                    }));
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
            self.controls.event_handler().recv().ok()
        }
        else {
            self.controls.event_handler().try_recv().ok()
        };

        match recv {
            None => (),
            Some(message) => match message {
                PlayerEvent::Open(source, buffer_signal) => {
                    self.cpal_output.ring_buffer_reader.skip_all();
                    self.output_writer = None;
                    self.playback = Some(Self::open(source, buffer_signal)?);
                }
                PlayerEvent::Play => {
                    self.state = DecoderState::Playing;
                    self.cpal_output.play();
                }
                PlayerEvent::Pause => {
                    self.state = DecoderState::Paused;
                    self.cpal_output.pause();
                }
                PlayerEvent::Stop => {
                    self.state = DecoderState::Idle;
                    self.cpal_output.ring_buffer_reader.skip_all();
                    self.cpal_output.pause();
                    self.output_writer = None;
                    self.playback = None;
                }
                PlayerEvent::DeviceChanged => {
                    Player::internal_pause(&self.controls);
                    self.cpal_output = CpalOutput::new(self.controls.clone())?;
                    self.output_writer = None;
                }
                PlayerEvent::Preload(source, buffer_signal) => {
                    self.preload_playback = None;
                    self.controls.set_is_file_preloaded(false);
                    let handle = self.preload(source, buffer_signal);
                    self.preload_thread = Some(handle);
                }
                PlayerEvent::PlayPreload => {
                    if self.preload_playback.is_none() {
                        return Ok(false);
                    }

                    let playback = self.preload_playback.take().unwrap();
                    self.playback = Some(playback);
                    self.controls.set_is_file_preloaded(false);

                    Player::internal_play(&self.controls);
                }
                PlayerEvent::ClearPreload => {
                    self.preload_playback = None;
                    self.controls.set_is_file_preloaded(false);
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

        // Handle buffering.
        if let Some(playback) = &self.playback {
            if playback
                .buffer_signal
                .load(std::sync::atomic::Ordering::SeqCst)
            {
                self.cpal_output.ring_buffer_reader.skip_all();
                self.cpal_output.pause();
            }
            else {
                self.cpal_output.play();
            }
        }

        let playback = self.playback.as_mut().unwrap();

        // If there is audio already decoded from preloading,
        // then output that instead.
        if let Some(preload) = playback.preload.take() {
            // Write the decoded packet to CPAL.
            if self.output_writer.is_none() {
                let spec = *preload.spec();
                let duration = preload.capacity() as u64;
                self.output_writer.replace(OutputWriter::new(
                    self.controls.clone(),
                    self.cpal_output.ring_buffer_writer.clone(),
                    self.cpal_output.stream_config.clone(),
                    spec,
                    duration,
                ));
            }

            let buffer_ref = preload.as_audio_buffer_ref();
            self.output_writer.as_mut().unwrap().write(buffer_ref);

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
            self.cpal_output.ring_buffer_reader.skip_all();
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
        if self.output_writer.is_none() {
            let spec = *decoded.spec();
            let duration = decoded.capacity() as u64;
            self.output_writer.replace(OutputWriter::new(
                self.controls.clone(),
                self.cpal_output.ring_buffer_writer.clone(),
                self.cpal_output.stream_config.clone(),
                spec,
                duration,
            ));
        }

        self.output_writer.as_mut().unwrap().write(decoded);

        Ok(false)
    }

    /// Called when the file is finished playing.
    ///
    /// Flushes the output writer and sends a `Done` message to Dart.
    fn finish_playback(&mut self)
    {
        if let Some(mut output_writer) = self.output_writer.take() {
            output_writer.flush();
        }

        // Use the preloaded playback as the current playback.
        if let Some(preload) = self.preload_playback.take() {
            self.playback = Some(preload);
            self.controls.set_is_file_preloaded(false);
            update_playback_state_stream(PlaybackState::PreloadPlayed);
        }
        // Nothing is preloaded so stop like normal.
        else {
            self.state = DecoderState::Idle;
            self.cpal_output.pause();
            update_playback_state_stream(PlaybackState::Done);
            self.controls.set_is_playing(false);
            self.controls.set_is_stopped(true);
            crate::media_controllers::set_playback_state(PlaybackState::Done);
        }

        let progress_state = ProgressState {
            position: 0,
            duration: 0,
        };

        update_progress_state_stream(progress_state);
        self.controls.set_progress(progress_state);
    }

    /// Opens the given source for playback. Returns a `Playback`
    /// for the source.
    fn open(
        source: Box<dyn MediaSource>,
        buffer_signal: Arc<AtomicBool>,
    ) -> anyhow::Result<Playback>
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

        let decoder = CODEC_REGISTRY.make(&track.codec_params, &Default::default())?;

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
            buffer_signal,
            preload: None,
        })
    }

    /// Spawns a thread that decodes the first packet of the source.
    ///
    /// Returns a preloaded `Playback` when complete.
    fn preload(
        &self,
        source: Box<dyn MediaSource>,
        buffer_signal: Arc<AtomicBool>,
    ) -> JoinHandle<anyhow::Result<Playback>>
    {
        thread::spawn(move || {
            let mut playback = Self::open(source, buffer_signal.clone())?;
            // Preload
            let packet = playback.reader.next_packet()?;
            let buf_ref = playback.decoder.decode(&packet)?;

            let spec = *buf_ref.spec();
            let duration = buf_ref.capacity() as u64;

            let mut buf = AudioBuffer::new(duration, spec);
            buf_ref.convert(&mut buf);
            playback.preload = Some(buf);

            Ok(playback)
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
    buffer_signal: Arc<AtomicBool>,
    /// A buffer of already decoded samples.
    preload: Option<AudioBuffer<f32>>,
}
