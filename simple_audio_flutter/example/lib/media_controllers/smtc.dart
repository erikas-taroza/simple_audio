import 'package:flutter/foundation.dart';
import 'package:simple_audio/simple_audio.dart';
import 'package:smtc_windows/smtc_windows.dart';

import 'media_controllers.dart';

class Smtc extends MediaController {
  Smtc(this.player)
      : smtc = SMTCWindows(
          config: const SMTCConfig(
            playEnabled: true,
            pauseEnabled: true,
            nextEnabled: true,
            prevEnabled: true,
            stopEnabled: false,
            fastForwardEnabled: false,
            rewindEnabled: false,
          ),
        ) {
    smtc.buttonPressStream.listen((event) {
      switch (event) {
        case PressedButton.play:
          player.play();
          break;
        case PressedButton.pause:
          player.pause();
          break;
        case PressedButton.next:
          debugPrint("Skip Next");
          break;
        case PressedButton.previous:
          debugPrint("Skip Previous");
          break;
        default:
          break;
      }
    });
  }

  final SimpleAudio player;
  final SMTCWindows smtc;

  @override
  void setMetadata(Metadata m, Duration duration) {
    smtc.updateMetadata(
      MusicMetadata(
        title: m.title,
        artist: m.artist,
        album: m.album,
        thumbnail: m.artUri,
      ),
    );

    smtc.updateTimeline(
      PlaybackTimeline(
        startTimeMs: 0,
        positionMs: 0,
        endTimeMs: duration.inMilliseconds,
        minSeekTimeMs: 0,
        maxSeekTimeMs: duration.inMilliseconds,
      ),
    );
  }

  @override
  void onPlaybackStarted() {
    smtc.setPlaybackStatus(PlaybackStatus.Playing);
  }

  @override
  void onPlaybackStateChanged(PlaybackState state, Duration position) {
    PlaybackStatus playbackStatus = {
      PlaybackState.play: PlaybackStatus.Playing,
      PlaybackState.preloadPlayed: PlaybackStatus.Playing,
      PlaybackState.pause: PlaybackStatus.Paused,
      PlaybackState.stop: PlaybackStatus.Stopped,
      PlaybackState.done: PlaybackStatus.Stopped,
    }[state]!;

    smtc.setPlaybackStatus(playbackStatus);
    smtc.setPosition(position);
  }
}
