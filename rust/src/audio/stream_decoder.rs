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

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use cpal::traits::StreamTrait;
use crossbeam::channel::unbounded;
use rb::RbConsumer;
use reqwest::blocking::Client;
use symphonia::{core::{io::MediaSourceStream, formats::{FormatOptions, SeekTo, SeekMode, FormatReader}, meta::MetadataOptions, probe::Hint, units::Time}, default};
use tempfile::NamedTempFile;

use crate::utils::{progress_state_stream::*, playback_state_stream::*};

use super::{controls::*, cpal_output::CpalOutput};

const CHUNK_SIZE:u64 = 1024 * 64;
const FETCH_OFFSET:u64 = 10000;

pub struct StreamDecoder
{
    url:String,
    source:NamedTempFile,
    read_pos:u64,
    // skipped:Vec<Range>
}

impl StreamDecoder
{
    pub fn new(url:String) -> StreamDecoder
    {
        StreamDecoder
        {
            url,
            source: tempfile::NamedTempFile::new().unwrap(),
            read_pos: 0,
            // skipped: Vec::new()
        }
    }

    pub fn decode(&mut self)
    {
        self.get_chunk(0);

        println!("{:?}", self.source.path());

        let file = File::open(self.source.path()).unwrap();
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let format_options = FormatOptions { enable_gapless: true, ..Default::default() };
        let metadata_options:MetadataOptions = Default::default();

        match default::get_probe().format(&Hint::new(), mss, &format_options, &metadata_options)
        {
            Err(err) => panic!("ERR: Failed to probe source. {err}"),
            Ok(mut probed) => self.decode_loop(&mut probed.format)
        }
    }

    fn decode_loop(&mut self, reader:&mut Box<dyn FormatReader>)
    {
        let track = reader.default_track().unwrap();
        let track_id = track.id;

        let mut decoder = default::get_codecs().make(&track.codec_params, &Default::default()).unwrap();
        let mut cpal_output:Option<CpalOutput> = None;

        // Used only for outputting the current position and duration.
        let timebase = track.codec_params.time_base.unwrap();
        let duration = track.codec_params.n_frames.map(|frames| track.codec_params.start_ts + frames).unwrap();
        let duration = timebase.calc_time(duration).seconds;

        let mut lock = PROGRESS.write().unwrap();
        *lock = ProgressState { position: 0, duration };
        drop(lock);

        // Clone a receiver to listen for the stop signal.
        let rx = TXRX.read().unwrap().1.clone();

        let lock = TXRX2.read().unwrap();
        let test = unbounded();
        let tx = lock.as_ref().unwrap_or(&test).0.clone();
        drop(lock);

        let mut has_reached_end = false;

        loop
        {
            // Get a chunk if we are running out of data.
            let len = self.source.as_file().metadata().unwrap().len();
            if self.read_pos >= len.saturating_sub(FETCH_OFFSET) {
                self.get_chunk(len);
            }

            // Poll the status of the RX in lib.rs.
            // If the player is paused, then block this thread until a message comes in
            // to save the CPU.
            let recv:Option<ThreadMessage> = if !IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst)
                // This will always be None on the first iteration
                // which is good because we don't need to block
                // on the first iteration.
                && cpal_output.is_some()
            {
                rx.recv().ok()
            } else {
                rx.try_recv().ok()
            };

            match recv
            {
                None => (),
                Some(message) => match message
                {
                    ThreadMessage::Play if cfg!(not(target_os = "windows")) => {
                        if let Some(cpal_output) = cpal_output.as_ref()
                        { cpal_output.stream.play().unwrap(); } 
                    },
                    ThreadMessage::Pause if cfg!(not(target_os = "windows")) => {
                        if let Some(cpal_output) = cpal_output.as_ref()
                        { cpal_output.stream.pause().unwrap(); } 
                    },
                    ThreadMessage::Stop => break,
                    _ => ()
                }
            }

            if !IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst)
            { continue; }

            // Seeking.
            let seek_ts:u64 = if let Some(seek_ts) = *SEEK_TS.read().unwrap()
            {
                //TODO: Seeking
                let pos = timebase.calc_timestamp(Time::from(seek_ts));
                self.seek(pos - 1024);
                println!("{pos}");

                let seek_to = SeekTo::Time { time: Time::from(seek_ts), track_id: Some(track_id) };
                match reader.seek(SeekMode::Accurate, seek_to)
                {
                    Ok(seeked_to) => seeked_to.required_ts,
                    Err(_) => 0
                }
            } else { 0 };

            // Clean up seek stuff.
            if SEEK_TS.read().unwrap().is_some()
            {
                *SEEK_TS.write().unwrap() = None;
                decoder.reset();
                // Clear the ring buffer which prevents the writer
                // from blocking.
                if let Some(cpal_output) = cpal_output.as_ref()
                { let _ = cpal_output.ring_buffer_reader.skip(usize::MAX); }
                continue;
            }

            // Decode the next packet.
            let packet = match reader.next_packet()
            {
                Ok(packet) => packet,
                // An error occurs when the stream
                // has reached the end of the audio.
                Err(_) => {
                    has_reached_end = true;
                    break;
                }
            };

            if packet.track_id() != track_id { continue; }

            // Keep track of how much data we are reading.
            self.read_pos += packet.data.len() as u64;

            match decoder.decode(&packet)
            {
                Err(err) => println!("WARN: Failed to decode sound. {err}"),
                Ok(decoded) => {
                    if packet.ts() < seek_ts { continue; }
                    
                    // Update the progress stream with calculated times.
                    let progress = ProgressState {
                        position: timebase.calc_time(packet.ts()).seconds,
                        duration
                    };

                    update_progress_state_stream(progress);
                    *PROGRESS.write().unwrap() = progress;

                    // Write the decoded packet to CPAL.
                    if cpal_output.is_none()
                    {
                        let spec = *decoded.spec();
                        let duration = decoded.capacity() as u64;
                        cpal_output.replace(CpalOutput::build_stream(spec, duration));
                    }

                    cpal_output.as_mut().unwrap().write(decoded);
                }
            }
        }

        // Tell the main thread that everything is done here.
        tx.send(true).unwrap();

        // This code is only ran if the stream
        // reached the end of the audio.
        if !has_reached_end { return; }
        
        // Flushing isn't needed when the user deliberately
        // stops the stream because they no longer care about the remaining samples.
        if let Some(cpal_output) = cpal_output.as_mut()
        { cpal_output.flush(); }

        // Send the done message once cpal finishes flushing.
        // There may be samples left over and we don't want to
        // start playing another file before they are read.
        update_playback_state_stream(crate::utils::playback_state::PlaybackState::Done);
        update_progress_state_stream(ProgressState { position: 0, duration: 0 });
        *PROGRESS.write().unwrap() = ProgressState { position: 0, duration: 0 };
        IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
        crate::metadata::set_playback_state(crate::utils::playback_state::PlaybackState::Done);
    }

    /// Gets the next chunk in the sequence.
    /// 
    /// Returns the number of bytes received.
    fn get_chunk(&mut self, start:u64) -> u64
    {
        // Get the next chunk.
        let end = start + CHUNK_SIZE;
        let mut chunk = Client::new().get(self.url.clone())
            .header("Range", format!("bytes={start}-{end}"))
            .send().unwrap().bytes().unwrap().to_vec();
        
        let num_received = chunk.len() as u64;

        // Add chunk to file.
        let file = self.source.as_file_mut();
        let file_len = file.metadata().unwrap().len();

        println!("Received chunk num[{}]: {start}-{end}", file_len / CHUNK_SIZE);

        // When the chunk is in consecutive order.
        if file_len < end {
            let prev = file.stream_position().unwrap();
            
            file.set_len(file_len + CHUNK_SIZE).unwrap();
            file.seek(SeekFrom::Start(file_len)).unwrap();
            file.write_all(&mut chunk).unwrap();
            file.seek(SeekFrom::Start(prev)).unwrap();
        }
        // When the chunk is located earlier in the file.
        else {
            // self.buffer[start..start + num_received].copy_from_slice(&chunk);
        }

        // We have finished filling the buffer if we do not receive
        // any more data.
        if num_received == 0 {
            // self.finished_writing = true;
        }

        num_received
    }

    /// Appends 0 to the file until `position` is reached.
    /// 
    /// Reads a chunk at position.
    fn seek(&mut self, position:u64)
    {
        let file = self.source.as_file_mut();
        file.set_len(position).unwrap();
        self.get_chunk(position);
    }
}