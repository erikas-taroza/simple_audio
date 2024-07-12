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

use std::time::Duration;

use crate::error::Error;

/// The playback state of the player.
#[derive(Clone, Copy, Debug)]
pub enum PlaybackState
{
    /// The player is currently playing the file.
    Play,
    /// The player is currently paused and there is no output.
    Pause,
    /// The player has finished playing the file.
    Done,
    /// The player was stopped
    Stop,
    /// The player has automatically started playing the preloaded file.
    PreloadPlayed,
}

/// Provides the current progress of the player.
#[derive(Clone, Copy)]
pub struct ProgressState
{
    /// The position of the player.
    pub position: Duration,
    /// The duration of the file that is being played.
    pub duration: Duration,
}

/// Messages to communicate with the player from the decoder.
pub enum PlayerEvent
{
    PlaybackStarted(Duration),
    Playback(PlaybackState),
    Progress(ProgressState),
    Error(Error),
}
