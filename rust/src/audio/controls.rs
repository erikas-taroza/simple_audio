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
use lazy_static::lazy_static;
use symphonia::core::io::MediaSource;

use crate::utils::types::ProgressState;

lazy_static! {
    /// Use this to stop the decoder thread.
    pub static ref THREAD_KILLER: RwLock<(Sender<bool>, Receiver<bool>)> = RwLock::new(unbounded());
}

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

type EventHandler = (Sender<ThreadMessage>, Receiver<ThreadMessage>);

#[derive(Clone)]
pub struct Controls
{
    event_handler: Arc<RwLock<EventHandler>>,
    is_playing: Arc<AtomicBool>,
    is_stopped: Arc<AtomicBool>,
    is_looping: Arc<AtomicBool>,
    is_normalizing: Arc<AtomicBool>,
    is_file_preloaded: Arc<AtomicBool>,
    volume: Arc<RwLock<f32>>,
    seek_ts: Arc<RwLock<Option<u64>>>,
    progress: Arc<RwLock<ProgressState>>,
}

impl Controls
{
    getset_rwlock!(event_handler, _set_event_handler, EventHandler);
    getset_atomic_bool!(is_playing, set_is_playing);
    getset_atomic_bool!(is_stopped, set_is_stopped);
    getset_atomic_bool!(is_looping, set_is_looping);
    getset_atomic_bool!(is_normalizing, set_is_normalizing);
    getset_atomic_bool!(is_file_preloaded, set_is_file_preloaded);
    getset_rwlock!(volume, set_volume, f32);
    getset_rwlock!(seek_ts, set_seek_ts, Option<u64>);
    getset_rwlock!(progress, set_progress, ProgressState);
}

impl Default for Controls
{
    fn default() -> Self
    {
        Controls {
            event_handler: Arc::new(RwLock::new(unbounded())),
            is_playing: Arc::new(AtomicBool::new(false)),
            is_stopped: Arc::new(AtomicBool::new(true)),
            is_looping: Arc::new(AtomicBool::new(false)),
            is_normalizing: Arc::new(AtomicBool::new(false)),
            is_file_preloaded: Arc::new(AtomicBool::new(false)),
            volume: Arc::new(RwLock::new(1.0)),
            seek_ts: Arc::new(RwLock::new(None)),
            progress: Arc::new(RwLock::new(ProgressState {
                position: 0,
                duration: 0,
            })),
        }
    }
}

pub enum ThreadMessage
{
    Open(Box<dyn MediaSource>),
    Play,
    Pause,
    Stop,
    /// Called by `cpal_output` in the event the device outputting
    /// audio was changed/disconnected.
    DeviceChanged,
    Preload(Box<dyn MediaSource>),
    PlayPreload,
}
