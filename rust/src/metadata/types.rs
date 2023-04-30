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

use crate::utils::types::PlaybackState;

pub trait MediaController: Send + Sync
{
    fn set_metadata(&self, metadata: Metadata);
    fn set_duration(&self, duration: u64);
    fn set_playback_state(&self, state: PlaybackState);
    fn stop(&self);
}

/// The metadata of the currently playing file
/// that will be shown in the OS's media controller.
#[derive(Default)]
pub struct Metadata
{
    /// The title of the file.
    pub title: Option<String>,
    /// The artist/creator of the file.
    pub artist: Option<String>,
    /// The album that the song is in.
    pub album: Option<String>,
    /// A URI that points to the art for this song.
    pub art_uri: Option<String>,
    /// The song's art in the form of a byte array.
    pub art_bytes: Option<Vec<u8>>,
}

// NOTE: When updating the enum values, they need to be updated
// in Kotlin and Swift.
/// The actions that an OS's media controller can support.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MediaControlAction
{
    /// Seeks backwards by 10 seconds.
    Rewind,
    /// Skip to the previous playing file (you will implement this functionality).
    SkipPrev,
    /// Play/pause the player.
    PlayPause,
    /// Skip to the next file to be played (you will implement this functionality).
    SkipNext,
    /// Seeks forwards by 10 seconds.
    FastForward,
}

impl From<i32> for MediaControlAction
{
    fn from(i: i32) -> Self
    {
        match i {
            0 => Self::Rewind,
            1 => Self::SkipPrev,
            2 => Self::PlayPause,
            3 => Self::SkipNext,
            4 => Self::FastForward,
            _ => panic!("ERR: This action is not supported."),
        }
    }
}

/// Callback events from the media notification.
#[derive(Clone, Copy, Debug)]
pub enum Event
{
    Next,
    Previous,
    Play,
    Pause,
    Stop,
    PlayPause,
    /// `i64`: Position.
    ///
    /// `bool`: Is absolute.
    /// If `true`, the position is between `0-duration`.
    /// If false, the position can be negative to indicate going backwards.
    Seek(i64, bool),
}

#[cfg(all(
    unix,
    not(target_os = "macos"),
    not(target_os = "android"),
    not(target_os = "ios")
))]
/// Commands to be sent via the thread's channels for MPRIS.
pub enum Command
{
    SetMetadata(Metadata),
    SetDuration(u64),
    SetPlaybackState(PlaybackState),
    Stop,
}
