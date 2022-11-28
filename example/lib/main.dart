import 'dart:io';

import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:file_picker/file_picker.dart';
import 'package:simple_audio/simple_audio.dart';

void main() async
{
    SimpleAudio.init(
        actions: [
            NotificationActions.playPause,
            NotificationActions.skipNext,
            NotificationActions.skipPrev
        ]
    );
    runApp(const MyApp());
}

class MyApp extends StatefulWidget
{
    const MyApp({super.key});

    @override
    State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp>
{
    final SimpleAudio player = SimpleAudio(
        onNextCallback: () => debugPrint("Next"),
        onPreviousCallback: () => debugPrint("Prev")
    );

    PlaybackState playbackState = PlaybackState.done;
    bool get isPlaying => playbackState == PlaybackState.play;

    bool get isMuted => volume == 0;
    double trueVolume = 1;
    double volume = 1;
    
    double position = 0;
    double duration = 0;

    String convertSecondsToReadableString(int seconds)
    {
        int m = seconds ~/ 60;
        int s = seconds % 60;
        
        return "$m:${s > 9 ? s : "0$s"}";
    }

    @override
    void initState()
    {
        super.initState();

        player.playbackStateStream.listen((event) {
            setState(() => playbackState = event);
        });

        player.progressStateStream.listen((event) {
            setState(() {
                position = event.position.toDouble();
                duration = event.duration.toDouble();
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
                            if(Platform.isAndroid || Platform.isIOS) ...{
                                Builder(
                                    builder: (context) => ElevatedButton(
                                        child: const Text("Get Storage Perms"),
                                        onPressed: () async {
                                            PermissionStatus status = await Permission.storage.request();

                                            if(!mounted) return;
                                            ScaffoldMessenger.of(context).showSnackBar(SnackBar(
                                                content: Text("Storage Permissions: ${status.name}"),
                                            ));
                                        },
                                    ),
                                ),
                            },
                            const SizedBox(height: 5),
                            ElevatedButton(
                                child: const Text("Open File"),
                                onPressed: () async {
                                    // FilePickerResult? file = await FilePicker.platform.pickFiles(
                                    //     dialogTitle: "Pick file to play.",
                                    //     //type: FileType.audio
                                    // );

                                    // if(file == null) return;

                                    // final PlatformFile pickedFile = file.files.single;

                                    await player.setMetadata(Metadata(
                                        title: "Title",
                                        artist: "Artist",
                                        album: "Album",
                                        artUri: "https://imgs.search.brave.com/6O9qUMKrlM5XUEZ1yyVELLuLW3kyjbFkBFLeIbDMHwo/rs:fit:700:700:1/g:ce/aHR0cHM6Ly93d3cu/Zmx1d2VsLmNvbS9t/ZWRpYS9jYXRhbG9n/L3Byb2R1Y3QvY2Fj/aGUvNjU0ODUwM2Fh/ODMzZTY4ZmZkYzQ1/Yjc1YmU2ZGEyZTUv/Yy9vL2NvcHBlcl9p/bWFnZV9hLmpwZw"
                                    ));
                                    await player.open(r"https://file-examples.com/storage/fe5a16a4f363851959d0e45/2017/11/file_example_MP3_2MG.mp3");
                                },
                            ),
                            const SizedBox(height: 20),

                            Row(
                                mainAxisAlignment: MainAxisAlignment.center,
                                children: [
                                    // Stop button.
                                    CircleButton(
                                        size: 35,
                                        onPressed: playbackState != PlaybackState.done ? player.stop : null,
                                        child: const Icon(
                                            Icons.stop, 
                                            color: Colors.white
                                        ),
                                    ),
                                    const SizedBox(width: 10),

                                    // Play/pause button.
                                    CircleButton(
                                        size: 40,
                                        onPressed: () {
                                            if(isPlaying) { player.pause(); }
                                            else { player.play(); }
                                        },
                                        child: Icon(
                                            isPlaying ? Icons.pause_rounded : Icons.play_arrow_rounded, 
                                            color: Colors.white
                                        ),
                                    ),
                                    const SizedBox(width: 10),
                                    
                                    // Toggle mute button.
                                    CircleButton(
                                        size: 35,
                                        onPressed: () {
                                            if(!isMuted)
                                            {
                                                player.setVolume(0);
                                                setState(() => volume = 0);
                                            }
                                            else
                                            {
                                                player.setVolume(trueVolume);
                                                setState(() => volume = trueVolume);
                                            }
                                        },
                                        child: Icon(
                                            isMuted ? Icons.volume_off : Icons.volume_up, 
                                            color: Colors.white
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
                                            }
                                        ),
                                    ),
                                ],
                            ),
                            
                            // Progress bar with time.
                            Padding(
                                padding: const EdgeInsets.symmetric(horizontal: 10),
                                child: Row(
                                    mainAxisAlignment: MainAxisAlignment.center,
                                    children: [
                                        Text(convertSecondsToReadableString(position.floor())),
                            
                                        Flexible(
                                            child: ConstrainedBox(
                                                constraints: const BoxConstraints(maxWidth: 450),
                                                child: Slider(
                                                    value: position,
                                                    max: duration,
                                                    onChanged: (value) {
                                                        player.seek(value.floor());
                                                    },
                                                ),
                                            ),
                                        ),
                            
                                        Text(convertSecondsToReadableString(duration.floor())),
                                    ],
                                ),
                            )
                        ],
                    )
                ),
            ),
        );
    }
}

class CircleButton extends StatelessWidget
{
    const CircleButton(
    {
        required this.onPressed,
        required this.child,
        this.size = 35,
        this.color = Colors.blue,
        super.key
    });

    final void Function()? onPressed;
    final Widget child;
    final double size;
    final Color color;

    @override
    Widget build(BuildContext context)
    {
        return SizedBox(
            height: size, width: size,
            child: ClipOval(
                child: Material(
                    color: color,
                    child: InkWell(
                        canRequestFocus: false,
                        onTap: onPressed,
                        child: child
                    )
                ),
            ),
        );
    }
}