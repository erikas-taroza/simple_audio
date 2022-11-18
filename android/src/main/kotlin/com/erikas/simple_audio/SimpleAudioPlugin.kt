package com.erikas.simple_audio

import android.os.StrictMode
import androidx.annotation.NonNull

import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.MethodChannel.MethodCallHandler
import io.flutter.plugin.common.MethodChannel.Result

// Initialized in the user's MainActivity.kt file.
lateinit var mediaService:MediaService

/// The MethodChannel that will the communication between Flutter and native Android
///
/// This local reference serves to register the plugin with the Flutter Engine and unregister it
/// when the Flutter Engine is detached from the Activity
lateinit var channel:MethodChannel

/** SimpleAudioPlugin */
class SimpleAudioPlugin: FlutterPlugin, MethodCallHandler
{
    override fun onAttachedToEngine(@NonNull flutterPluginBinding: FlutterPlugin.FlutterPluginBinding)
    {
        channel = MethodChannel(flutterPluginBinding.binaryMessenger, "simple_audio")
        channel.setMethodCallHandler(this)

        // Allows for HTTP requests to be made to get images
        // for the media notification.
        val policy: StrictMode.ThreadPolicy = StrictMode.ThreadPolicy.Builder().permitNetwork().build()
        StrictMode.setThreadPolicy(policy)
    }

    override fun onDetachedFromEngine(@NonNull binding: FlutterPlugin.FlutterPluginBinding)
    {
        channel.invokeMethod("stop", null)
        mediaService.kill()

        channel.setMethodCallHandler(null)
    }

    override fun onMethodCall(@NonNull call: MethodCall, @NonNull result: Result) {
        when (call.method) {
            "setMetadata" -> {
                mediaService.setMetadata(
                    call.argument("title"),
                    call.argument("artist"),
                    call.argument("album"),
                    call.argument("artUrl"),
                    call.argument("duration")
                )
            }
            "setPlaybackState" -> {
                mediaService.setPlaybackState(
                    call.argument("state"),
                    call.argument("position")
                )
            }
            else -> { result.notImplemented() }
        }
    }
}