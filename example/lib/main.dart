import 'package:flutter/material.dart';
import 'package:simple_audio/simple_audio.dart';

final SimpleAudio player = SimpleAudio();

void main() async
{
    await player.init();

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
                                    await player.open("https://rr4---sn-a5msenek.googlevideo.com/videoplayback?expire=1666838220&ei=bJpZY56qKM7YkgaH3qW4CQ&ip=2600%3A8801%3A4760%3A600%3A0%3A0%3A0%3A8c6a&id=o-AKyyWJDZYIWw9uDCPXfEFVr_DzAg4PKBq2GDk9o1SnIX&itag=140&source=youtube&requiressl=yes&mh=xS&mm=31%2C26&mn=sn-a5msenek%2Csn-n4v7sney&ms=au%2Conr&mv=m&mvi=4&pl=34&gcr=us&initcwndbps=1720000&vprv=1&mime=audio%2Fmp4&ns=5vuKLl7gP8jEvnfMZuRTTcEI&gir=yes&clen=2745189&dur=169.411&lmt=1592541047399525&mt=1666816375&fvip=2&keepalive=yes&fexp=24001373%2C24007246&c=WEB&txp=2311222&n=01BrDplHLNJ5GA&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cgcr%2Cvprv%2Cmime%2Cns%2Cgir%2Cclen%2Cdur%2Clmt&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRAIgUHoPRVGxXoR6AZJeL4mTZVQjH0o3pH3n_CGESe4ipfQCIFWRJJz3TaIPy2N5hKvsJtAVKtJzVQeUfVSw4jq0ww-I&sig=AOq0QJ8wRgIhAKon8z_nntZg0Whe_BkiRJHyGPNYVzUP-6SYyfy9w1cjAiEA5S0HgrXrMACGUMRLuRV52ZDnIcdOsg7MbwOghYb3opA=")
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
                            )
                        ],
                    )
                ),
            ),
        );
    }
}
