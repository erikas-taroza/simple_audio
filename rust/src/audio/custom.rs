use cpal::{Stream, traits::{HostTrait, DeviceTrait, StreamTrait}, Device, StreamConfig};
use symphonia::core::audio::SignalSpec;

pub struct CpalOutput
{
    device:Device,
    stream:Option<Stream>,
    config:StreamConfig
}

impl CpalOutput
{
    pub fn new(spec:SignalSpec) -> Self
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

        CpalOutput
        {
            device,
            stream: None,
            config
        }
    }

    pub fn stream(&mut self)
    {
        let stream = self.device.build_output_stream(
            &self.config,
            move |data:&mut [f32], _:&cpal::OutputCallbackInfo| {
                // This is where data should be modified (like changing volume).
                // This will be the point where there is the lowest latency.
            },
            move |err| {
                panic!("ERR: An error occurred during the stream. {err}");
            }
        );

        if let Err(err) = stream
        { panic!("ERR: An error occurred when building the stream. {err}"); }

        let stream = stream.unwrap();
        stream.play().expect("ERR: Failed to play the stream.");

        self.stream = Some(stream);
    }
}