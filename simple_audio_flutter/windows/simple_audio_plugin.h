#ifndef FLUTTER_PLUGIN_SIMPLE_AUDIO_PLUGIN_H_
#define FLUTTER_PLUGIN_SIMPLE_AUDIO_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace simple_audio {

class SimpleAudioPlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  SimpleAudioPlugin();

  virtual ~SimpleAudioPlugin();

  // Disallow copy and assign.
  SimpleAudioPlugin(const SimpleAudioPlugin&) = delete;
  SimpleAudioPlugin& operator=(const SimpleAudioPlugin&) = delete;

 private:
  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace simple_audio

#endif  // FLUTTER_PLUGIN_SIMPLE_AUDIO_PLUGIN_H_
