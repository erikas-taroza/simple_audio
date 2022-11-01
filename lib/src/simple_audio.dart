export 'bridge_definitions.dart' show ProgressState;

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
    /// [wakelock] If set to true, this will prevent Android and iOS devices from stopping playback
    /// due to inactivity. The lock activates when opening or playing something
    /// but closes when the player is paused or stopped.
    static void init({bool wakelock = true})
    {
        _wakelock = (Platform.isAndroid || Platform.isIOS) && wakelock;
        _player = Player(bridge: api, dummy: 0);
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
}

enum PlaybackState
{
    play,
    pause,
    done
}