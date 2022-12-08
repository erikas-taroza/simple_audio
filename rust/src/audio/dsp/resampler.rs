use rubato::FftFixedIn;
use symphonia::core::audio::SignalSpec;

pub struct Resampler
{
    resampler:FftFixedIn<f32>,
    output_buffer:Vec<Vec<f32>>,
    num_frames:usize,
    num_channels:usize
}

impl Resampler
{
    pub fn new(spec:SignalSpec, to_sample_rate:usize, num_frames:usize) -> Self
    {
        let num_channels = spec.channels.count();

        let resampler = FftFixedIn::<f32>::new(
            spec.rate as usize,
            to_sample_rate,
            num_frames,
            2,
            num_channels
        ).unwrap();

        let output_buffer = rubato::Resampler::output_buffer_allocate(&resampler);

        Self
        {
            resampler,
            output_buffer,
            num_frames,
            num_channels
        }
    }

    /// Resamples a planar/non-interleaved input.
    /// 
    /// Returns the resampled samples in an interleaved format.
    pub fn resample(&mut self, input:&[f32]) -> Vec<f32>
    {
        // The `input` is represented like so: LLLLLLRRRRRR
        // To resample this input, we split the channels (L, R) into 2 vectors.
        // The input now becomes [[LLLLLL], [RRRRRR]].
        // This is what `rubato` needs.
        let mut planar:Vec<Vec<f32>> = vec![];

        let mut offset = 0;
        for _ in 0..self.num_channels
        {
            planar.push(input[offset..offset + self.num_frames].to_vec());
            offset += self.num_frames;
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
            for channel in 0..self.num_channels
            {
                interleaved.push(self.output_buffer[channel][i]);
            }
        }

        interleaved
    }
}