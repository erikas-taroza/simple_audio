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

// A file that defines controls globally.

use std::sync::{atomic::AtomicBool, Arc, RwLock};

use crossbeam::channel::{unbounded, Receiver, Sender};
use lazy_static::lazy_static;
use symphonia::core::io::MediaSource;

use crate::utils::types::ProgressState;

lazy_static! {
    /// Use this to communicate between the main thread and the decoder thread.
    /// Ex: play/pause commands.
    pub static ref TXRX: RwLock<(Sender<ThreadMessage>, Receiver<ThreadMessage>)> = RwLock::new(unbounded());
}

#[derive(Clone)]
pub struct Controls
{
    pub is_playing: Arc<AtomicBool>,
    pub is_stopped: Arc<AtomicBool>,
    pub is_looping: Arc<AtomicBool>,
    pub is_normalizing: Arc<AtomicBool>,
    pub is_file_preloaded: Arc<AtomicBool>,
    pub volume: Arc<RwLock<f32>>,
    pub seek_ts: Arc<RwLock<Option<u64>>>,
    pub progress: Arc<RwLock<ProgressState>>,
}

impl Default for Controls
{
    fn default() -> Self
    {
        Controls {
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
    /// Stops the current running thread.
    Dispose,
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
