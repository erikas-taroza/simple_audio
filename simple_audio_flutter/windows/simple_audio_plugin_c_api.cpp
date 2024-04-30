#include "include/simple_audio/simple_audio_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "simple_audio_plugin.h"

void SimpleAudioPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  simple_audio::SimpleAudioPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
