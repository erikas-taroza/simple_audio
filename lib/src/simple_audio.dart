import './ffi.dart';

class SimpleAudio
{
    final Player _player = Player(bridge: api, dummy: 0);
    late Stream<PlaybackState> playbackStateStream = api.playbackStateStreamStaticMethodPlayer()
        .map((event) => PlaybackState.values[event]); // Map the int event to a dart enum.
    late Stream<ProgressState> progressStateStream = api.progressStateStreamStaticMethodPlayer();

    Future<bool> get isPlaying => _player.isPlaying();

    Future<void> open(String path, [bool autoplay = true]) async
    {
        return await api.openMethodPlayer(that: _player, path: path, autoplay: autoplay);
    }

    Future<void> play() async
    {
        return await api.playMethodPlayer(that: _player);
    }

    Future<void> pause() async
    {
        return await api.pauseMethodPlayer(that: _player);
    }

    Future<void> stop() async
    {
        return await api.stopMethodPlayer(that: _player);
    }

    Future<void> setVolume(double volume) async
    {
        return api.setVolumeMethodPlayer(that: _player, volume: volume);
    }

    Future<void> seek(double seconds) async
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