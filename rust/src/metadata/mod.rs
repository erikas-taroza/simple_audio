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

use self::types::{Event, Metadata, Actions};

use crate::utils::types::PlaybackState;

pub mod mpris;
pub mod smtc;
pub mod types;

/// Initialize a platform specific metadata handler.
#[allow(unused_variables)]
pub fn init<C>(actions: Vec<Actions>, dbus_name: String, hwnd: Option<i64>, callback: C)
where
    C: Fn(Event) + Send + 'static
{
    if actions.is_empty() { return; }

    #[cfg(all(unix, not(target_os = "macos"), not(target_os = "android"), not(target_os = "ios")))]
    {
        let mut mpris = mpris::HANDLER.write().unwrap();
        *mpris = Some(mpris::Mpris::new(actions, dbus_name, callback));
    }

    #[cfg(target_os = "windows")]
    {
        // Ignore this if a test is active. SMTC needs a HWND which is
        // not available when running the tests.
        if cfg!(test) { return; }

        let mut smtc = smtc::HANDLER.write().unwrap();
        *smtc = Some(smtc::Smtc::new(actions, hwnd.unwrap() as isize, callback));
    }
}

/// Stops the OS's media controller.
pub fn dispose()
{
    #[cfg(all(unix, not(target_os = "macos"), not(target_os = "android"), not(target_os = "ios")))]
    {
        let mut lock = mpris::HANDLER.write().unwrap();
        if lock.is_none() { return; }
        let mpris = (*lock).take().unwrap();
        mpris.stop();
    }

    #[cfg(target_os = "windows")]
    {
        let smtc = smtc::HANDLER.read().unwrap();
        if smtc.is_none() { return; }
        // TODO
        *smtc = None;
    }
}

pub fn set_metadata(metadata: Metadata)
{
    #[cfg(all(unix, not(target_os = "macos"), not(target_os = "android"), not(target_os = "ios")))]
    {
        let mpris = mpris::HANDLER.read().unwrap();
        if mpris.is_none() { return; }
        mpris.as_ref().unwrap().set_metadata(metadata);
    }

    #[cfg(target_os = "windows")]
    {
        let smtc = smtc::HANDLER.read().unwrap();
        if smtc.is_none() { return; }
        smtc.as_ref().unwrap().set_metadata(metadata);
    }
}


pub fn set_playback_state(state: PlaybackState)
{
    #[cfg(all(unix, not(target_os = "macos"), not(target_os = "android"), not(target_os = "ios")))]
    {
        let mpris = mpris::HANDLER.read().unwrap();
        if mpris.is_none() { return; }
        mpris.as_ref().unwrap().set_playback_state(state);
    }

    #[cfg(target_os = "windows")]
    {
        let smtc = smtc::HANDLER.read().unwrap();
        if smtc.is_none() { return; }
        smtc.as_ref().unwrap().set_playback_state(state);
    }
}