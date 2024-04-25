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

use chrono::Duration;

/// The playback state of the player.
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

/// Events that are handled in Dart because they need user action.
pub enum Callback
{
    Error(super::error::Error),
    /// The player started playing a new file. Contains the duration of the file.
    /// This is meant to be used to send a new duration to the media controller.
    PlaybackStarted(Duration),
}
