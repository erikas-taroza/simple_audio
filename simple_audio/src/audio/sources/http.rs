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

pub struct HttpStream
{
    url: String,
    buffer: Vec<u8>,
    read_position: usize,
    downloaded: RangeSet<usize>,
    requested: RangeSet<usize>,
    receivers: Vec<Receiver>,
    buffer_signal: Arc<AtomicBool>,
    error_sender: crossbeam::channel::Sender<PlayerEvent>,
}

impl HttpStream
{
    pub fn new(
        url: String,
        buffer_signal: Arc<AtomicBool>,
        error_sender: crossbeam::channel::Sender<PlayerEvent>,
    ) -> anyhow::Result<Self>
    {
        // Get the size of the file we are streaming.
        let res = Client::new().head(&url).send()?.error_for_status()?;
        let header = res.headers().get("Content-Length");

        let buffer;
        let mut downloaded = RangeSet::new();
        let mut requested = RangeSet::new();

        match header {
            Some(content_length) => {
                buffer = vec![0; content_length.to_str()?.parse()?];
            }
            None => {
                // Content-Length request failed to get file size. Download the whole file.
                let response = Client::new()
                    .get(&url)
                    .header("Range", "bytes=0-")
                    .send()
                    .context("Could not download file for playback.")?;

                let bytes = response.bytes()?.to_vec();
                buffer = bytes;
                downloaded.insert(0..buffer.len());
                requested.insert(0..buffer.len());
            }
        }

        Ok(HttpStream {
            url,
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

impl Streamable for HttpStream
{
    /// Gets the next chunk in the sequence.
    ///
    /// Returns the received bytes by sending them via `tx`.
    fn read_chunk(
        tx: Sender<(usize, Vec<u8>)>,
        url: String,
        start: usize,
        file_size: usize,
    ) -> anyhow::Result<()>
    {
        let end = (start + CHUNK_SIZE).min(file_size) - 1;

        let chunk = Client::new()
            .get(url)
            .header("Range", format!("bytes={start}-{end}"))
            .send()?
            .error_for_status()?
            .bytes()?
            .to_vec();

        // We don't care if the data was sent or not.
        let _ = tx.send((start, chunk));
        Ok(())
    }

    /// Polls all receivers.
    ///
    /// If there is data to receive, then write it to the buffer.
    ///
    /// Changes made are commited to `downloaded`.
    fn try_write_chunk(&mut self, should_buffer: bool)
    {
        let mut completed_downloads = Vec::new();

        for Receiver { id, receiver } in &self.receivers {
            // Block on the first chunk or when buffering.
            // Buffering fixes the issue with seeking on MP3 (no blocking on data).
            let result = if self.downloaded.is_empty() || should_buffer {
                receiver.recv().ok()
            }
            else {
                receiver.try_recv().ok()
            };

            match result {
                None => (),
                Some((position, chunk)) => {
                    // Write the data.
                    let end = (position + chunk.len()).min(self.buffer.len());

                    if position != end {
                        self.buffer[position..end].copy_from_slice(chunk.as_slice());
                        self.downloaded.insert(position..end);
                    }

                    // Clean up.
                    completed_downloads.push(*id);
                }
            }
        }

        // Remove completed receivers.
        self.receivers
            .retain(|receiver| !completed_downloads.contains(&receiver.id));
    }

    /// Determines if a chunk should be downloaded by getting
    /// the downloaded range that contains `self.read_position`.
    ///
    /// Returns `true` and the start index of the chunk
    /// if one should be downloaded.
    fn should_get_chunk(&self) -> (bool, usize)
    {
        let closest_range = self.downloaded.get(&self.read_position);

        if closest_range.is_none() {
            return (true, self.read_position);
        }

        let closest_range = closest_range.unwrap();

        // Make sure that the same chunk isn't being downloaded again.
        // This may happen because the next `read` call happens
        // before the chunk has finished downloading. In that case,
        // it is unnecessary to request another chunk.
        let is_already_downloading = self.requested.contains(&(self.read_position + CHUNK_SIZE));

        // Basically, if the condition below is true,
        // then a chunk needs to be downloaded to ensure
        // that there are at least 2 chunks ahead of the read_position.
        // This reduces buffering in the FLAC and OGG formats.
        let prefetch_pos = self.read_position + (CHUNK_SIZE * 2);

        let should_get_chunk = prefetch_pos >= closest_range.end
            && !is_already_downloading
            && closest_range.end != self.buffer.len();

        (should_get_chunk, closest_range.end)
    }
}

impl Read for HttpStream
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>
    {
        // If we are reading after the buffer,
        // then return early with 0 written bytes.
        if self.read_position >= self.buffer.len() {
            return Ok(0);
        }

        // This defines the end position of the packet
        // we want to read.
        let read_max = (self.read_position + buf.len()).min(self.buffer.len());

        // If the position we are reading at is close
        // to the last downloaded chunk, then fetch more.
        let (should_get_chunk, chunk_write_pos) = self.should_get_chunk();

        // println!("Read: read_pos[{}] read_max[{read_max}] buf[{}] write_pos[{chunk_write_pos}] download[{should_get_chunk}]", self.read_position, buf.len());
        if should_get_chunk {
            self.requested
                .insert(chunk_write_pos..chunk_write_pos + CHUNK_SIZE + 1);

            let url = self.url.clone();
            let file_size = self.buffer.len();
            let (tx, receiver) = channel();

            let id = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            self.receivers.push(Receiver { id, receiver });

            let error_sender = self.error_sender.clone();
            thread::spawn(move || {
                let result = Self::read_chunk(tx, url, chunk_write_pos, file_size);

                if let Err(err) = result {
                    let _ = error_sender
                        .send(PlayerEvent::Error(Error::NetworkStream(err.to_string())));
                }
            });
        }

        // Write any new bytes.
        let should_buffer = !self.downloaded.contains(&self.read_position);
        // If this source is active, then allow buffering in `cpal_output`.
        self.buffer_signal
            .store(should_buffer, std::sync::atomic::Ordering::SeqCst);
        self.try_write_chunk(should_buffer);

        // These are the bytes that we want to read.
        let bytes = &self.buffer[self.read_position..read_max];
        buf[0..bytes.len()].copy_from_slice(bytes);

        self.read_position += bytes.len();
        Ok(bytes.len())
    }
}

impl Seek for HttpStream
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

unsafe impl Send for HttpStream {}
unsafe impl Sync for HttpStream {}

impl MediaSource for HttpStream
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
