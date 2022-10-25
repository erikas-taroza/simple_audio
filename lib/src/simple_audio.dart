import './ffi.dart';

class SimpleAudio
{
    SimpleAudio() : _player = Player(bridge: api, isPlaying: true);

    final Player _player;

    bool get isPlaying => _player.isPlaying;

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
}