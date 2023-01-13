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

const NORMALIZE_TO:f32 = 1.0;

pub struct Normalizer
{
    buffer:Vec<f32>
}

impl Normalizer
{
    pub fn new() -> Self
    {
        Normalizer { buffer: Vec::new() }
    }

    pub fn normalize(&mut self, input:&[f32]) -> &[f32]
    {
        self.buffer = input.to_vec();

        let mut peaks:Vec<f32> = Vec::new();
        for window in self.buffer.chunks_exact(input.len() / 10)
        {
            let peak = Self::calc_peak(window);
            peaks.push(peak.abs());
        }

        let rms = Self::calc_rms(&peaks);
        let gain = NORMALIZE_TO / rms;

        self.buffer = input.iter().map(|sample| sample * gain).collect();

        &self.buffer
    }

    fn calc_rms(input:&[f32]) -> f32
    {
        let mut sum = 0.0;
        input.iter().for_each(|sample| sum += sample.powi(2));
        (sum / input.len() as f32).sqrt()
    }

    fn calc_peak(input:&[f32]) -> f32
    {
        input.to_vec().into_iter()
            .map(|sample| sample.abs())
            .reduce(f32::max).unwrap_or(1.0)
    }
}