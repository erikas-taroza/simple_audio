import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'simple_audio_method_channel.dart';

abstract class SimpleAudioPlatform extends PlatformInterface {
  /// Constructs a SimpleAudioPlatform.
  SimpleAudioPlatform() : super(token: _token);

  static final Object _token = Object();

  static SimpleAudioPlatform _instance = MethodChannelSimpleAudio();

  /// The default instance of [SimpleAudioPlatform] to use.
  ///
  /// Defaults to [MethodChannelSimpleAudio].
  static SimpleAudioPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [SimpleAudioPlatform] when
  /// they register themselves.
  static set instance(SimpleAudioPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}
