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

use cpal::StreamConfig;
use symphonia_core::audio::{AudioBufferRef, SampleBuffer, SignalSpec};

use crate::{
    audio::{
        controls::Controls,
        dsp::{mono_to_stereo::MonoToStereo, normalizer::Normalizer, resampler::Resampler},
    },
    utils::blocking_rb::{BlockingRb, Producer},
};

pub struct OutputWriter
{
    controls: Controls,
    pub ring_buffer_writer: BlockingRb<f32, Producer>,
    sample_buffer: SampleBuffer<f32>,
    resampler: Option<Resampler<f32>>,
    normalizer: Normalizer,
    mono_to_stereo: Option<MonoToStereo>,
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

        let mono_to_stereo = if spec.channels.count() as u16 != config.channels {
            Some(MonoToStereo::new())
        }
        else {
            None
        };

        let normalizer = Normalizer::new(spec.channels.count(), stream_sample_rate);

        let sample_buffer = SampleBuffer::<f32>::new(duration, spec);

        Self {
            controls,
            ring_buffer_writer,
            normalizer,
            resampler,
            mono_to_stereo,
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

        if let Some(mono_to_stereo) = &mut self.mono_to_stereo {
            samples = mono_to_stereo.mono_to_stereo(samples);
        }

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
    pub fn flush(&mut self, wait_for_samples: bool)
    {
        // If there is a resampler, then it may need to be flushed
        // depending on the number of samples it has.
        if let Some(resampler) = &mut self.resampler {
            let mut remaining_samples = resampler.flush().unwrap_or_default();

            while let Some(written) = self.ring_buffer_writer.write(remaining_samples) {
                remaining_samples = &remaining_samples[written..];
            }
        }

        // Wait for the remaining samples to be written.
        // Not needed when preloading since the samples will be played anyways.
        if wait_for_samples {
            while !self.ring_buffer_writer.is_empty() {
                continue;
            }
        }
    }
}
