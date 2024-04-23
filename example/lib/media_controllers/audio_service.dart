import 'package:audio_service/audio_service.dart';
import 'package:simple_audio/simple_audio.dart' as sa;

import 'media_controllers.dart';

class AudioServiceController extends BaseAudioHandler with SeekHandler {
  AudioServiceController(this.player);
  final sa.SimpleAudio player;

  static Future<AudioServiceController> init(sa.SimpleAudio player) {
    return AudioService.init(
      builder: () => AudioServiceController(player),
      config: const AudioServiceConfig(
        androidNotificationChannelId: "com.erikastaroza.simple_audio.example",
        androidNotificationChannelName: "Simple Audio",
        androidNotificationOngoing: true,
        fastForwardInterval: Duration(seconds: 5),
        rewindInterval: Duration(seconds: 5),
      ),
    );
  }

  @override
  Future<void> play() => player.play();

  @override
  Future<void> pause() => player.pause();

  @override
  Future<void> seek(Duration position) => player.seek(position.inSeconds);

  void setMetadata(Metadata m, int duration) {
    mediaItem.add(
      MediaItem(
        id: m.hashCode.toString(),
        title: m.title ?? "Unknown Title",
        artist: m.artist,
        album: m.album,
        artUri: Uri.tryParse(m.artUri ?? ""),
        duration: Duration(seconds: duration),
      ),
    );
  }

  void onPlaybackStarted() {
    playbackState.add(
      PlaybackState(
        controls: [
          MediaControl.rewind,
          MediaControl.skipToPrevious,
          MediaControl.pause,
          MediaControl.skipToNext,
          MediaControl.fastForward,
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

  void onPlaybackStateChanged(sa.PlaybackState state, int position) async {
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
          MediaControl.rewind,
          MediaControl.skipToPrevious,
          isPlaying ? MediaControl.pause : MediaControl.play,
          MediaControl.skipToNext,
          MediaControl.fastForward,
        ],
        updatePosition: Duration(seconds: position),
      ),
    );
  }
}
