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

use chrono::Duration;
use crossbeam::channel::{unbounded, Receiver, Sender};
use flutter_rust_bridge::{RustOpaque, StreamSink};
use simple_audio::types::PlayerEvent;
pub use simple_audio::Player;

use std::{sync::OnceLock, thread};

use crate::streams::*;

static EVENT_THREAD_KILLER: OnceLock<(Sender<bool>, Receiver<bool>)> = OnceLock::new();
static PLAYER_THREAD_KILLER: OnceLock<(Sender<bool>, Receiver<bool>)> = OnceLock::new();

pub struct PlayerWrapper
{
    pub(crate) internal: RustOpaque<Player>,
}

impl PlayerWrapper
{
    pub fn new() -> PlayerWrapper
    {
        // Enable logging from Rust to Android logcat.
        // `android_logger::init_once` can safely be called multiple times
        // but will only initialize once.
        #[cfg(all(debug_assertions, target_os = "android"))]
        {
            use android_logger::Config;
            use log::LevelFilter;
            android_logger::init_once(Config::default().with_max_level(LevelFilter::Debug));
        }

        let _ = EVENT_THREAD_KILLER.set(unbounded());
        let _ = PLAYER_THREAD_KILLER.set(unbounded());

        let internal = Player::new(PLAYER_THREAD_KILLER.get().unwrap().1.clone());

        thread::spawn({
            let event_receiver = internal.event_receiver.clone();
            let event_thread_killer = EVENT_THREAD_KILLER.get().unwrap().1.clone();
            move || loop {
                if let Ok(should_stop) = event_thread_killer.try_recv() {
                    if should_stop {
                        break;
                    }
                }

                if let Ok(event) = event_receiver.recv() {
                    match event {
                        PlayerEvent::PlaybackStarted(duration) => {
                            update_playback_started_stream(Duration::from_std(duration).unwrap())
                        }
                        PlayerEvent::Playback(playback) => {
                            update_playback_state_stream(playback.into())
                        }
                        PlayerEvent::Progress(progress) => {
                            update_progress_state_stream(progress.into())
                        }
                        PlayerEvent::Error(error) => update_error_stream(error.into()),
                    }
                }
            }
        });

        PlayerWrapper {
            internal: RustOpaque::new(internal),
        }
    }

    pub fn dispose()
    {
        if let Some(thread_killer) = EVENT_THREAD_KILLER.get() {
            thread_killer.0.send(true).unwrap();
        }

        if let Some(thread_killer) = PLAYER_THREAD_KILLER.get() {
            thread_killer.0.send(true).unwrap();
        }
    }

    pub fn playback_started_stream(stream: StreamSink<Duration>)
    {
        playback_started_stream(stream);
    }

    pub fn playback_state_stream(stream: StreamSink<PlaybackState>)
    {
        playback_state_stream(stream);
    }

    pub fn progress_state_stream(stream: StreamSink<ProgressState>)
    {
        progress_state_stream(stream);
    }

    pub fn error_stream(stream: StreamSink<Error>)
    {
        error_stream(stream);
    }

    pub fn playback_state(&self) -> PlaybackState
    {
        self.internal.playback_state().into()
    }

    pub fn progress(&self) -> ProgressState
    {
        self.internal.progress().into()
    }

    /// Returns `true` if there is a file preloaded for playback.
    pub fn is_preloaded(&self) -> bool
    {
        self.internal.is_preloaded()
    }

    pub fn is_looping(&self) -> bool
    {
        self.internal.is_looping()
    }

    pub fn is_normalizing(&self) -> bool
    {
        self.internal.is_normalizing()
    }

    pub fn volume(&self) -> f32
    {
        self.internal.volume()
    }

    pub fn open(&self, path: String, autoplay: bool) -> Result<(), Error>
    {
        self.internal.open(path, autoplay).map_err(|err| err.into())
    }

    pub fn preload(&self, path: String) -> Result<(), Error>
    {
        self.internal.preload(path).map_err(|err| err.into())
    }

    pub fn play_preload(&self)
    {
        self.internal.play_preload()
    }

    pub fn clear_preload(&self)
    {
        self.internal.clear_preload()
    }

    pub fn play(&self)
    {
        self.internal.play()
    }

    pub fn pause(&self)
    {
        self.internal.pause()
    }

    pub fn stop(&self)
    {
        self.internal.stop()
    }

    pub fn loop_playback(&self, should_loop: bool)
    {
        self.internal.loop_playback(should_loop)
    }

    pub fn set_volume(&self, volume: f32)
    {
        self.internal.set_volume(volume)
    }

    pub fn seek(&self, position: Duration)
    {
        self.internal
            .seek(position.to_std().unwrap_or(std::time::Duration::ZERO))
    }

    pub fn normalize_volume(&self, should_normalize: bool)
    {
        self.internal.normalize_volume(should_normalize)
    }
}

pub enum PlaybackState
{
    /// The player is currently playing the file.
    Play,
    /// The player is currently paused and there is no output.
    Pause,
    /// The player has finished playing the file.
    Done,
    /// The player was stopped
    Stop,
    /// The player has automatically started playing the preloaded file.
    PreloadPlayed,
}

impl Into<PlaybackState> for simple_audio::types::PlaybackState
{
    fn into(self) -> PlaybackState
    {
        match self {
            Self::Play => PlaybackState::Play,
            Self::Pause => PlaybackState::Pause,
            Self::Done => PlaybackState::Done,
            Self::Stop => PlaybackState::Stop,
            Self::PreloadPlayed => PlaybackState::PreloadPlayed,
        }
    }
}

pub struct ProgressState
{
    /// The position of the player.
    pub position: Duration,
    /// The duration of the file that is being played.
    pub duration: Duration,
}

impl Into<ProgressState> for simple_audio::types::ProgressState
{
    fn into(self) -> ProgressState
    {
        ProgressState {
            position: Duration::from_std(self.position).unwrap(),
            duration: Duration::from_std(self.duration).unwrap(),
        }
    }
}

pub enum Error
{
    /// An error occurred when trying to fetch more bytes for
    /// a network stream.
    NetworkStream(String),
    /// An error occurred when decoding the file.
    Decode(String),
    /// An error occurred when trying to open a file.
    Open(String),
    /// An error occurred when trying to preload a file.
    Preload(String),
}

impl Into<Error> for simple_audio::error::Error
{
    fn into(self) -> Error
    {
        match self {
            Self::NetworkStream(message) => Error::NetworkStream(message),
            Self::Decode(message) => Error::Decode(message),
            Self::Open(message) => Error::Open(message),
            Self::Preload(message) => Error::Preload(message),
        }
    }
}
