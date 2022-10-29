import 'dart:io';

import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:simple_audio/simple_audio.dart';

void main() async
{
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
    double volume = 1;

    @override
    Widget build(BuildContext context) {
        return MaterialApp(
            home: Scaffold(
                appBar: AppBar(
                    title: const Text('Plugin example app'),
                ),
                body: Center(
                    child: Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                            if(Platform.isAndroid || Platform.isIOS) ...{
                                ElevatedButton(
                                    child: const Text("Perms"),
                                    onPressed: () async {
                                        PermissionStatus status = await Permission.storage.request();
                                        print(status);
                                    },
                                )
                            },
                            ElevatedButton(
                                child: const Text("Open"),
                                onPressed: () async {
                                    await player.open("/home/erikas/Music/test.mp3");
                                    //await player.open("/storage/emulated/0/Music/test.mp3");
                                },
                            ),
                            StreamBuilder(
                                stream: player.playbackStateStream,
                                builder: (_, data) {
                                    return Text("Is playing: ${data.data}");
                                },
                            ),
                            ElevatedButton(
                                child: const Text("Play"),
                                onPressed: () async {
                                    await player.play();
                                    print(await player.isPlaying);
                                },
                            ),
                            ElevatedButton(
                                child: const Text("Pause"),
                                onPressed: () async {
                                    await player.pause();
                                    print(await player.isPlaying);
                                }
                            ),
                            SizedBox(
                                width: 200,
                                child: Slider(
                                    value: volume,
                                    onChanged: (value) {
                                        setState(() => volume = value);
                                        player.setVolume(value);
                                    }
                                ),
                            ),
                            ElevatedButton(
                                child: const Text("Seek 20s"),
                                onPressed: () async {
                                    await player.seek(20);
                                }
                            ),
                        ],
                    )
                ),
            ),
        );
    }
}
