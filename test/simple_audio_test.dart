import 'package:flutter_test/flutter_test.dart';
import 'package:simple_audio/simple_audio.dart';
import 'package:simple_audio/simple_audio_platform_interface.dart';
import 'package:simple_audio/simple_audio_method_channel.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

class MockSimpleAudioPlatform
    with MockPlatformInterfaceMixin
    implements SimpleAudioPlatform {

  @override
  Future<String?> getPlatformVersion() => Future.value('42');
}

void main() {
  final SimpleAudioPlatform initialPlatform = SimpleAudioPlatform.instance;

  test('$MethodChannelSimpleAudio is the default instance', () {
    expect(initialPlatform, isInstanceOf<MethodChannelSimpleAudio>());
  });

  test('getPlatformVersion', () async {
    SimpleAudio simpleAudioPlugin = SimpleAudio();
    MockSimpleAudioPlatform fakePlatform = MockSimpleAudioPlatform();
    SimpleAudioPlatform.instance = fakePlatform;

    expect(await simpleAudioPlugin.getPlatformVersion(), '42');
  });
}
