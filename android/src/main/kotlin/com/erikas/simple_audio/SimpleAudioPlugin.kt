package com.erikas.simple_audio

import android.content.Intent
import android.os.StrictMode
import android.support.v4.media.session.PlaybackStateCompat
import androidx.annotation.NonNull

import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.MethodChannel.MethodCallHandler
import io.flutter.plugin.common.MethodChannel.Result

// Initialized in the user's MainActivity.kt file.
var simpleAudioService:SimpleAudioService? = null
var notificationClickedIntent:Intent = Intent()

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
        simpleAudioService?.kill()
        simpleAudioService = null

        channel.setMethodCallHandler(null)
    }

    override fun onMethodCall(@NonNull call: MethodCall, @NonNull result: Result) {
        when (call.method) {
            "init" -> {
                if(call.argument<Boolean>("showMediaNotification") == false)
                {
                    simpleAudioService = null
                    return
                }

                simpleAudioService?.iconPath = call.argument("icon")!!

                val actions = ArrayList<PlaybackActions>()
                for(action in call.argument<List<Int>>("actions")!!)
                { actions.add(PlaybackActions.values()[action]) }

                simpleAudioService?.playbackActions = actions
                simpleAudioService?.compactPlaybackActions = call.argument<List<Int>>("compactActions")!!

                simpleAudioService?.init()
            }
            "setMetadata" -> {
                simpleAudioService?.setMetadata(
                    call.argument("title"),
                    call.argument("artist"),
                    call.argument("album"),
                    call.argument("artUri"),
                    call.argument("duration")
                )
            }
            "setPlaybackState" -> {
                simpleAudioService?.setPlaybackState(
                    call.argument("state"),
                    call.argument("position")
                )
            }
            else -> { result.notImplemented() }
        }
    }
}

enum class PlaybackActions(val data:PlaybackActionsMapping)
{
    Rewind(PlaybackActionsMapping(R.drawable.rewind, "Rewind", PlaybackStateCompat.ACTION_REWIND, ACTION_REWIND)),
    SkipPrev(PlaybackActionsMapping(R.drawable.skip_prev, "Skip Prev", PlaybackStateCompat.ACTION_SKIP_TO_PREVIOUS, ACTION_PREV)),
    PlayPause(PlaybackActionsMapping(0, "PlayPause", PlaybackStateCompat.ACTION_PLAY_PAUSE, "")),
    SkipNext(PlaybackActionsMapping(R.drawable.skip_next, "Skip Next", PlaybackStateCompat.ACTION_SKIP_TO_NEXT, ACTION_NEXT)),
    FastForward(PlaybackActionsMapping(R.drawable.fast_forward, "Fast Forward", PlaybackStateCompat.ACTION_FAST_FORWARD, ACTION_FAST_FORWARD))
}

class PlaybackActionsMapping(
    val icon: Int,
    val name:String,
    // The action used by MediaSession.
    val sessionAction:Long,
    // The action used by the notification for SimpleAudioReceiver.
    val notificationAction:String
)