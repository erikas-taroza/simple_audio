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
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};

use rangemap::RangeSet;
use reqwest::blocking::Client;
use symphonia::core::io::MediaSource;

const CHUNK_SIZE:usize = 1024 * 64;
const FETCH_OFFSET:usize = CHUNK_SIZE / 2;

pub struct StreamableFile
{
    url:String,
    buffer:Vec<u8>,
    write_position:usize,
    read_position:usize,
    finished_writing:bool,
    downloaded:RangeSet<usize>,
    receiver:Option<Receiver<Vec<u8>>>
}

impl StreamableFile
{
    pub fn new(url:String) -> Self
    {
        // Get the size of the file we are streaming.
        let res = Client::new().head(&url)
            .send()
            .unwrap();

        let header = res
            .headers().get("Content-Length")
            .unwrap();

        let size:usize = header
            .to_str()
            .unwrap()
            .parse()
            .unwrap();

        println!("{size}");

        StreamableFile
        {
            url,
            buffer: vec![0; size],
            write_position: 0,
            read_position: 0,
            finished_writing: false,
            downloaded: RangeSet::new(),
            receiver: None
        }
    }

    /// Gets the next chunk in the sequence.
    /// 
    /// Returns the received bytes by sending them via `tx`.
    fn read_chunk(tx:Sender<Vec<u8>>, url:String, start:usize)
    {
        let end = start + CHUNK_SIZE;

        let chunk = Client::new().get(url)
            .header("Range", format!("bytes={start}-{end}"))
            .send().unwrap().bytes().unwrap().to_vec();
        
        tx.send(chunk).unwrap();
    }

    /// Polls the receiver if it exists.
    /// 
    /// If there is data to receive, then write it to the buffer.
    /// 
    /// Changes made are commited to `downloaded` and 
    /// `write_position` is moved.
    fn try_write_chunk(&mut self)
    {
        if let Some(rx) = &self.receiver
        {
            let start = self.write_position;

            // Block on the first chunk.
            let result = if self.downloaded.is_empty() {
                rx.recv().ok()
            } else { rx.try_recv().ok() };

            match result
            {
                None => (),
                Some(data) => {
                    // Write the data.
                    let end = start + data.len();
                    self.buffer[start..end].copy_from_slice(data.as_slice());
                    self.downloaded.insert(start..end);

                    // Clean up.
                    self.write_position += data.len();
                    self.finished_writing = data.len() < CHUNK_SIZE;
                    self.receiver = None;
                }
            }
        }
    }
}

impl Read for StreamableFile
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>
    {
        println!("{:?}", self.downloaded);

        // This defines the end position of the packet
        // we want to read.
        let read_max = self.read_position + buf.len();
        println!("Read: read_pos[{}] read_max[{read_max}] write_pos[{}]", self.read_position, self.write_position);

        // If the position we are reading at is close to the end of the chunk,
        // then fetch more.
        if !self.finished_writing && read_max >= self.write_position.saturating_sub(FETCH_OFFSET)
            && self.receiver.is_none()
        // TODO: Check if downloaded range.
        {
            let (tx, rx) = channel();
            self.receiver = Some(rx);

            let url = self.url.clone();
            let start = self.write_position;
            thread::spawn(move || {
                Self::read_chunk(tx, url, start);
            });
        }

        // Write any new bytes.
        self.try_write_chunk();

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
            self.buffer.append(&mut vec![0; read_max - self.buffer.len()]);
        }

        // These are the bytes that we want to read.
        let bytes = &self.buffer[self.read_position..read_max];
        buf.copy_from_slice(bytes);

        self.read_position += buf.len();
        Ok(buf.len())
    }
}

impl Seek for StreamableFile
{
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64>
    {
        let seek_position:usize = match pos
        {
            std::io::SeekFrom::Start(pos) => {
                pos as usize
            },
            std::io::SeekFrom::End(pos) => {
                let pos = self.buffer.len() as i64 + pos;

                if pos < 0 {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid seek"));
                }

                pos as usize
            },
            std::io::SeekFrom::Current(pos) => {
                let pos = self.read_position as i64 + pos;

                if pos < 0 {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid seek"));
                }

                pos as usize
            },
        };

        if seek_position > self.buffer.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid seek"));
        }

        println!("Seeking: pos[{seek_position}] type[{pos:?}]");

        self.read_position = seek_position;
        self.write_position = seek_position;

        Ok(seek_position as u64)
    }
}

unsafe impl Send for StreamableFile {}
unsafe impl Sync for StreamableFile {}

impl MediaSource for StreamableFile
{
    fn is_seekable(&self) -> bool {
        super::controls::SEEK_TS.read().unwrap().is_some()
    }

    fn byte_len(&self) -> Option<u64> {
        Some(self.buffer.len() as u64)
    }
}