export 'bridge_definitions.dart' show ProgressState, Metadata;

import 'dart:io';

import 'package:wakelock/wakelock.dart';

import './ffi.dart';

late final Player _player;

class SimpleAudio
{
    late Stream<PlaybackState> playbackStateStream = api.playbackStateStreamStaticMethodPlayer()
        .map((event) => PlaybackState.values[event]).asBroadcastStream(); // Map the int event to a dart enum.
    late Stream<ProgressState> progressStateStream = api.progressStateStreamStaticMethodPlayer().asBroadcastStream();

    Future<bool> get isPlaying => _player.isPlaying();

    static bool _wakelock = false;

    /// Initialize [SimpleAudio]. Should be done only once in the `main` method.
    /// 
    /// **[wakelock]** If set to true, this will prevent Android and iOS devices from stopping playback
    /// due to inactivity. The lock activates when opening or playing something
    /// but closes when the player is paused or stopped.
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
        bool wakelock = true,
        String mprisName = "SimpleAudio",
        void Function()? onPreviousRequested,
        void Function()? onNextRequested
    }) async
    {
        _wakelock = (Platform.isAndroid || Platform.isIOS) && wakelock;
        _player = await api.newStaticMethodPlayer(
            mprisName: mprisName,
            hwnd: Platform.isWindows ? getHWND() : null
        );

        api.metadataCallbackStreamStaticMethodPlayer().listen((event) {
            if(!event) { onPreviousRequested?.call(); }
            else { onNextRequested?.call(); }
        });
    }

    Future<void> open(String path, [bool autoplay = true]) async
    {
        if(_wakelock) { Wakelock.enable(); }

        return await api.openMethodPlayer(that: _player, path: path, autoplay: autoplay);
    }

    Future<void> play() async
    {
        if(_wakelock) { Wakelock.enable(); }

        return await api.playMethodPlayer(that: _player);
    }

    Future<void> pause() async
    {
        if(_wakelock) { Wakelock.disable(); }

        return await api.pauseMethodPlayer(that: _player);
    }

    Future<void> stop() async
    {
        if(_wakelock) { Wakelock.disable(); }

        return await api.stopMethodPlayer(that: _player);
    }

    Future<void> setVolume(double volume) async
    {
        return api.setVolumeMethodPlayer(that: _player, volume: volume);
    }

    Future<void> seek(int seconds) async
    {
        return api.seekMethodPlayer(that: _player, seconds: seconds);
    }

    Future<void> setMetadata(Metadata metadata) async
    {
        if(Platform.isLinux)
        {
            return api.setMetadataMethodPlayer(that: _player, metadata: metadata);
        }
    }
}

enum PlaybackState
{
    play,
    pause,
    done
}