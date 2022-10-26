import 'package:flutter/material.dart';
import 'package:simple_audio/simple_audio.dart';

void main()
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

    @override
    Widget build(BuildContext context) {
        return MaterialApp(
            home: Scaffold(
                appBar: AppBar(
                    title: const Text('Plugin example app'),
                ),
                body: Center(
                    child: Column(
                        children: [
                            ElevatedButton(
                                child: const Text("Open"),
                                onPressed: () async => {
                                    await player.open("/home/erikas/Music/test.mp3")
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
                                onPressed: () async => await player.play(),
                            ),
                            ElevatedButton(
                                child: const Text("Pause"),
                                onPressed: () async => await player.pause(),
                            )
                        ],
                    )
                ),
            ),
        );
    }
}
