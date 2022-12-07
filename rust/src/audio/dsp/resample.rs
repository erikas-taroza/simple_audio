use symphonia::core::audio::SignalSpec;

pub struct Resampler
{
    spec:SignalSpec,
    to_sample_rate:u32
}

impl Resampler
{
    pub fn new(spec:SignalSpec, to_sample_rate:u32) -> Self
    {
        Self { spec, to_sample_rate }
    }

    pub fn resample(&self, input:&[f32]) -> Vec<f32>
    {
        let resampled = samplerate::convert(
            self.spec.rate,
            self.to_sample_rate,
            self.spec.channels.count(),
            samplerate::ConverterType::Linear,
            input
        ).unwrap();

        resampled
    }
}