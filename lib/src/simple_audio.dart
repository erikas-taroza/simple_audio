import './ffi.dart';

class SimpleAudio
{
    final Player _player = Player(bridge: api, dummy: 0);
    late Stream<bool> playbackStateStream = api.playbackStateStreamStaticMethodPlayer();

    Future<bool> get isPlaying => _player.isPlaying();

    Future<void> open(String path) async
    {
        return await api.openMethodPlayer(that: _player, path: path);
    }

    Future<void> play() async
    {
        return await api.playMethodPlayer(that: _player);
    }

    Future<void> pause() async
    {
        return await api.pauseMethodPlayer(that: _player);
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