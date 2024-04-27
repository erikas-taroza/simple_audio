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

use std::io::{Read, Seek};
use std::ops::Range;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;

use anyhow::Context;
use rangemap::RangeSet;
use reqwest::blocking::Client;
use symphonia::core::io::MediaSource;

use crate::error::Error;
use crate::PlayerEvent;

use super::{streamable::*, Receiver};

// NOTE: Most of the implementation is the same as HttpStream.
// Most comments can be found in HttpStream. Only the HLS specific
// comments are included.

pub struct HlsStream
{
    /// A list of parts with their size and URL.
    urls: Vec<(Range<usize>, String)>,
    buffer: Vec<u8>,
    read_position: usize,
    downloaded: RangeSet<usize>,
    requested: RangeSet<usize>,
    receivers: Vec<Receiver>,
    buffer_signal: Arc<AtomicBool>,
    error_sender: crossbeam::channel::Sender<PlayerEvent>,
}

impl HlsStream
{
    pub fn new(
        url: String,
        buffer_signal: Arc<AtomicBool>,
        error_sender: crossbeam::channel::Sender<PlayerEvent>,
    ) -> anyhow::Result<Self>
    {
        let mut urls = Vec::new();
        let mut total_size = 0;

        let mut buffer = Vec::new();
        let mut downloaded = RangeSet::new();
        let mut requested = RangeSet::new();

        let file = Client::new().get(url).send()?.text()?;
        for line in file.lines() {
            if !line.contains("http") {
                continue;
            }

            // Get the size of the part.
            let res = Client::new().head(line).send()?.error_for_status()?;
            let header = res.headers().get("Content-Length");

            match header {
                Some(content_length) => {
                    let size = content_length.to_str()?.parse()?;
                    urls.push((total_size..total_size + size + 1, line.to_string()));
                    total_size += size;
                    buffer.extend_from_slice(&vec![0; size]);
                }
                None => {
                    // Content-Length request failed to get file size. Download the whole part.
                    let response = Client::new()
                        .get(line)
                        .header("Range", "bytes=0-")
                        .send()
                        .context("Could not download part for playback.")?;

                    let mut bytes = response.bytes()?.to_vec();
                    let size = bytes.len();
                    buffer.append(&mut bytes);
                    downloaded.insert(total_size..total_size + size);
                    requested.insert(total_size..total_size + size);
                    total_size += size;
                }
            }
        }

        Ok(HlsStream {
            urls,
            buffer,
            read_position: 0,
            downloaded,
            requested,
            receivers: Vec::new(),
            buffer_signal,
            error_sender,
        })
    }
}

impl Streamable for HlsStream
{
    fn read_chunk(
        tx: Sender<(usize, Vec<u8>)>,
        url: String,
        start: usize,
        _: usize,
    ) -> anyhow::Result<()>
    {
        let chunk = Client::new()
            .get(url)
            .send()?
            .error_for_status()?
            .bytes()?
            .to_vec();

        // We don't care if the data was sent or not.
        let _ = tx.send((start, chunk));
        Ok(())
    }

    fn try_write_chunk(&mut self, should_buffer: bool)
    {
        let mut completed_downloads = Vec::new();

        for Receiver { id, receiver } in &self.receivers {
            let result = if self.downloaded.is_empty() || should_buffer {
                receiver.recv().ok()
            }
            else {
                receiver.try_recv().ok()
            };

            match result {
                None => (),
                Some((position, chunk)) => {
                    let end = position + chunk.len();

                    if position != end {
                        self.buffer[position..end].copy_from_slice(chunk.as_slice());
                        self.downloaded.insert(position..end);
                    }

                    completed_downloads.push(*id);
                }
            }
        }

        self.receivers
            .retain(|receiver| !completed_downloads.contains(&receiver.id));
    }

    fn should_get_chunk(&self) -> (bool, usize)
    {
        let closest_range = self.downloaded.get(&self.read_position);

        if closest_range.is_none() {
            return (true, self.read_position);
        }

        let closest_range = closest_range.unwrap();
        let is_already_downloading = self.requested.contains(&(self.read_position + CHUNK_SIZE));
        let prefetch_pos = self.read_position + (CHUNK_SIZE * 2);

        let should_get_chunk = prefetch_pos >= closest_range.end
            && !is_already_downloading
            && closest_range.end != self.buffer.len();

        (should_get_chunk, closest_range.end)
    }
}

impl Read for HlsStream
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>
    {
        if self.read_position >= self.buffer.len() {
            return Ok(0);
        }

        let read_max = (self.read_position + buf.len()).min(self.buffer.len());
        let (should_get_chunk, chunk_write_pos) = self.should_get_chunk();

        // println!("Read: read_pos[{}] read_max[{read_max}] buf[{}] write_pos[{chunk_write_pos}] download[{should_get_chunk}]", self.read_position, buf.len());
        if should_get_chunk {
            // Find the correct part by checking if its range contains the
            // `chunk_write_pos`.
            let part = self
                .urls
                .iter()
                .find(|(range, _)| range.contains(&chunk_write_pos));

            if let Some((range, url)) = part {
                self.requested.insert(range.clone());

                let url = url.clone();
                let start = range.start;
                let file_size = self.buffer.len();
                let (tx, receiver) = channel();

                let id = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                self.receivers.push(Receiver { id, receiver });

                let error_sender = self.error_sender.clone();
                thread::spawn(move || {
                    let result = Self::read_chunk(tx, url, start, file_size);

                    if let Err(err) = result {
                        let _ = error_sender
                            .send(PlayerEvent::Error(Error::NetworkStream(err.to_string())));
                    }
                });

                // Because the sizes of the parts vary, `should_get_chunk` may
                // still return true (it checks based on `CHUNK_SIZE`).
                // This would cause an issue where the beginning of the audio
                // will be written again, but a little later causing a replay.
                // To prevent this, remove the downloaded
                // part so that the `find()` call above returns None.
                let index = self.urls.iter().position(|x| x == part.unwrap()).unwrap();
                self.urls.remove(index);
            }
        }

        let should_buffer = !self.downloaded.contains(&self.read_position);
        // If this source is active, then allow buffering in `cpal_output`.
        self.buffer_signal
            .store(should_buffer, std::sync::atomic::Ordering::SeqCst);
        self.try_write_chunk(should_buffer);

        let bytes = &self.buffer[self.read_position..read_max];
        buf[0..bytes.len()].copy_from_slice(bytes);

        self.read_position += bytes.len();
        Ok(bytes.len())
    }
}

impl Seek for HlsStream
{
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64>
    {
        let seek_position: usize = match pos {
            std::io::SeekFrom::Start(pos) => pos as usize,
            std::io::SeekFrom::Current(pos) => {
                let pos = self.read_position as i64 + pos;
                pos.try_into().map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid seek: {pos}"),
                    )
                })?
            }
            std::io::SeekFrom::End(pos) => {
                let pos = self.buffer.len() as i64 + pos;
                pos.try_into().map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid seek: {pos}"),
                    )
                })?
            }
        };

        if seek_position > self.buffer.len() {
            return Ok(self.read_position as u64);
        }

        // println!("Seeking: pos[{seek_position}] type[{pos:?}]");

        self.read_position = seek_position;

        Ok(seek_position as u64)
    }
}

unsafe impl Send for HlsStream {}
unsafe impl Sync for HlsStream {}

impl MediaSource for HlsStream
{
    fn is_seekable(&self) -> bool
    {
        true
    }

    fn byte_len(&self) -> Option<u64>
    {
        Some(self.buffer.len() as u64)
    }
}
