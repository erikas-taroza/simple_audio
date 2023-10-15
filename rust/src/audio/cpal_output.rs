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

use std::sync::{atomic::AtomicBool, Arc};

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
};

/// The default output volume is way too high.
/// Multiplying the volume input by this number
/// will help to reduce it.
const BASE_VOLUME: f32 = 0.8;

pub struct CpalOutput
{
    controls: Controls,
    stream: Stream,
    pub stream_config: StreamConfig,
    pub ring_buffer_reader: BlockingRb<f32, Consumer>,
    pub ring_buffer_writer: BlockingRb<f32, Producer>,
}

impl CpalOutput
{
    pub fn new(controls: Controls) -> anyhow::Result<Self>
    {
        let (device, stream_config, ring_buffer_size) = Self::get_config()?;

        // Create the buffers for the stream.
        let rb = BlockingRb::<f32>::new(ring_buffer_size);
        let rb_clone = rb.clone();
        let ring_buffer_writer = rb.0;
        let ring_buffer_reader = rb.1;

        let stream = device.build_output_stream(
            &stream_config,
            {
                let controls = controls.clone();
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // TODO: Reimplement buffering
                    let buffering = false;

                    // "Pause" the stream.
                    // What this really does is mute the stream.
                    // With only a return statement, the current sample still plays.
                    // CPAL states that `stream.pause()` may not work for all devices.
                    // `stream.pause()` is the ideal way to play/pause.
                    if (cfg!(target_os = "windows")
                        && !controls.is_playing()
                        && !controls.is_stopped())
                        || buffering
                    {
                        data.iter_mut().for_each(|s| *s = 0.0);

                        if buffering {
                            ring_buffer_reader.skip_all();
                        }

                        return;
                    }

                    let written = ring_buffer_reader.read(data);

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
                                .send(PlayerEvent::DeviceChanged)
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

        Ok(Self {
            controls,
            stream,
            stream_config,
            ring_buffer_reader,
            ring_buffer_writer,
        })
    }

    pub fn play(&self)
    {
        self.stream.play();
    }

    pub fn pause(&self)
    {
        self.stream.pause();
    }

    fn get_config() -> anyhow::Result<(Device, StreamConfig, usize)>
    {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .context("Failed to get default output device.")?;
        let default_output_config = device
            .default_output_config()
            .context("Failed to get default output config.")?;

        let sample_rate = default_output_config.sample_rate();
        let channels = default_output_config.channels();

        let default_ring_buf_size = ((200 * sample_rate.0) / 1000) * channels as u32;
        let ring_buffer_size: usize = match default_output_config.buffer_size() {
            cpal::SupportedBufferSize::Range { min, max: _ } => {
                if min <= &default_ring_buf_size {
                    default_ring_buf_size
                }
                else {
                    *min + 10000
                }
            }
            cpal::SupportedBufferSize::Unknown => default_ring_buf_size,
        } as usize;

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
            config = cpal::StreamConfig {
                channels: channels as cpal::ChannelCount,
                sample_rate,
                buffer_size: cpal::BufferSize::Default,
            };
        }

        Ok((device, config, ring_buffer_size))
    }
}

pub struct OutputWriter
{
    controls: Controls,
    pub ring_buffer_writer: BlockingRb<f32, Producer>,
    pub spec: SignalSpec,
    pub duration: u64,
    sample_buffer: SampleBuffer<f32>,
    resampler: Option<Resampler<f32>>,
    normalizer: Normalizer,
}

impl OutputWriter
{
    pub fn new(
        controls: Controls,
        ring_buffer_writer: BlockingRb<f32, Producer>,
        config: StreamConfig,
        spec: SignalSpec,
        duration: u64,
    ) -> Self
    {
        let stream_sample_rate = config.sample_rate.0;

        // Create a resampler if the output stream's sample rate doesn't match the audio's.
        let resampler: Option<Resampler<f32>> = if spec.rate != stream_sample_rate {
            Some(Resampler::new(spec, stream_sample_rate as usize, duration))
        }
        else {
            None
        };

        let normalizer = Normalizer::new(spec.channels.count(), stream_sample_rate);

        let sample_buffer = SampleBuffer::<f32>::new(duration, spec);

        Self {
            controls,
            ring_buffer_writer,
            spec,
            duration,
            normalizer,
            resampler,
            sample_buffer,
        }
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
            if let Some(normalized) = self.normalizer.normalize(samples) {
                samples = normalized;
            }
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
    }
}
