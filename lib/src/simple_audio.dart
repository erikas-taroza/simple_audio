export 'bridge_definitions.dart' show ProgressState, Metadata;

import 'dart:io';

import 'package:flutter/services.dart';

import './ffi.dart';

late final Player _player;

class SimpleAudio
{
    static MethodChannel methodChannel = const MethodChannel("simple_audio");

    late Stream<PlaybackState> playbackStateStream = api.playbackStateStreamStaticMethodPlayer()
        .map((event) => PlaybackState.values[event]).asBroadcastStream(); // Map the int event to a dart enum.
    late Stream<ProgressState> progressStateStream = api.progressStateStreamStaticMethodPlayer().asBroadcastStream();

    Future<bool> get isPlaying => _player.isPlaying();
    Future<ProgressState> get _progress => _player.getProgress();

    /// Initialize [SimpleAudio]. Should be done only once in the `main` method.
    /// 
    /// **[mprisName]** The name of the MPRIS metadata handler. The name has to follow these requirements:
    /// - Be less than or equal to 255 characters in length.
    /// - Cannot start with a number.
    /// - Can only contain these characters: "[A-Z][a-z][0-9]_"
    /// 
    /// MPRIS is a D-Bus interface for controlling media players. See: https://wiki.archlinux.org/title/MPRIS
    /// 
    /// **[onPreviousRequested]** Callback for when the user wants to skip to the previous media
    /// (via a media notification).
    /// 
    /// **[onNextRequested]** Callback for when the user wants to skip to the next media
    /// (via a media notification).
    static Future<void> init({
        String mprisName = "SimpleAudio",
        void Function()? onPreviousRequested,
        void Function()? onNextRequested
    }) async
    {
        _player = await api.newStaticMethodPlayer(
            mprisName: mprisName,
            hwnd: Platform.isWindows ? getHWND() : null
        );

        api.metadataCallbackStreamStaticMethodPlayer().listen((event) {
            if(!event) { onPreviousRequested?.call(); }
            else { onNextRequested?.call(); }
        });

        methodChannel.setMethodCallHandler((call) async {
            switch(call.method)
            {
                case "seek":
                    _player.seek(seconds: call.arguments);
                    break;
            }
        });
    }

    Future<void> open(String path, [bool autoplay = true]) async
    {
        await api.openMethodPlayer(that: _player, path: path, autoplay: autoplay);

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

        return await api.playMethodPlayer(that: _player);
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

        return await api.pauseMethodPlayer(that: _player);
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

        return await api.stopMethodPlayer(that: _player);
    }

    Future<void> setVolume(double volume) async
    {
        return api.setVolumeMethodPlayer(that: _player, volume: volume);
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

        return api.seekMethodPlayer(that: _player, seconds: seconds);
    }

    Future<void> setMetadata(Metadata metadata) async
    {
        if(Platform.isLinux || Platform.isWindows)
        {
            return api.setMetadataMethodPlayer(that: _player, metadata: metadata);
        }
        else if(Platform.isAndroid)
        {
            // Wait for a valid duration.
            while((await _progress).duration == 0) { continue; }

            return await methodChannel.invokeMethod("setMetadata", {
                "title": metadata.title,
                "artist": metadata.artist,
                "album": metadata.album,
                "artUrl": metadata.artUrl,
                "duration": (await _progress).duration
            });
        }
    }
}

enum PlaybackState
{
    play,
    pause,
    done
}