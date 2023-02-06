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

use std::{io::{Read, Seek}, sync::{atomic::AtomicBool, mpsc::Sender}};

use symphonia::core::io::MediaSource;

// Used in cpal_output.rs to mute the stream when buffering.
pub static IS_STREAM_BUFFERING:AtomicBool = AtomicBool::new(false);
pub const CHUNK_SIZE:usize = 1024 * 128;

pub trait Streamable<T: Read + Seek + Send + Sync + MediaSource>
{
    fn read_chunk(tx:Sender<(usize, Vec<u8>)>, url:String, start:usize, file_size:usize);
    fn try_write_chunk(&mut self, should_buffer:bool);
    fn should_get_chunk(&self) -> (bool, usize);
}