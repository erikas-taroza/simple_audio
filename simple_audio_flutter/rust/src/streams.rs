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

use std::sync::{OnceLock, RwLock};

use chrono::Duration;
use flutter_rust_bridge::StreamSink;

use crate::api::{Error, PlaybackState, ProgressState};

static PLAYBACK_STARTED_STREAM: OnceLock<RwLock<Option<StreamSink<Duration>>>> = OnceLock::new();

/// Creates a new playback started stream.
pub fn playback_started_stream(stream: StreamSink<Duration>)
{
    *PLAYBACK_STARTED_STREAM
        .get_or_init(|| RwLock::new(None))
        .write()
        .unwrap() = Some(stream);
}

/// Updates/adds to the stream with the given value.
pub fn update_playback_started_stream(value: Duration)
{
    if let Some(lock) = PLAYBACK_STARTED_STREAM.get() {
        if let Some(stream) = &*lock.read().unwrap() {
            stream.add(value);
        }
    }
}

static PLAYBACK_STATE_STREAM: OnceLock<RwLock<Option<StreamSink<PlaybackState>>>> = OnceLock::new();

/// Creates a new playback stream.
pub fn playback_state_stream(stream: StreamSink<PlaybackState>)
{
    *PLAYBACK_STATE_STREAM
        .get_or_init(|| RwLock::new(None))
        .write()
        .unwrap() = Some(stream);
}

/// Updates/adds to the stream with the given value.
pub fn update_playback_state_stream(value: PlaybackState)
{
    if let Some(lock) = PLAYBACK_STATE_STREAM.get() {
        if let Some(stream) = &*lock.read().unwrap() {
            stream.add(value);
        }
    }
}

static PROGRESS_STATE_STREAM: OnceLock<RwLock<Option<StreamSink<ProgressState>>>> = OnceLock::new();

/// Creates a new progress stream.
pub fn progress_state_stream(stream: StreamSink<ProgressState>)
{
    *PROGRESS_STATE_STREAM
        .get_or_init(|| RwLock::new(None))
        .write()
        .unwrap() = Some(stream);
}

/// Updates/adds to the stream with the given value.
pub fn update_progress_state_stream(value: ProgressState)
{
    if let Some(lock) = PROGRESS_STATE_STREAM.get() {
        if let Some(stream) = &*lock.read().unwrap() {
            stream.add(value);
        }
    }
}

static ERROR_STREAM: OnceLock<RwLock<Option<StreamSink<Error>>>> = OnceLock::new();

/// Creates a new error stream.
pub fn error_stream(stream: StreamSink<Error>)
{
    *ERROR_STREAM
        .get_or_init(|| RwLock::new(None))
        .write()
        .unwrap() = Some(stream);
}

/// Updates/adds to the stream with the given value.
pub fn update_error_stream(value: Error)
{
    if let Some(lock) = ERROR_STREAM.get() {
        if let Some(stream) = &*lock.read().unwrap() {
            stream.add(value);
        }
    }
}
