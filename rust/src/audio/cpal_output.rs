use cpal::{Stream, traits::{HostTrait, DeviceTrait, StreamTrait}, Device, StreamConfig};
use rb::*;
use symphonia::core::audio::{SignalSpec, SampleBuffer, AudioBufferRef};

use super::controls::*;

//TODO: Support i16 and u16 instead of only f32.
pub struct CpalOutput
{
    _device:Device,
    _stream:Stream,
    _config:StreamConfig,
    _spec:SignalSpec,
    ring_buffer_writer:Producer<f32>,
    sample_buffer:SampleBuffer<f32>
}

impl CpalOutput
{
    fn get_config(spec:SignalSpec) -> (Device, StreamConfig)
    {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("ERR: Failed to get default output device.");
        // let mut supported_configs = device.supported_output_configs()
        //     .expect("ERR: Failed to get supported outputs.");
        // let config = supported_configs.next()
        //     .expect("ERR: Failed to get supported config.").with_max_sample_rate().config();

        let channels = spec.channels.count();
        let config = cpal::StreamConfig {
            channels: channels as cpal::ChannelCount,
            sample_rate: cpal::SampleRate(spec.rate),
            buffer_size: cpal::BufferSize::Default,
        };

        (device, config)
    }

    /// Starts a new stream on the default device.
    /// Creates a new ring buffer and sample buffer.
    pub fn build_stream(spec:SignalSpec, duration:u64) -> Self
    {
        // Get the output config.
        let (device, config) = Self::get_config(spec);

        // Create a ring buffer with a capacity for up-to 200ms of audio.
        let channels = spec.channels.count();
        let ring_len = ((200 * spec.rate as usize) / 1000) * channels;
        let ring_buffer:SpscRb<f32> = SpscRb::new(ring_len);
        // Create the buffers for the stream.
        let ring_buffer_writer = ring_buffer.producer();
        let sample_buffer = SampleBuffer::<f32>::new(duration, spec);

        let stream = device.build_output_stream(
            &config,
            move |data:&mut [f32], _:&cpal::OutputCallbackInfo| {
                // Pause the stream.
                if !IS_PLAYING.load(std::sync::atomic::Ordering::Relaxed)
                {
                    data.iter_mut().for_each(|s| *s = 0.0);
                    return;
                }

                // This is where data should be modified (like changing volume).
                // This will be the point where there is the lowest latency.
                let written = ring_buffer.consumer().read(data).unwrap_or(0);
                
                // Set the volume.
                data[0..written].iter_mut().for_each(|s| *s = *s * (*VOLUME.read().unwrap()));
            },
            move |err| {
                panic!("ERR: An error occurred during the stream. {err}");
            }
        );

        if let Err(err) = stream
        { panic!("ERR: An error occurred when building the stream. {err}"); }

        let stream = stream.unwrap();
        stream.play().expect("ERR: Failed to play the stream.");

        let stream = stream;

        CpalOutput
        {
            _device: device,
            _config: config,
            _spec: spec,
            _stream: stream,
            ring_buffer_writer,
            sample_buffer
        }
    }

    /// Write the `AudioBufferRef` to the buffers.
    pub fn write(&mut self, decoded:AudioBufferRef)
    {
        if decoded.frames() == 0 { return; }

        // CPAL wants the audio interleaved.
        self.sample_buffer.copy_interleaved_ref(decoded);
        let mut samples = self.sample_buffer.samples();
        
        // Write the interleaved samples to the ring buffer which is output by CPAL.
        while let Some(written) = self.ring_buffer_writer.write_blocking(samples)
        {
            samples = &samples[written..];
        }
    }
}