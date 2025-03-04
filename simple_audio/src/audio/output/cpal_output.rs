// This file is a part of simple_audio
// Copyright (c) 2022-2025 Erikas Taroza <erikastaroza@gmail.com>
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

use anyhow::Context;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Stream, StreamConfig,
};

use crate::{
    audio::controls::{Controls, DecoderEvent},
    utils::blocking_rb::*,
};

/// The default output volume is way too high.
const BASE_VOLUME: f32 = 0.8;

pub struct CpalOutput
{
    stream: Stream,
    /// A safety to prevent playing/pausing the stream when it is already in that state.
    /// If this happens, the Android implementation won't work.
    is_playing: bool,
    pub needs_flush: bool,
    controls: Controls,
    device: Device,
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
        let ring_buffer_writer = rb.0;
        let ring_buffer_reader = rb.1;
        let stream = Self::build_stream(
            &device,
            &stream_config,
            &controls,
            &ring_buffer_writer,
            &ring_buffer_reader,
        )?;

        Ok(Self {
            stream,
            is_playing: true,
            needs_flush: true,
            controls,
            device,
            stream_config,
            ring_buffer_reader,
            ring_buffer_writer,
        })
    }

    fn build_stream(
        device: &Device,
        stream_config: &StreamConfig,
        controls: &Controls,
        ring_buffer_writer: &BlockingRb<f32, Producer>,
        ring_buffer_reader: &BlockingRb<f32, Consumer>,
    ) -> anyhow::Result<Stream>
    {
        let stream = device.build_output_stream(
            &stream_config,
            {
                let controls = controls.clone();
                let ring_buffer_reader = ring_buffer_reader.clone();
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    if let Some(written) = ring_buffer_reader.read(data) {
                        data[0..written]
                            .iter_mut()
                            // Set the volume.
                            .for_each(|s| *s *= BASE_VOLUME * *controls.volume());
                    }
                    else {
                        data.fill(0.0);
                    }
                }
            },
            {
                let controls = controls.clone();
                let ring_buffer_writer = ring_buffer_writer.clone();
                move |err| {
                    match err {
                        cpal::StreamError::DeviceNotAvailable => {
                            // Tell the decoder that there is no longer a valid device.
                            // The decoder will make a new `cpal_output`.
                            controls
                                .decoder_event_handler()
                                .0
                                .send(DecoderEvent::DeviceChanged)
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
        let _ = stream.play();
        Ok(stream)
    }

    /// Prepares `CpalOutput` for playback again. Not needed when playing a preloaded file.
    pub fn flush(&mut self) -> anyhow::Result<()>
    {
        if !self.needs_flush {
            return Ok(());
        }

        self.ring_buffer_reader.skip_all();
        self.stream.pause()?;
        self.stream = Self::build_stream(
            &self.device,
            &self.stream_config,
            &self.controls,
            &self.ring_buffer_writer,
            &self.ring_buffer_reader,
        )?;

        self.needs_flush = false;

        Ok(())
    }

    pub fn play(&mut self)
    {
        if self.is_playing {
            return;
        }

        _ = self.stream.play();
        self.is_playing = true;
    }

    pub fn pause(&mut self)
    {
        if !self.is_playing {
            return;
        }

        _ = self.stream.pause();
        self.is_playing = false;
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
