pub struct Normalizer
{
    buffer:Vec<f32>
}

impl Normalizer
{
    pub fn new(duration:u64) -> Self
    {
        Normalizer { buffer: vec![0.0f32; duration as usize] }
    }

    pub fn normalize(&mut self, input:&[f32]) -> &[f32]
    {
        let normalized = input.to_owned();
        let mut rms = 0.0;
        normalized.iter().for_each(|sample| rms += sample.powi(2));

        let rms = rms / normalized.len() as f32;
        let rms = rms.sqrt();
        let gain = 0.1 / rms;

        self.buffer = input.to_vec();
        self.buffer.iter_mut().for_each(|sample| *sample = *sample * gain);

        // input.iter_mut().for_each(|sample| *sample = *sample * gain);

        &self.buffer
    }
}