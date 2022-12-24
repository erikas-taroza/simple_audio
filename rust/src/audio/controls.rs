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

// A file that defines controls globally.

use std::sync::{RwLock, atomic::AtomicBool};

use crossbeam::channel::{Sender, Receiver, unbounded};
use lazy_static::lazy_static;

use crate::utils::progress_state_stream::ProgressState;

lazy_static!
{
    /// Use this to communicate between the main thread and the decoder thread
    /// Ex: play/pause commands
    pub static ref TXRX:RwLock<(Sender<ThreadMessage>, Receiver<ThreadMessage>)> = RwLock::new(unbounded());

    /// Use this to communicate from the decoder to the main thread.
    /// Ex: Tell the main thread this thread is done.
    // This is an option because we don't want to wait for a non existent thread on the first run.
    pub static ref TXRX2:RwLock<Option<(Sender<bool>, Receiver<bool>)>> = RwLock::new(None);
}

pub static IS_PLAYING:AtomicBool = AtomicBool::new(false);
pub static VOLUME:RwLock<f32> = RwLock::new(1.0);
pub static SEEK_TS:RwLock<Option<u64>> = RwLock::new(None);
pub static PROGRESS:RwLock<ProgressState> = RwLock::new(ProgressState { position: 0, duration: 0 });

pub enum ThreadMessage
{
    Play,
    Pause,
    Stop
}