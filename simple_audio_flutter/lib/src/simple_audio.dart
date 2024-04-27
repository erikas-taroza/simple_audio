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

export 'bridge_definitions.dart' show ProgressState, PlaybackState;

import 'dart:async';

import './ffi.dart' hide PlaybackState;
import 'bridge_definitions.dart';

late final PlayerWrapper _player;

class SimpleAudio {
  late final Stream<Error> _error =
      PlayerWrapper.errorStream(bridge: api).asBroadcastStream();

  /// A stream that returns the [Duration] of the file when it begins playing or looping.
  /// The [Duration] is meant to be used in a media controller to support seeking and progress bars.
  late Stream<Duration> playbackStarted =
      PlayerWrapper.playbackStartedStream(bridge: api).asBroadcastStream();

  /// A stream that returns a [PlaybackState] when the state of the player is changed.
  late Stream<PlaybackState> playbackState =
      PlayerWrapper.playbackStateStream(bridge: api).asBroadcastStream();

  /// A stream that returns a [ProgressState] when the progress of the player
  /// or duration of the file is changed.
  late Stream<ProgressState> progressState =
      PlayerWrapper.progressStateStream(bridge: api).asBroadcastStream();

  late Stream<String> decodeError = _error
      .where((error) => error is Error_Decode)
      .map((error) => error.field0);

  late Stream<String> networkError = _error
      .where((error) => error is Error_NetworkStream)
      .map((error) => error.field0);

  /// Returns `true` if the player is playing.
  Future<bool> get isPlaying async {
    final state = await _player.playbackState();
    return state == PlaybackState.play || state == PlaybackState.preloadPlayed;
  }

  /// Returns `true` if the player has a file preloaded.
  Future<bool> get hasPreloaded => _player.isPreloaded();

  /// **[shouldNormalizeVolume]** Whether or not to normalize the volume
  /// of the playback. You can also change this by calling [normalizeVolume]
  /// when you desire. The normalization uses the `EbuR128` standard and
  /// it normalizes to `-14 LUFS`.
  SimpleAudio({
    bool shouldNormalizeVolume = false,
  }) {
    _player.normalizeVolume(shouldNormalize: shouldNormalizeVolume);
  }

  /// Initialize [SimpleAudio]. This should only be done once in the `main` method.
  /// This method should be awaited to make sure that the player is created
  /// before the app runs.
  static Future<void> init() async {
    // Disposes of any old players and starts the Rust code from a fresh state.
    PlayerWrapper.dispose(bridge: api);
    _player = await PlayerWrapper.newPlayerWrapper(bridge: api);
  }

  /// Open a new file for playback.
  ///
  /// **[path]**: The path of the file. For example, `https://my-domain.com/file.mp3`
  /// or `/path/to/file.mp3`.
  ///
  /// **[autoplay]** Whether or not to immediately start playing the file when opened.
  ///
  /// Throws [Error_Open] if the file couldn't be opened.
  Future<void> open(String path, {bool autoplay = true}) =>
      _player.open(path: path, autoplay: autoplay);

  /// Plays the opened file. If the player was paused,
  /// this resumes it.
  Future<void> play() => _player.play();

  /// Pauses playback of the opened file.
  Future<void> pause() => _player.pause();

  /// Stops playback of the opened file. Another file will have
  /// to be opened to start playback.
  Future<void> stop() => _player.stop();

  /// Set the player in a looping mode where
  /// the opened file will be replayed when it is done.
  Future<void> loopPlayback(bool shouldLoop) =>
      _player.loopPlayback(shouldLoop: shouldLoop);

  /// Set the volume of the playback.
  ///
  /// **[volume]** The volume from `0.0` to `1.0`.
  Future<void> setVolume(double volume) => _player.setVolume(volume: volume);

  /// Seek to a certain spot in the file and play from there.
  ///
  /// **[position]** The position to seek to.
  Future<void> seek(Duration position) => _player.seek(position: position);

  /// Whether or not to normalize the volume
  /// of the playback. The normalization uses the `EbuR128` standard and
  /// it normalizes to `-14 LUFS`.
  Future<void> normalizeVolume(bool shouldNormalize) =>
      _player.normalizeVolume(shouldNormalize: shouldNormalize);

  /// Preloads a file or network resource for reading and playing.
  /// The preloaded file is automatically played when the current file is finished playing.
  ///
  /// Use this method if you want gapless playback. It reduces
  /// the time spent loading between tracks (especially important
  /// for streaming network files).
  ///
  /// Throws [Error_Preload] if the file couldn't be preloaded.
  Future<void> preload(String path) => _player.preload(path: path);

  /// Plays the preloaded file.
  Future<void> playPreload() => _player.playPreload();

  /// Clears the preloaded file so that it doesn't play when the current file is finished.
  Future<void> clearPreload() => _player.clearPreload();
}
