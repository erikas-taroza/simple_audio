export 'audio_service.dart';

class Metadata {
  final String? title;
  final String? artist;
  final String? album;
  final String? artUri;

  const Metadata({
    this.title,
    this.artist,
    this.album,
    this.artUri,
  });
}
