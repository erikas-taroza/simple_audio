import 'dart:io';

import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:simple_audio/simple_audio.dart';

void main() async
{
    SimpleAudio.init(
        onNextRequested: () => debugPrint("Next"),
        onPreviousRequested: () => debugPrint("Prev")
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
    final SimpleAudio player = SimpleAudio();

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
                                ElevatedButton(
                                    child: const Text("Get Storage Perms"),
                                    onPressed: () async {
                                        PermissionStatus status = await Permission.storage.request();
                                        print(status);
                                    },
                                ),
                            },
                            const SizedBox(height: 5),
                            ElevatedButton(
                                child: const Text("Open File"),
                                onPressed: () async {
                                    //TODO: File picker.
                                    await player.open(r"C:\Users\Erikas Taroza\Music\1.mp3");
                                    await player.setMetadata(Metadata(
                                        title: "Test Media",
                                        artist: "Test Artist",
                                        album: "Test Album"
                                    ));
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
                            Row(
                                mainAxisAlignment: MainAxisAlignment.center,
                                children: [
                                    Text(convertSecondsToReadableString(position.floor())),

                                    SizedBox(
                                        width: 450,
                                        child: Slider(
                                            value: position,
                                            max: duration,
                                            onChanged: (value) {
                                                player.seek(value.floor());
                                            },
                                        ),
                                    ),

                                    Text(convertSecondsToReadableString(duration.floor())),
                                ],
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