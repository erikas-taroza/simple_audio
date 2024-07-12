import 'package:audio_service/audio_service.dart';
import 'package:flutter/foundation.dart';
import 'package:simple_audio/simple_audio.dart' as sa;

import 'media_controllers.dart';

class AudioServiceController extends BaseAudioHandler
    with SeekHandler, MediaController {
  AudioServiceController(this.player) {
    AudioService.init(
      builder: () => this,
      config: const AudioServiceConfig(
        androidNotificationChannelId: "com.erikastaroza.simple_audio.example",
        androidNotificationChannelName: "Simple Audio",
        androidNotificationOngoing: true,
        fastForwardInterval: Duration(seconds: 5),
        rewindInterval: Duration(seconds: 5),
      ),
    );
  }

  final sa.SimpleAudio player;

  @override
  Future<void> play() => player.play();

  @override
  Future<void> pause() => player.pause();

  @override
  Future<void> skipToNext() async => debugPrint("Skip Next");

  @override
  Future<void> skipToPrevious() async => debugPrint("Skip Previous");

  @override
  Future<void> seek(Duration position) => player.seek(position);

  @override
  void setMetadata(Metadata m, Duration duration) {
    mediaItem.add(
      MediaItem(
        id: m.hashCode.toString(),
        title: m.title,
        artist: m.artist,
        album: m.album,
        artUri: Uri.tryParse(m.artUri),
        duration: duration,
      ),
    );
  }

  @override
  void onPlaybackStarted() {
    playbackState.add(
      PlaybackState(
        controls: [
          MediaControl.skipToPrevious,
          MediaControl.pause,
          MediaControl.skipToNext,
        ],
        systemActions: const {
          MediaAction.seek,
          MediaAction.seekForward,
          MediaAction.seekBackward,
        },
        androidCompactActionIndices: const [1, 2, 3],
        processingState: AudioProcessingState.ready,
        playing: true,
        speed: 1,
        updatePosition: const Duration(seconds: 0),
      ),
    );
  }

  @override
  void onPlaybackStateChanged(sa.PlaybackState state, Duration position) {
    bool isPlaying = state == sa.PlaybackState.preloadPlayed ||
        state == sa.PlaybackState.play;

    playbackState.add(
      playbackState.value.copyWith(
        playing: isPlaying,
        processingState:
            state == sa.PlaybackState.stop || state == sa.PlaybackState.done
                ? AudioProcessingState.completed
                : AudioProcessingState.ready,
        controls: [
          MediaControl.skipToPrevious,
          isPlaying ? MediaControl.pause : MediaControl.play,
          MediaControl.skipToNext,
        ],
        updatePosition: position,
      ),
    );
  }
}
