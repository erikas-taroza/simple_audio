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

import 'dart:typed_data';

import 'bridge_definitions.dart' as inner;

// NOTE: These are already defined in bridge_definitions.dart
// but they are redefined here to provide documentation.

/// Provides the current progress of the player.
class ProgressState
{
    /// The position, in seconds, of the player.
    final int position;
    /// The duration, in seconds, of the file that
    /// is being played.
    final int duration;

    /// Provides the current progress of the player.
    ProgressState({
        required this.position,
        required this.duration,
    });
}

/// Declared as a top level function so that it can be
/// hidden in the public API.
ProgressState progressStateFromInner(inner.ProgressState other)
{
    return ProgressState(
        position: other.position,
        duration: other.duration
    );
}

/// The metadata of the currently playing file
/// that will be shown in the OS's media controller.
class Metadata
{
    /// The title of the file.
    final String? title;
    /// The artist/creator of the file.
    final String? artist;
    /// The album that the song is in.
    final String? album;
    /// A URI that points to the art for this song.
    final String? artUri;
    /// The song's art in the form of a byte array.
    final Uint8List? artBytes;

    /// The metadata of the currently playing file
    /// that will be shown in the OS's media controller.
    Metadata({
        this.title,
        this.artist,
        this.album,
        this.artUri,
        this.artBytes,
    });
}

/// Declared as a top level function so that it can be
/// hidden in the public API.
Metadata metadataFromInner(inner.Metadata other)
{
    return Metadata(
        title: other.title,
        artist: other.artist,
        album: other.album,
        artUri: other.artUri,
        artBytes: other.artBytes
    );
}

/// The playback state of the player.
enum PlaybackState
{
    /// The player is currently playing the file.
    play,
    /// The player is currently paused and there is no output.
    pause,
    /// The player has finished playing the file.
    done
}

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