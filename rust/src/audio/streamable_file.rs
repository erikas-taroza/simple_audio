// This file is a part of simple_audio
// Copyright (c) 2022 Erikas Taroza <erikastaroza@gmail.com>
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

use std::io::{Read, Seek};

use reqwest::blocking::Client;
use symphonia::core::io::MediaSource;

const CHUNK_SIZE:usize = 1024 * 64;
const FETCH_OFFSET:usize = 10000;

pub struct StreamableFile
{
    buffer:Vec<Option<u8>>,
    write_position:usize,
    read_position:usize,
    seek_position:Option<usize>,
    finished_writing:bool,
    url:String
}

impl StreamableFile
{
    pub fn new(url:&str) -> Self
    {
        StreamableFile
        {
            buffer: Vec::new(),
            write_position: 0,
            read_position: 0,
            seek_position: None,
            finished_writing: false,
            url: url.to_string()
        }
    }

    /// Gets the next chunk in the sequence.
    fn get_chunk(&mut self)
    {
        // Get the chunk for the seek.
        if self.seek_position.is_some()
        {
            let seek_position = self.seek_position.unwrap();
            self.seek_position = None;
            println!("Seeking at: {}", seek_position);

            // If the seek is after the buffer, then fill previous
            // values with None.
            if seek_position > self.buffer.len() {
                println!("Filling buffer after seeking");
                self.buffer.append(&mut vec![None; seek_position - self.buffer.len()]);
            }
            // If the seek is in the buffer, then make sure
            // there is a chunk available.
            else if seek_position < self.buffer.len() {
                let forward_slice = &self.buffer[seek_position..seek_position + CHUNK_SIZE];
                if !forward_slice.contains(&None) {
                    println!("Buffer doesn't contain None values");
                    return;
                }
            }

            self.write_position = seek_position;
        }

        // Get the next chunk.
        let start = self.write_position;
        let end = start + CHUNK_SIZE;
        let chunk = Client::new().get(self.url.clone())
            .header("Range", format!("bytes={start}-{end}"))
            .send().unwrap().bytes().unwrap().to_vec();
        
        let num_received = chunk.len();

        println!("Received chunk ({}): {}", self.buffer.len() / CHUNK_SIZE, num_received);

        // Add chunk to buffer.
        let mut chunk:Vec<Option<u8>> = chunk.iter().map(|b| Some(*b))
            .collect();
        self.buffer.append(&mut chunk);

        // Move write_position by how much data was received.
        self.write_position += num_received;

        // We have finished filling the buffer if we do not receive
        // any more data.
        if num_received == 0 {
            self.finished_writing = true;
        }
    }
}

impl Read for StreamableFile
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>
    {
        // This defines the end position of the packet
        // we want to read.
        let read_max = self.read_position + buf.len();

        // If we haven't received all the chunks,
        // then get more.
        if self.buffer.is_empty() || self.seek_position.is_some() ||
            // If the position we are reading at is close to the end of the chunk,
            // then fetch more.
            (!self.finished_writing && read_max >= self.buffer.len().saturating_sub(FETCH_OFFSET))
        {
            self.get_chunk();
        }

        // If we are reading after the buffer has been filled,
        // then throw an error to signal that we have reached the end.
        if self.read_position >= self.buffer.len() && self.finished_writing {
            println!("Finished reading!");
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "End of file."));
        }

        // If the buffer doesn't align with what is being requested,
        // fill the buffer with 0s.
        // This happens at the end of the file.
        if self.buffer.len() < read_max {
            self.buffer.append(&mut vec![Some(0); read_max - self.buffer.len()]);
        }

        // These are the bytes that we want to read.
        let bytes:Vec<u8> = self.buffer[self.read_position..read_max]
            .iter().take(buf.len())
            .map(|b| b.unwrap())
            .collect();

        self.read_position += buf.len();
        buf.copy_from_slice(bytes.as_slice());
        Ok(buf.len())
    }
}

impl Seek for StreamableFile
{
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64>
    {
        match pos
        {
            std::io::SeekFrom::Start(pos) => {
                self.seek_position = Some(pos as usize);
                self.read_position = pos as usize;
            },
            std::io::SeekFrom::End(_) => todo!(),
            std::io::SeekFrom::Current(_) => todo!(),
        }

        Ok(self.read_position as u64)
    }
}

unsafe impl Send for StreamableFile {}
unsafe impl Sync for StreamableFile {}

impl MediaSource for StreamableFile
{
    fn is_seekable(&self) -> bool {
        false
    }

    fn byte_len(&self) -> Option<u64> {
        None
    }
}