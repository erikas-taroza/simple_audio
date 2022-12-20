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

use crossbeam::channel::{Sender, Receiver};

use crate::utils::progress_state_stream::ProgressState;

pub static TXRX:RwLock<Option<(Sender<ThreadMessage>, Receiver<ThreadMessage>)>> = RwLock::new(None);
pub static IS_PLAYING:AtomicBool = AtomicBool::new(false);
pub static VOLUME:RwLock<f32> = RwLock::new(1.0);
pub static SEEK_TS:RwLock<Option<u64>> = RwLock::new(None);
pub static PROGRESS:RwLock<ProgressState> = RwLock::new(ProgressState { position: 0, duration: 0 });

pub enum ThreadMessage
{
    Play,
    Pause,
    Quit
}