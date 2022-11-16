package com.erikas.simple_audio

import androidx.annotation.NonNull

import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.MethodChannel.MethodCallHandler
import io.flutter.plugin.common.MethodChannel.Result

// Initialized in the user's MainActivity.kt file.
lateinit var mediaService:MediaService

/** SimpleAudioPlugin */
class SimpleAudioPlugin: FlutterPlugin, MethodCallHandler
{
    /// The MethodChannel that will the communication between Flutter and native Android
    ///
    /// This local reference serves to register the plugin with the Flutter Engine and unregister it
    /// when the Flutter Engine is detached from the Activity
    private lateinit var channel : MethodChannel

    override fun onAttachedToEngine(@NonNull flutterPluginBinding: FlutterPlugin.FlutterPluginBinding)
    {
        channel = MethodChannel(flutterPluginBinding.binaryMessenger, "simple_audio")
        channel.setMethodCallHandler(this)
    }

    override fun onDetachedFromEngine(@NonNull binding: FlutterPlugin.FlutterPluginBinding)
    {
        channel.setMethodCallHandler(null)
    }

    override fun onMethodCall(@NonNull call: MethodCall, @NonNull result: Result)
    {
        if(call.method == "updateMediaSession")
        {
            mediaService.updateMediaSession()
        }
        else { result.notImplemented() }
    }
}