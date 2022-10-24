
import 'simple_audio_platform_interface.dart';

class SimpleAudio {
  Future<String?> getPlatformVersion() {
    return SimpleAudioPlatform.instance.getPlatformVersion();
  }
}
