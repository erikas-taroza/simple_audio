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

use std::sync::RwLock;

use self::types::{Event, MediaControlAction, MediaController, Metadata};

use crate::utils::types::PlaybackState;

pub mod mpris;
pub mod smtc;
pub mod types;

static MEDIA_CONTROLLER: RwLock<Option<Box<dyn MediaController>>> = RwLock::new(None);

/// Initialize a platform specific metadata handler.
#[allow(unused_variables)]
pub fn init<C>(actions: Vec<MediaControlAction>, dbus_name: String, hwnd: Option<i64>, callback: C)
where
    C: Fn(Event) + Send + 'static,
{
    if actions.is_empty() {
        return;
    }

    #[cfg(all(
        unix,
        not(target_os = "macos"),
        not(target_os = "android"),
        not(target_os = "ios")
    ))]
    {
        let mut lock = MEDIA_CONTROLLER.write().unwrap();
        *lock = Some(Box::new(mpris::Mpris::new(actions, dbus_name, callback)));
    }

    #[cfg(target_os = "windows")]
    {
        // Ignore this if a test is active. SMTC needs a HWND which is
        // not available when running the tests.
        if cfg!(test) {
            return;
        }

        let mut lock = MEDIA_CONTROLLER.write().unwrap();
        *lock = Some(Box::new(smtc::Smtc::new(
            actions,
            hwnd.unwrap() as isize,
            callback,
        )));
    }
}

/// Stops the OS's media controller.
pub fn dispose()
{
    let mut lock = MEDIA_CONTROLLER.write().unwrap();
    if lock.is_none() {
        return;
    }

    let controller = (*lock).take().unwrap();
    controller.stop();
}

pub fn set_metadata(metadata: Metadata)
{
    let lock = MEDIA_CONTROLLER.read().unwrap();
    if lock.is_none() {
        return;
    }

    lock.as_ref().unwrap().set_metadata(metadata);
}

/// Sets the current position of playback for the OS's
/// media controller. This should be called once every second.
pub fn set_position(position: u64)
{
    let lock = MEDIA_CONTROLLER.read().unwrap();
    if lock.is_none() {
        return;
    }

    lock.as_ref().unwrap().set_position(position);
}

/// Sets the file's duration for the OS's media controller.
///
/// This should be called as soon as the duration is calculated
/// in the decoder.
pub fn set_duration(duration: u64)
{
    let lock = MEDIA_CONTROLLER.read().unwrap();
    if lock.is_none() {
        return;
    }

    lock.as_ref().unwrap().set_duration(duration);
}

pub fn set_playback_state(state: PlaybackState)
{
    let lock = MEDIA_CONTROLLER.read().unwrap();
    if lock.is_none() {
        return;
    }

    lock.as_ref().unwrap().set_playback_state(state);
}
