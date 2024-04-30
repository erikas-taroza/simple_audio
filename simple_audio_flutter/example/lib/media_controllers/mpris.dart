import 'dart:io';

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
      player.seek(Duration(microseconds: position));

  /*
    In order for this to work, you need the following code in `my_application.cc` in the linux folder:
    
    static void my_application_activate(GApplication* application) {
      GList* windows = gtk_application_get_windows(GTK_APPLICATION(application));
      if(windows)
      {
        gtk_window_present_with_time(
          GTK_WINDOW(windows->data),
          g_get_monotonic_time() / 1000
        );
        return;
      }
      // ...
    }


    MyApplication* my_application_new() {
      return MY_APPLICATION(g_object_new(my_application_get_type(),
        "application-id", APPLICATION_ID, "flags",
        G_APPLICATION_HANDLES_COMMAND_LINE | G_APPLICATION_HANDLES_OPEN,
        nullptr
      ));
    }
  */
  @override
  Future<void> onRaise() async => Process.run(Platform.resolvedExecutable, []);

  @override
  void setMetadata(mc.Metadata m, Duration duration) {
    metadata = Metadata(
      trackId: "/${m.hashCode}",
      trackTitle: m.title,
      trackArtist: [m.artist],
      artUrl: m.artUri,
      trackLength: duration,
    );
  }

  @override
  void onPlaybackStarted() {
    playbackStatus = PlaybackStatus.playing;
    playbackRate = 1.0;
    updatePosition(Duration.zero);
  }

  @override
  void onPlaybackStateChanged(PlaybackState state, Duration position) {
    playbackStatus = {
      PlaybackState.play: PlaybackStatus.playing,
      PlaybackState.preloadPlayed: PlaybackStatus.playing,
      PlaybackState.pause: PlaybackStatus.paused,
      PlaybackState.stop: PlaybackStatus.stopped,
      PlaybackState.done: PlaybackStatus.stopped,
    }[state]!;

    updatePosition(position);
  }
}
