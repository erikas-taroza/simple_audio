export 'bridge_definitions.dart' show ProgressState, Metadata;

import 'dart:io';

import 'package:flutter/services.dart';

import './ffi.dart';

late final Player _player;

class SimpleAudio
{
    static MethodChannel methodChannel = const MethodChannel("simple_audio");

    late Stream<PlaybackState> playbackStateStream = Player.playbackStateStream(bridge: api)
        .map((event) => PlaybackState.values[event]).asBroadcastStream(); // Map the int event to a dart enum.
    late Stream<ProgressState> progressStateStream = Player.progressStateStream(bridge: api).asBroadcastStream();

    Future<bool> get isPlaying => _player.isPlaying();
    Future<ProgressState> get _progress => _player.getProgress();

    void Function()? onPreviousCallback;
    void Function()? onNextCallback;

    /// **[onPreviousCallback]** Callback for when the user wants to skip to the previous media
    /// (via a media notification).
    /// 
    /// **[onNextCallback]** Callback for when the user wants to skip to the next media
    /// (via a media notification).
    SimpleAudio({
        this.onPreviousCallback,
        this.onNextCallback
    })
    {
        Player.metadataCallbackStream(bridge: api).listen((event) {
            if(!event) { onPreviousCallback?.call(); }
            else { onPreviousCallback?.call(); }
        });
        
        methodChannel.setMethodCallHandler((call) async {
            switch(call.method)
            {
                case "play":
                    play();
                    break;
                case "pause":
                    pause();
                    break;
                case "stop":
                    stop();
                    break;
                case "next":
                    onNextCallback?.call();
                    break;
                case "previous":
                    onPreviousCallback?.call();
                    break;
                case "seekRelative":
                    int position = (await _progress).position;
                    int seekTo = call.arguments as bool ?
                        position + 10
                        : (position - 10).clamp(0, double.maxFinite).toInt();

                    seek(seekTo);
                    break;
                case "seek":
                    seek(call.arguments);
                    break;
            }
        });
    }

    /// Initialize [SimpleAudio]. Should be done only once in the `main` method.
    /// 
    /// **[mprisName]** The name of the MPRIS metadata handler. The name has to follow these requirements:
    /// - Be less than or equal to 255 characters in length.
    /// - Cannot start with a number.
    /// - Can only contain these characters: "[A-Z][a-z][0-9]_"
    /// 
    /// MPRIS is a D-Bus interface for controlling media players. See: https://wiki.archlinux.org/title/MPRIS
    static Future<void> init({
        String mprisName = "SimpleAudio"
    }) async
    {
        _player = await Player.newPlayer(
            bridge: api,
            mprisName: mprisName,
            hwnd: Platform.isWindows ? getHWND() : null
        );
    }

    Future<void> open(String path, [bool autoplay = true]) async
    {
        await _player.open(path: path, autoplay: autoplay);

        if(Platform.isAndroid && autoplay)
        {
            methodChannel.invokeMethod("setPlaybackState", {
                "state": PlaybackState.play.index,
                "position": 0
            });
        }
    }

    Future<void> play() async
    {
        if(Platform.isAndroid)
        {
            methodChannel.invokeMethod("setPlaybackState", {
                "state": PlaybackState.play.index,
                "position": (await _progress).position
            });
        }

        return await _player.play();
    }

    Future<void> pause() async
    {
        if(Platform.isAndroid)
        {
            methodChannel.invokeMethod("setPlaybackState", {
                "state": PlaybackState.pause.index,
                "position": (await _progress).position
            });
        }

        return await _player.pause();
    }

    Future<void> stop() async
    {
        if(Platform.isAndroid)
        {
            methodChannel.invokeMethod("setPlaybackState", {
                "state": -1,
                "position": 0
            });
        }

        return await _player.stop();
    }

    Future<void> setVolume(double volume) async
    {
        return await _player.setVolume(volume: volume);
    }

    Future<void> seek(int seconds) async
    {
        if(Platform.isAndroid)
        {
            methodChannel.invokeMethod("setPlaybackState", {
                "state": (await isPlaying ? PlaybackState.play : PlaybackState.pause).index,
                "position": seconds
            });
        }

        return await _player.seek(seconds: seconds);
    }

    Future<void> setMetadata(Metadata metadata) async
    {
        if(Platform.isLinux || Platform.isWindows)
        {
            return await _player.setMetadata(metadata: metadata);
        }
        else if(Platform.isAndroid)
        {
            // Prevent users from awaiting this method
            // and blocking their program infintely
            Future<void> _() async
            {
                // Wait for a valid duration.
                while((await _progress).duration == 0) { continue; }

                await methodChannel.invokeMethod("setMetadata", {
                    "title": metadata.title,
                    "artist": metadata.artist,
                    "album": metadata.album,
                    "artUrl": metadata.artUrl,
                    "duration": (await _progress).duration
                });
            }
            _();
        }
    }
}

enum PlaybackState
{
    play,
    pause,
    done
}