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

use std::sync::{atomic::AtomicBool, Mutex, MutexGuard};

pub mod hls;
pub mod http;
pub mod local;
pub mod streamable;

// Used in cpal_output.rs to mute the stream when buffering.
pub static IS_STREAM_BUFFERING: AtomicBool = AtomicBool::new(false);
// Used to ensure that only one file can access
// things that should only be controlled by one source.
// For example, `IS_STREAM_BUFFERING` shouldn't be set
// when preloading.
static ACTIVE_LOCK: Mutex<()> = Mutex::new(());

/// Attempts to set this source as active by returning a guard.
/// If another source already has a lock, this returns `None`.
fn try_get_active_lock() -> Option<MutexGuard<'static, ()>>
{
    ACTIVE_LOCK.try_lock().ok()
}

/// A type that holds an ID and a `std::sync::mpsc::Receiver`.
/// Used for multithreaded download of audio data.
struct Receiver
{
    id: u128,
    receiver: std::sync::mpsc::Receiver<(usize, Vec<u8>)>,
}
