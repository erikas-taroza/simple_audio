import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'simple_audio_platform_interface.dart';

/// An implementation of [SimpleAudioPlatform] that uses method channels.
class MethodChannelSimpleAudio extends SimpleAudioPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('simple_audio');

  @override
  Future<String?> getPlatformVersion() async {
    final version = await methodChannel.invokeMethod<String>('getPlatformVersion');
    return version;
  }
}
