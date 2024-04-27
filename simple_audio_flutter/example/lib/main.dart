import 'dart:io';
import 'dart:math';

import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:file_picker/file_picker.dart';
import 'package:simple_audio/simple_audio.dart';

import 'media_controllers/media_controllers.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  await SimpleAudio.init();
  final SimpleAudio player = SimpleAudio(shouldNormalizeVolume: false);

  MediaController mediaController;
  if (Platform.isAndroid || Platform.isIOS || Platform.isMacOS) {
    mediaController = AudioServiceController(player);
  } else if (Platform.isLinux) {
    mediaController = Mpris(player);
  } else if (Platform.isWindows) {
    mediaController = Smtc(player);
  } else {
    throw UnsupportedError("Platform is not supported");
  }

  runApp(MyApp(player, mediaController));
}

class MyApp extends StatefulWidget {
  const MyApp(this.player, this.mediaController, {super.key});

  final SimpleAudio player;
  final MediaController mediaController;

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  SimpleAudio get player => widget.player;
  MediaController get mediaController => widget.mediaController;

  PlaybackState playbackState = PlaybackState.stop;
  bool get isPlaying =>
      playbackState == PlaybackState.play ||
      playbackState == PlaybackState.preloadPlayed;

  bool get isMuted => volume == 0;
  double trueVolume = 1;
  double volume = 1;
  bool normalize = false;
  bool loop = false;

  Duration position = Duration.zero;
  Duration duration = Duration.zero;

  String convertSecondsToReadableString(Duration seconds) {
    int m = seconds.inSeconds ~/ 60;
    int s = seconds.inSeconds % 60;

    return "$m:${s > 9 ? s : "0$s"}";
  }

  Future<String> pickFile() async {
    FilePickerResult? file = await FilePicker.platform
        .pickFiles(dialogTitle: "Pick file to play.", type: FileType.audio);

    final PlatformFile pickedFile = file!.files.single;
    return pickedFile.path!;
  }

  @override
  void initState() {
    super.initState();

    player.playbackStarted.listen((duration) {
      // Update your media controller metadata here.
      mediaController.onPlaybackStarted();
      mediaController.setMetadata(
        const Metadata(
          title: "Title",
          artist: "Artist",
          album: "Album",
          artUri: "https://picsum.photos/200",
        ),
        duration,
      );
      debugPrint("Playback Started: $duration seconds");
    });

    player.decodeError.listen((error) {
      debugPrint("Decode Error: $error");
      player.stop();
    });

    player.networkError.listen((error) {
      debugPrint("Network Stream Error: $error");
      player.stop();
    });

    player.playbackState.listen((event) async {
      setState(() => playbackState = event);
      mediaController.onPlaybackStateChanged(event, position);
    });

    player.progressState.listen((event) {
      setState(() {
        position = event.position;
        duration = event.duration;
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Simple Audio Example'),
        ),
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              if (Platform.isAndroid || Platform.isIOS) ...{
                Builder(
                  builder: (context) => ElevatedButton(
                    child: const Text("Get Storage Perms"),
                    onPressed: () async {
                      PermissionStatus status =
                          await Permission.storage.request();

                      if (!context.mounted) return;
                      ScaffoldMessenger.of(context).showSnackBar(
                        SnackBar(
                          content: Text("Storage Permissions: ${status.name}"),
                        ),
                      );
                    },
                  ),
                ),
              },
              const SizedBox(height: 5),
              ElevatedButton(
                child: const Text("Open File"),
                onPressed: () async {
                  String path = await pickFile();

                  await player.stop();
                  await player.open(path);
                },
              ),
              const SizedBox(height: 10),
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  ElevatedButton(
                    child: const Text("Preload File"),
                    onPressed: () async {
                      String path = await pickFile();
                      await player.preload(path);
                    },
                  ),
                  const SizedBox(width: 5),
                  ElevatedButton(
                    child: const Text("Play Preload"),
                    onPressed: () async {
                      if (!await player.hasPreloaded) {
                        debugPrint("No preloaded file to play!");
                        return;
                      }

                      debugPrint("Playing preloaded file.");
                      await player.stop();
                      await player.playPreload();
                    },
                  ),
                  const SizedBox(width: 5),
                  ElevatedButton(
                    child: const Text("Clear Preload"),
                    onPressed: () async {
                      if (!await player.hasPreloaded) {
                        debugPrint("No preloaded file to clear!");
                        return;
                      }

                      debugPrint("Cleared preloaded file.");
                      await player.clearPreload();
                    },
                  ),
                ],
              ),
              const SizedBox(height: 20),

              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  // Stop button.
                  CircleButton(
                    size: 35,
                    onPressed: playbackState != PlaybackState.done
                        ? player.stop
                        : null,
                    child: const Icon(Icons.stop, color: Colors.white),
                  ),
                  const SizedBox(width: 10),

                  // Play/pause button.
                  CircleButton(
                    size: 40,
                    onPressed: () {
                      if (isPlaying) {
                        player.pause();
                      } else {
                        player.play();
                      }
                    },
                    child: Icon(
                      isPlaying
                          ? Icons.pause_rounded
                          : Icons.play_arrow_rounded,
                      color: Colors.white,
                    ),
                  ),
                  const SizedBox(width: 10),

                  // Toggle mute button.
                  CircleButton(
                    size: 35,
                    onPressed: () {
                      if (!isMuted) {
                        player.setVolume(0);
                        setState(() => volume = 0);
                      } else {
                        player.setVolume(trueVolume);
                        setState(() => volume = trueVolume);
                      }
                    },
                    child: Icon(
                      isMuted ? Icons.volume_off : Icons.volume_up,
                      color: Colors.white,
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 10),

              // Volume control.
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  const Text("Volume: "),
                  SizedBox(
                    width: 200,
                    child: Slider(
                      value: volume,
                      onChanged: (value) {
                        setState(() {
                          volume = value;
                          trueVolume = value;
                        });
                        player.setVolume(value);
                      },
                    ),
                  ),
                ],
              ),

              // Toggle looping playback.
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Checkbox(
                    value: loop,
                    onChanged: (value) {
                      setState(() => loop = value!);
                      player.loopPlayback(loop);
                    },
                  ),
                  const Text("Loop Playback"),
                ],
              ),

              // Toggle volume normalization
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Checkbox(
                    value: normalize,
                    onChanged: (value) {
                      setState(() => normalize = value!);
                      player.normalizeVolume(normalize);
                    },
                  ),
                  const Text("Normalize Volume"),
                ],
              ),

              // Progress bar with time.
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 10),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Text(convertSecondsToReadableString(position)),
                    Flexible(
                      child: ConstrainedBox(
                        constraints: const BoxConstraints(maxWidth: 450),
                        child: Slider(
                          value: min(
                            position.inSeconds.toDouble(),
                            duration.inSeconds.toDouble(),
                          ),
                          max: duration.inSeconds.toDouble(),
                          onChanged: (value) {
                            player.seek(Duration(seconds: value.floor()));
                          },
                        ),
                      ),
                    ),
                    Text(convertSecondsToReadableString(duration)),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class CircleButton extends StatelessWidget {
  const CircleButton({
    required this.onPressed,
    required this.child,
    this.size = 35,
    this.color = Colors.blue,
    super.key,
  });

  final void Function()? onPressed;
  final Widget child;
  final double size;
  final Color color;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: size,
      width: size,
      child: ClipOval(
        child: Material(
          color: color,
          child: InkWell(
            canRequestFocus: false,
            onTap: onPressed,
            child: child,
          ),
        ),
      ),
    );
  }
}
