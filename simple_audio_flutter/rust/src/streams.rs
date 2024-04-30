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
