// This file is a part of simple_audio
// Copyright (c) 2022-2023 Erikas Taroza <erikastaroza@gmail.com>
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of 
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program.
// If not, see <https://www.gnu.org/licenses/>.

package com.erikas.simple_audio

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.os.StrictMode
import android.support.v4.media.session.PlaybackStateCompat

import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.embedding.engine.plugins.activity.ActivityAware
import io.flutter.embedding.engine.plugins.activity.ActivityPluginBinding
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.MethodChannel.MethodCallHandler
import io.flutter.plugin.common.MethodChannel.Result

var simpleAudioService: SimpleAudioService? = null

/// The MethodChannel that will the communication between Flutter and native Android
///
/// This local reference serves to register the plugin with the Flutter Engine and unregister it
/// when the Flutter Engine is detached from the Activity
var channel: MethodChannel? = null

/** SimpleAudioPlugin */
class SimpleAudioPlugin: FlutterPlugin, MethodCallHandler, ActivityAware
{
    private var appContext: Context? = null
    private var appActivity: Activity? = null

    override fun onAttachedToEngine(flutterPluginBinding: FlutterPlugin.FlutterPluginBinding)
    {
        channel = MethodChannel(flutterPluginBinding.binaryMessenger, "simple_audio")
        channel?.setMethodCallHandler(this)

        // Allows for HTTP requests to be made to get images
        // for the media notification.
        val policy: StrictMode.ThreadPolicy = StrictMode.ThreadPolicy.Builder().permitNetwork().build()
        StrictMode.setThreadPolicy(policy)

        appContext = flutterPluginBinding.applicationContext
    }

    override fun onDetachedFromEngine(binding: FlutterPlugin.FlutterPluginBinding)
    {
        channel?.setMethodCallHandler(null)
        channel = null
        appContext = null
        appActivity = null
    }

    override fun onAttachedToActivity(binding: ActivityPluginBinding) {
        appActivity = binding.activity
    }

    override fun onDetachedFromActivityForConfigChanges() { }
    override fun onReattachedToActivityForConfigChanges(binding: ActivityPluginBinding) { }
    override fun onDetachedFromActivity() { }

    override fun onMethodCall(call: MethodCall, result: Result) {
        when (call.method) {
            "init" -> {
                if(call.argument<Boolean>("showMediaNotification") == false) return

                val actions = ArrayList<PlaybackActions>()
                for(action in call.argument<List<Int>>("actions")!!)
                { actions.add(PlaybackActions.values()[action]) }

                val intent = Intent(appContext, SimpleAudioService::class.java).apply {
                    putExtra("iconPath", call.argument<String>("icon")!!)
                    putExtra("playbackActions", actions)
                    putExtra("compactPlaybackActions", call.argument<List<Int>>("compactActions")!!.toIntArray())
                    putExtra("notificationClickedIntent", Intent(appContext, appActivity!!::class.java))
                }

                appContext?.startService(intent)
            }
            "dispose" -> {
                simpleAudioService?.exitProcessOnDestroy = false
                simpleAudioService?.kill()
            }
            "setMetadata" -> {
                simpleAudioService?.setMetadata(
                    call.argument("title"),
                    call.argument("artist"),
                    call.argument("album"),
                    call.argument("artUri"),
                    call.argument("artBytes"),
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

enum class PlaybackActions(val data: PlaybackActionsMapping)
{
    Rewind(PlaybackActionsMapping(R.drawable.rewind, "Rewind", PlaybackStateCompat.ACTION_REWIND, ACTION_REWIND)),
    SkipPrev(PlaybackActionsMapping(R.drawable.skip_prev, "Skip Prev", PlaybackStateCompat.ACTION_SKIP_TO_PREVIOUS, ACTION_PREV)),
    PlayPause(PlaybackActionsMapping(0, "PlayPause", PlaybackStateCompat.ACTION_PLAY_PAUSE, "")),
    SkipNext(PlaybackActionsMapping(R.drawable.skip_next, "Skip Next", PlaybackStateCompat.ACTION_SKIP_TO_NEXT, ACTION_NEXT)),
    FastForward(PlaybackActionsMapping(R.drawable.fast_forward, "Fast Forward", PlaybackStateCompat.ACTION_FAST_FORWARD, ACTION_FAST_FORWARD))
}

class PlaybackActionsMapping(
    val icon: Int,
    val name: String,
    // The action used by MediaSession.
    val sessionAction: Long,
    // The action used by the notification for SimpleAudioReceiver.
    val notificationAction: String
)