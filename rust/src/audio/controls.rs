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

use std::sync::{RwLock, atomic::AtomicBool};

use crossbeam::channel::{Sender, Receiver, unbounded};
use lazy_static::lazy_static;
use symphonia::core::io::MediaSource;

use crate::utils::types::ProgressState;

lazy_static!
{
    /// Use this to communicate between the main thread and the decoder thread.
    /// Ex: play/pause commands.
    pub static ref TXRX: RwLock<(Sender<ThreadMessage>, Receiver<ThreadMessage>)> = RwLock::new(unbounded());
}

pub static IS_PLAYING: AtomicBool = AtomicBool::new(false);
pub static IS_STOPPED: AtomicBool = AtomicBool::new(true);
pub static IS_LOOPING: AtomicBool = AtomicBool::new(false);
pub static IS_NORMALIZING: AtomicBool = AtomicBool::new(false);
pub static IS_FILE_QUEUED: AtomicBool = AtomicBool::new(false);
pub static VOLUME: RwLock<f32> = RwLock::new(1.0);
pub static SEEK_TS: RwLock<Option<u64>> = RwLock::new(None);
pub static PROGRESS: RwLock<ProgressState> = RwLock::new(ProgressState { position: 0, duration: 0 });

pub fn reset_controls_to_default()
{
    IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
    IS_STOPPED.store(true, std::sync::atomic::Ordering::SeqCst);
    IS_LOOPING.store(false, std::sync::atomic::Ordering::SeqCst);
    IS_NORMALIZING.store(false, std::sync::atomic::Ordering::SeqCst);
    *VOLUME.write().unwrap() = 1.0;
    *SEEK_TS.write().unwrap() = None;
    *PROGRESS.write().unwrap() = ProgressState { position: 0, duration: 0 };
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
    Queue(Box<dyn MediaSource>),
    PlayQueue,
}