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

use std::{
    io::{Read, Seek},
    sync::mpsc::Sender,
};

use symphonia::core::io::MediaSource;

#[cfg(any(feature = "hls_streaming", feature = "http_streaming"))]
pub const CHUNK_SIZE: usize = 1024 * 256;

pub trait Streamable: Read + Seek + Send + Sync + MediaSource
{
    fn read_chunk(
        tx: Sender<(usize, Vec<u8>)>,
        url: String,
        start: usize,
        file_size: usize,
    ) -> anyhow::Result<()>;

    fn try_write_chunk(&mut self, should_buffer: bool);
    fn should_get_chunk(&self) -> (bool, usize);
}
