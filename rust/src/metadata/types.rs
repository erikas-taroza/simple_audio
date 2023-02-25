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

use crate::utils::playback_state::PlaybackState;

#[derive(Default)]
pub struct Metadata
{
    pub title:Option<String>,
    pub artist:Option<String>,
    pub album:Option<String>,
    pub art_uri:Option<String>,
    pub art_bytes:Option<Vec<u8>>
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
    Seek(i64, bool)
}

/// Commands to be sent via the thread's channels.
pub enum Command
{
    SetMetadata(Metadata),
    SetPlaybackState(PlaybackState)
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Actions
{
    Rewind,
    SkipPrev,
    PlayPause,
    SkipNext,
    FastForward
}

impl From<i32> for Actions
{
    fn from(i:i32) -> Self
    {
        match i
        {
            0 => Self::Rewind,
            1 => Self::SkipPrev,
            2 => Self::PlayPause,
            3 => Self::SkipNext,
            4 => Self::FastForward,
            _ => panic!("ERR: This action is not supported.")
        } 
    }
}