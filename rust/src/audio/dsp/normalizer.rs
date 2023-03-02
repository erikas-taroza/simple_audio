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

use ebur128::*;

/// The target LUFS value.
const NORMALIZE_TO: f64 = -15.0;

pub struct Normalizer
{
    ebur128: EbuR128,
    buffer: Vec<f32>
}

impl Normalizer
{
    pub fn new(channels: usize, sample_rate: u32) -> Self
    {
        let ebur128 = EbuR128::new(
            channels as u32,
            sample_rate,
            Mode::I.union(Mode::M)
        ).unwrap();

        Normalizer { ebur128, buffer: Vec::new() }
    }

    pub fn normalize(&mut self, input: &[f32]) -> &[f32]
    {
        // Completely quiet inputs cause a crackling sound to be made.
        if !input.iter().any(|x| *x != 0.0) { return &[];}

        let _ = self.ebur128.add_frames_f32(input);

        let global_loudness = self.ebur128.loudness_global().unwrap();

        // There may not be enough data to calculate a global loudness value.
        let loudness = if global_loudness.is_finite() {
            global_loudness
        }
        else {
            self.ebur128.loudness_momentary().unwrap()
        };
        
        let gain = (loudness / NORMALIZE_TO).clamp(0.35, 1.0) as f32;

        self.buffer.clear();
        self.buffer.extend_from_slice(input);

        self.buffer.iter_mut().for_each(|sample| *sample *= gain);
        &self.buffer
    }
}