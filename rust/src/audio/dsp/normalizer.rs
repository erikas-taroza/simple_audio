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

const NORMALIZE_TO:f32 = 0.1;

#[derive(Default)]
pub struct Normalizer
{
    rms:Vec<f32>
}

impl Normalizer
{
    pub fn normalize(&mut self, input:&[f32]) -> Vec<f32>
    {
        let rms = Self::calc_rms(&input);
        self.rms.push(rms);
        let total:f32 = self.rms.iter().fold(0f32, |acc, x| acc + x);
        let average_rms = total / self.rms.len() as f32;

        let gain = NORMALIZE_TO / average_rms;

        let mut input = input.to_vec();

        input.iter_mut().for_each(|sample| *sample *= gain);
        input
    }

    fn calc_rms(input:&[f32]) -> f32
    {
        let mut sum = 0.0;
        input.iter().for_each(|sample| sum += sample.powi(2));
        (sum / input.len() as f32).sqrt()
    }
}