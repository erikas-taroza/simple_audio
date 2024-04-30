import 'package:simple_audio/simple_audio.dart';

export 'audio_service.dart';
export 'mpris.dart';
export 'smtc.dart';

abstract class MediaController {
  void setMetadata(Metadata m, Duration duration);
  void onPlaybackStarted();
  void onPlaybackStateChanged(PlaybackState state, Duration position);
}

class Metadata {
  final String title;
  final String artist;
  final String album;
  final String artUri;

  const Metadata({
    this.title = "Unknown Title",
    this.artist = "Unknown Artist",
    this.album = "Unknown Album",
    this.artUri = "",
  });
}
