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

use std::sync::{Mutex, Arc, Condvar};

use cpal::{Stream, traits::{HostTrait, DeviceTrait, StreamTrait}, Device, StreamConfig};
use rb::{Producer, Consumer, SpscRb, RB, RbConsumer, RbProducer};
use symphonia::core::audio::{SignalSpec, SampleBuffer, AudioBufferRef};

use super::{controls::*, dsp::{resampler::Resampler, normalizer::Normalizer}, streaming::streamable::IS_STREAM_BUFFERING};

/// The default output volume is way too high.
/// Multiplying the volume input by this number
/// will help to reduce it.
const BASE_VOLUME:f32 = 0.7;

//TODO: Support i16 and u16 instead of only f32.
pub struct CpalOutput
{
    _device:Device,
    _config:StreamConfig,
    pub stream:Stream,
    pub ring_buffer_reader:Consumer<f32>,
    ring_buffer_writer:Producer<f32>,
    sample_buffer:SampleBuffer<f32>,
    resampler:Option<Resampler<f32>>,
    is_stream_done:Arc<(Mutex<bool>, Condvar)>
}

impl CpalOutput
{
    fn get_config(spec:SignalSpec) -> (Device, StreamConfig)
    {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("ERR: Failed to get default output device.");

        let config;

        #[cfg(target_os = "windows")]
        {
            let mut supported_configs = device.supported_output_configs()
                .expect("ERR: Failed to get supported outputs.");
            config = supported_configs.next()
                .expect("ERR: Failed to get supported config.").with_max_sample_rate().config();
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

        (device, config)
    }

    /// Starts a new stream on the default device.
    /// Creates a new ring buffer and sample buffer.
    pub fn build_stream(spec:SignalSpec, duration:u64) -> Self
    {
        // Get the output config.
        let (device, config) = Self::get_config(spec);

        // Create a resampler only if the code is running on Windows
        // and if the output config's sample rate doesn't match the audio's.
        let resampler:Option<Resampler<f32>> = if cfg!(target_os = "windows") &&
            spec.rate != config.sample_rate.0 { Some(Resampler::new(spec, config.sample_rate.0 as usize, duration)) }
            else { None };

        // Create a ring buffer with a capacity for up-to 200ms of audio.
        let channels = spec.channels.count();
        let ring_len = ((200 * spec.rate as usize) / 1000) * channels;
        let ring_buffer:SpscRb<f32> = SpscRb::new(ring_len);
        // Create the buffers for the stream.
        let ring_buffer_reader = ring_buffer.consumer();
        let ring_buffer_writer = ring_buffer.producer();
        let sample_buffer = SampleBuffer::<f32>::new(duration, spec);

        let is_stream_done = Arc::new((Mutex::new(false), Condvar::new()));
        let is_stream_done_clone = is_stream_done.clone();

        let stream = device.build_output_stream(
            &config,
            move |data:&mut [f32], _:&cpal::OutputCallbackInfo| {
                let buffering = IS_STREAM_BUFFERING.load(std::sync::atomic::Ordering::SeqCst);

                // "Pause" the stream.
                // What this really does is mute the stream.
                // With only a return statement, the current sample still plays.
                // CPAL states that `stream.pause()` may not work for all devices.
                // `stream.pause()` is the ideal way to play/pause.
                if (!IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst)
                    && cfg!(target_os = "windows")) || buffering
                {
                    data.iter_mut().for_each(|s| *s = 0.0);
                    
                    if buffering {
                        let _ = ring_buffer_reader.skip_pending();
                    }

                    return;
                }

                // This is where data should be modified (like changing volume).
                // This will be the point where there is the lowest latency.
                let written = ring_buffer_reader.read(data).unwrap_or(0);

                // Notify that the stream was finished reading.
                if written == 0 {
                    let (mutex, cvar) = &*is_stream_done_clone;
                    *mutex.lock().unwrap() = true;
                    cvar.notify_one();
                    return;
                }
                
                // Set the volume.
                data[0..written].iter_mut().for_each(|s| *s *= BASE_VOLUME * *VOLUME.read().unwrap());
            },
            move |err| {
                panic!("ERR: An error occurred during the stream. {err}");
            }, None
        );

        if let Err(err) = stream
        { panic!("ERR: An error occurred when building the stream. {err}"); }

        let stream = stream.unwrap();
        stream.play().expect("ERR: Failed to play the stream.");

        CpalOutput
        {
            _device: device,
            _config: config,
            stream,
            ring_buffer_reader: ring_buffer.consumer(),
            ring_buffer_writer,
            sample_buffer,
            resampler,
            is_stream_done
        }
    }

    /// Write the `AudioBufferRef` to the buffers.
    pub fn write(&mut self, decoded:AudioBufferRef)
    {
        if decoded.frames() == 0 { return; }

        let mut samples = if let Some(resampler) = &mut self.resampler {
            // If there is a resampler, then write resampled values
            // instead of the normal `samples`.
            resampler.resample(decoded).unwrap_or(&[])
        } else {
            self.sample_buffer.copy_interleaved_ref(decoded);
            self.sample_buffer.samples()
        };

        // let normalized = Normalizer::normalize(samples);
        // samples = normalized.as_slice();

        // Write the interleaved samples to the ring buffer which is output by CPAL.
        while let Some(written) = self.ring_buffer_writer.write_blocking(samples)
        { samples = &samples[written..]; }
    }

    /// Clean up after playback is done.
    pub fn flush(&mut self)
    {
        // If there is a resampler, then it may need to be flushed
        // depending on the number of samples it has.
        if let Some(resampler) = &mut self.resampler {
            let mut remaining_samples = resampler.flush().unwrap_or_default();

            while let Some(written) = self.ring_buffer_writer.write_blocking(remaining_samples) {
                remaining_samples = &remaining_samples[written..];
            }
        }

        // Wait for all the samples to be read.
        let (mutex, cvar) = &*self.is_stream_done;
        let _lock = cvar.wait(mutex.lock().unwrap());

        self.stream.pause().unwrap();
    }
}