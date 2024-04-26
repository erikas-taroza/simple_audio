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

use std::sync::{atomic::AtomicBool, Arc, RwLock, RwLockReadGuard};

use crossbeam::channel::{unbounded, Receiver, Sender};
use std::time::Duration;
use symphonia::core::io::MediaSource;

use crate::types::*;

/// Creates a getter and setter for an AtomicBool.
macro_rules! getset_atomic_bool {
    ($name:ident, $setter_name:ident) => {
        pub fn $name(&self) -> bool
        {
            self.$name.load(std::sync::atomic::Ordering::SeqCst)
        }

        pub fn $setter_name(&self, value: bool)
        {
            self.$name.store(value, std::sync::atomic::Ordering::SeqCst);
        }
    };
}

/// Creates a getter and setter for an RwLock.
macro_rules! getset_rwlock {
    ($name:ident, $setter_name:ident, $lock_type:ty) => {
        pub fn $name(&self) -> RwLockReadGuard<'_, $lock_type>
        {
            self.$name.read().unwrap()
        }

        pub fn $setter_name(&self, value: $lock_type)
        {
            *self.$name.write().unwrap() = value;
        }
    };
}

type EventHandler<T> = (Sender<T>, Receiver<T>);

#[derive(Clone)]
pub struct Controls
{
    decoder_event_handler: Arc<RwLock<EventHandler<DecoderEvent>>>,
    player_event_handler: Arc<RwLock<EventHandler<PlayerEvent>>>,
    playback_state: Arc<RwLock<PlaybackState>>,
    is_looping: Arc<AtomicBool>,
    is_normalizing: Arc<AtomicBool>,
    is_preloaded: Arc<AtomicBool>,
    volume: Arc<RwLock<f32>>,
    seek_ts: Arc<RwLock<Option<Duration>>>,
    progress: Arc<RwLock<ProgressState>>,
}

impl Controls
{
    getset_rwlock!(
        decoder_event_handler,
        _set_decoder_event_handler,
        EventHandler<DecoderEvent>
    );
    getset_rwlock!(
        player_event_handler,
        _set_player_event_handler,
        EventHandler<PlayerEvent>
    );
    getset_rwlock!(playback_state, set_playback_state, PlaybackState);
    getset_atomic_bool!(is_looping, set_is_looping);
    getset_atomic_bool!(is_normalizing, set_is_normalizing);
    getset_atomic_bool!(is_preloaded, set_is_file_preloaded);
    getset_rwlock!(volume, set_volume, f32);
    getset_rwlock!(seek_ts, set_seek_ts, Option<Duration>);
    getset_rwlock!(progress, set_progress, ProgressState);
}

impl Default for Controls
{
    fn default() -> Self
    {
        Controls {
            decoder_event_handler: Arc::new(RwLock::new(unbounded())),
            player_event_handler: Arc::new(RwLock::new(unbounded())),
            playback_state: Arc::new(RwLock::new(PlaybackState::Stop)),
            is_looping: Arc::new(AtomicBool::new(false)),
            is_normalizing: Arc::new(AtomicBool::new(false)),
            is_preloaded: Arc::new(AtomicBool::new(false)),
            volume: Arc::new(RwLock::new(1.0)),
            seek_ts: Arc::new(RwLock::new(None)),
            progress: Arc::new(RwLock::new(ProgressState {
                position: Duration::ZERO,
                duration: Duration::ZERO,
            })),
        }
    }
}

/// Messages to communicate with the decoder from the player.
pub enum DecoderEvent
{
    Open(Box<dyn MediaSource>, Arc<AtomicBool>),
    Play,
    Pause,
    Stop,
    /// Called by `cpal_output` in the event the device outputting
    /// audio was changed/disconnected.
    DeviceChanged,
    Preload(Box<dyn MediaSource>, Arc<AtomicBool>),
    PlayPreload,
    ClearPreload,
}
