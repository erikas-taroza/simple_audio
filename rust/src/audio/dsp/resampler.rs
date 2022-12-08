use rubato::{FftFixedIn, VecResampler};
use symphonia::core::audio::SignalSpec;

pub struct Resampler
{
    resampler:FftFixedIn<f32>,
    output_buffer:Vec<Vec<f32>>
}

impl Resampler
{
    pub fn new(spec:SignalSpec, to_sample_rate:usize, num_frames:usize) -> Self
    {
        let resampler = FftFixedIn::<f32>::new(
            spec.rate as usize,
            to_sample_rate,
            num_frames,
            2,
            spec.channels.count()
        ).unwrap();

        let output_buffer = rubato::Resampler::output_buffer_allocate(&resampler);

        Self
        {
            resampler,
            output_buffer
        }
    }

    /// Resamples a planar/non-interleaved input.
    /// 
    /// Returns the resampled samples in an interleaved format.
    pub fn resample(&mut self, input:&[f32]) -> Vec<f32>
    {
        let num_channels = self.resampler.nbr_channels();

        // The `input` is represented like so: LLLLLLRRRRRR
        // To resample this input, we split the channels (L, R) into 2 vectors.
        // The input now becomes [[LLLLLL], [RRRRRR]].
        // This is what `rubato` needs.
        let mut planar:Vec<Vec<f32>> = vec![];

        let mut offset = 0;
        let step_by = input.len() / num_channels;
        for _ in 0..num_channels
        {
            planar.push(input[offset..offset + step_by].to_vec());
            offset += step_by;
        }

        rubato::Resampler::process_into_buffer(
            &mut self.resampler,
            &planar,
            &mut self.output_buffer,
            None
        ).unwrap();


        // The `interleaved` samples are represented like so: LRLRLRLRLRLR
        let mut interleaved:Vec<f32> = vec![];

        for i in 0..self.output_buffer[0].len()
        {
            for channel in 0..num_channels
            {
                interleaved.push(self.output_buffer[channel][i]);
            }
        }

        interleaved
    }
}