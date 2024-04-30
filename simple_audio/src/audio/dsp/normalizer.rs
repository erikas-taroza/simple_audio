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
const NORMALIZE_TO: f64 = -14.0;
const LOWER_THRESHOLD: f32 = 0.2;

pub struct Normalizer
{
    ebur128: EbuR128,
    buffer: Vec<f32>,
    /// True if the input samples are loud enough to start being normalized.
    /// This prevents normalizing parts of a song that the artist intented to be quiet.
    passed_lower_threshold: bool,
}

impl Normalizer
{
    pub fn new(channels: usize, sample_rate: u32) -> Self
    {
        let ebur128 = EbuR128::new(channels as u32, sample_rate, Mode::I.union(Mode::M)).unwrap();

        Normalizer {
            ebur128,
            buffer: Vec::new(),
            passed_lower_threshold: false,
        }
    }

    pub fn normalize(&mut self, input: &[f32]) -> Option<&[f32]>
    {
        // Completely quiet inputs cause a crackling sound to be made.
        if !input.iter().any(|x| *x != 0.0) {
            return None;
        }

        // Don't apply any gain when threshold is not passed.
        if !self.passed_lower_threshold {
            let samples_passing_threshold = input[0..3].iter().find(|e| **e >= LOWER_THRESHOLD);
            self.passed_lower_threshold = samples_passing_threshold.is_some();
            return None;
        }

        let _ = self.ebur128.add_frames_f32(input);

        let global_loudness = self.ebur128.loudness_global().unwrap();
        let gain = if global_loudness.is_finite() {
            calc_gain(global_loudness)
        }
        else {
            let loudness = self.ebur128.loudness_momentary().unwrap();
            calc_gain(loudness)
        };

        let gain = gain.clamp(0.0, 1.2);

        self.buffer.clear();
        self.buffer.extend_from_slice(input);

        self.buffer.iter_mut().for_each(|sample| *sample *= gain);
        Some(&self.buffer)
    }
}

fn calc_gain(loudness: f64) -> f32
{
    let gain_db = NORMALIZE_TO - loudness;
    10.0_f32.powf(gain_db as f32 / 20.0)
}

#[cfg(test)]
mod tests
{
    use crossbeam::channel::unbounded;

    #[test]
    fn normalize() -> anyhow::Result<()>
    {
        let thread_killer = unbounded();
        let player = crate::Player::new(thread_killer.1);
        player.set_volume(0.5);
        player.normalize_volume(true);
        // Try out different files here.
        player.open(
            "https://dl.espressif.com/dl/audio/ff-16b-2c-44100hz.mp3".to_string(),
            true,
        )?;
        std::thread::sleep(std::time::Duration::from_secs(100));
        Ok(())
    }
}
