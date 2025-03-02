// This file is a part of simple_audio
// Copyright (c) 2022-2025 Erikas Taroza <erikastaroza@gmail.com>
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

pub struct MonoToStereo
{
    buffer: Vec<f32>,
}

impl MonoToStereo
{
    pub fn new() -> Self
    {
        MonoToStereo { buffer: Vec::new() }
    }

    pub fn mono_to_stereo(&mut self, input: &[f32]) -> &[f32]
    {
        self.buffer = vec![0.0; input.len() * 2];

        for (i, &sample) in input.iter().enumerate() {
            self.buffer[2 * i] = sample;
            self.buffer[2 * i + 1] = sample;
        }

        &self.buffer
    }
}
