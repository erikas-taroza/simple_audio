import 'package:anni_mpris_service/anni_mpris_service.dart';
import 'package:flutter/foundation.dart';
import 'package:simple_audio/simple_audio.dart';

import './media_controllers.dart' as mc;

class Mpris extends MPRISService with mc.MediaController {
  Mpris(this.player)
      : super(
          "com.erikastaroza.SimpleAudio",
          identity: "Simple Audio",
          emitSeekedSignal: true,
          canPlay: true,
          canPause: true,
          canGoPrevious: true,
          canGoNext: true,
          canSeek: true,
          canRaise: true,
          supportLoopStatus: true,
        );

  final SimpleAudio player;

  @override
  Future<void> onPlayPause() async {
    if (await player.isPlaying) {
      player.pause();
    } else {
      player.play();
    }
  }

  @override
  Future<void> onPlay() => player.play();

  @override
  Future<void> onPause() => player.pause();

  @override
  Future<void> onNext() async => debugPrint("Skip Next");

  @override
  Future<void> onPrevious() async => debugPrint("Skip Previous");

  @override
  Future<void> onSetPosition(String trackId, int position) =>
      player.seek(Duration(microseconds: position).inSeconds);

  @override
  void setMetadata(mc.Metadata m, int duration) {
    metadata = Metadata(
      trackId: "/${m.hashCode}",
      trackTitle: m.title,
      trackArtist: [m.artist],
      artUrl: m.artUri,
      trackLength: Duration(seconds: duration),
    );
  }

  @override
  void onPlaybackStarted() {
    playbackStatus = PlaybackStatus.playing;
    playbackRate = 1.0;
    updatePosition(Duration.zero);
  }

  @override
  void onPlaybackStateChanged(PlaybackState state, int position) {
    playbackStatus = {
      PlaybackState.play: PlaybackStatus.playing,
      PlaybackState.preloadPlayed: PlaybackStatus.playing,
      PlaybackState.pause: PlaybackStatus.paused,
      PlaybackState.stop: PlaybackStatus.stopped,
      PlaybackState.done: PlaybackStatus.stopped,
    }[state]!;

    updatePosition(Duration(seconds: position));
  }
}
