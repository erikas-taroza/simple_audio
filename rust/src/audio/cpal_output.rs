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

use std::sync::{Arc, Condvar, Mutex};

use anyhow::Context;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Stream, StreamConfig,
};
use symphonia::core::audio::{AudioBufferRef, SampleBuffer, SignalSpec};

use crate::utils::blocking_rb::*;

use super::{
    controls::*,
    dsp::{normalizer::Normalizer, resampler::Resampler},
    sources::IS_STREAM_BUFFERING,
};

/// The default output volume is way too high.
/// Multiplying the volume input by this number
/// will help to reduce it.
const BASE_VOLUME: f32 = 0.7;

//TODO: Support i16 and u16 instead of only f32.
pub struct CpalOutput
{
    pub stream: Stream,
    pub ring_buffer_reader: BlockingRb<f32, Consumer>,
    ring_buffer_writer: BlockingRb<f32, Producer>,
    sample_buffer: SampleBuffer<f32>,
    resampler: Option<Resampler<f32>>,
    is_stream_done: Arc<(Mutex<bool>, Condvar)>,
    normalizer: Normalizer,
    controls: Controls,
}

impl CpalOutput
{
    /// Starts a new stream on the default device.
    pub fn new(controls: Controls, spec: SignalSpec, duration: u64) -> anyhow::Result<Self>
    {
        // Get the output config.
        let (device, config) = Self::get_config(spec)?;

        // Create a resampler only if the code is running on Windows
        // and if the output config's sample rate doesn't match the audio's.
        let resampler: Option<Resampler<f32>> =
            if cfg!(target_os = "windows") && spec.rate != config.sample_rate.0 {
                Some(Resampler::new(
                    spec,
                    config.sample_rate.0 as usize,
                    duration,
                ))
            }
            else {
                None
            };

        // Create a ring buffer with a capacity for up-to 200ms of audio.
        let channels = spec.channels.count();
        let ring_len = ((200 * spec.rate as usize) / 1000) * channels;

        // Create the buffers for the stream.
        let rb = BlockingRb::<f32>::new(ring_len);
        let rb_clone = rb.clone();
        let ring_buffer_writer = rb.0;
        let ring_buffer_reader = rb.1;

        let sample_buffer = SampleBuffer::<f32>::new(duration, spec);

        let is_stream_done = Arc::new((Mutex::new(false), Condvar::new()));

        let stream = device.build_output_stream(
            &config,
            {
                let controls = controls.clone();
                let is_stream_done = is_stream_done.clone();
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let buffering = IS_STREAM_BUFFERING.load(std::sync::atomic::Ordering::SeqCst);

                    // "Pause" the stream.
                    // What this really does is mute the stream.
                    // With only a return statement, the current sample still plays.
                    // CPAL states that `stream.pause()` may not work for all devices.
                    // `stream.pause()` is the ideal way to play/pause.
                    if (!controls.is_playing() && cfg!(target_os = "windows")) || buffering {
                        data.iter_mut().for_each(|s| *s = 0.0);

                        if buffering {
                            ring_buffer_reader.skip_all();
                        }

                        return;
                    }

                    let written = ring_buffer_reader.read(data);

                    // Notify that the stream was finished reading.
                    if written.is_none() {
                        let (mutex, cvar) = &*is_stream_done;
                        *mutex.lock().unwrap() = true;
                        cvar.notify_one();
                        return;
                    }

                    // Set the volume.
                    data[0..written.unwrap()]
                        .iter_mut()
                        .for_each(|s| *s *= BASE_VOLUME * *controls.volume());
                }
            },
            {
                let controls = controls.clone();
                move |err| {
                    match err {
                        cpal::StreamError::DeviceNotAvailable => {
                            // Tell the decoder that there is no longer a valid device.
                            // The decoder will make a new `cpal_output`.
                            controls
                                .event_handler()
                                .0
                                .send(ThreadMessage::DeviceChanged)
                                .unwrap();
                            ring_buffer_writer.cancel_write();
                        }
                        cpal::StreamError::BackendSpecific { err } => {
                            // This should never happen.
                            panic!("Unknown error occurred during playback: {err}");
                        }
                    }
                }
            },
            None,
        );

        let stream = stream.context("Could not build the stream.")?;
        stream.play()?;

        let sample_rate = config.sample_rate.0;

        Ok(CpalOutput {
            stream,
            ring_buffer_writer: rb_clone.0,
            ring_buffer_reader: rb_clone.1,
            sample_buffer,
            resampler,
            is_stream_done,
            normalizer: Normalizer::new(spec.channels.count(), sample_rate),
            controls,
        })
    }

    fn get_config(spec: SignalSpec) -> anyhow::Result<(Device, StreamConfig)>
    {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .context("Failed to get default output device.")?;

        let config;

        #[cfg(target_os = "windows")]
        {
            let mut supported_configs = device
                .supported_output_configs()
                .context("Failed to get supported output configs.")?;
            config = supported_configs
                .next()
                .context("Failed to get a config.")?
                .with_max_sample_rate()
                .config();
        }

        #[cfg(not(target_os = "windows"))]
        {
            let channels = spec.channels.count();
            config = cpal::StreamConfig {
                channels: channels as cpal::ChannelCount,
                sample_rate: cpal::SampleRate(spec.rate),
                buffer_size: cpal::BufferSize::Default,
            };
        }

        Ok((device, config))
    }

    /// Write the `AudioBufferRef` to the buffers.
    pub fn write(&mut self, decoded: AudioBufferRef)
    {
        if decoded.frames() == 0 {
            return;
        }

        let mut samples = if let Some(resampler) = &mut self.resampler {
            // If there is a resampler, then write resampled values
            // instead of the normal `samples`.
            resampler.resample(decoded).unwrap_or(&[])
        }
        else {
            self.sample_buffer.copy_interleaved_ref(decoded);
            self.sample_buffer.samples()
        };

        if self.controls.is_normalizing() {
            samples = self.normalizer.normalize(samples);
        }

        while let Some(written) = self.ring_buffer_writer.write(samples) {
            samples = &samples[written..];
        }
    }

    /// Clean up after playback is done.
    pub fn flush(&mut self)
    {
        // If there is a resampler, then it may need to be flushed
        // depending on the number of samples it has.
        if let Some(resampler) = &mut self.resampler {
            let mut remaining_samples = resampler.flush().unwrap_or_default();

            while let Some(written) = self.ring_buffer_writer.write(remaining_samples) {
                remaining_samples = &remaining_samples[written..];
            }
        }

        // Wait for all the samples to be read.
        let (mutex, cvar) = &*self.is_stream_done;
        let _lock = cvar.wait(mutex.lock().unwrap());

        self.stream.pause().unwrap();
    }
}

unsafe impl Send for CpalOutput {}
