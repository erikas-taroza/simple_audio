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

// NOTE: When updating the enum values, they need to be updated
// in Kotlin and in Rust.
/// The actions that an OS's media controller can support.
enum NotificationActions
{
    /// Seeks backwards by 10 seconds.
    rewind,
    /// Skip to the previous playing file (you will implement this functionality).
    skipPrev,
    /// Play/pause the player.
    playPause,
    /// Skip to the next file to be played (you will implement this functionality).
    skipNext,
    /// Seeks forwards by 10 seconds.
    fastForward
}