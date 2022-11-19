package com.erikas.simple_audio

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent

private const val ACTION_HEADER:String = "com.erikas.simple_audio.action."
const val ACTION_PLAY = ACTION_HEADER + "play"
const val ACTION_PAUSE = ACTION_HEADER + "pause"
const val ACTION_NEXT = ACTION_HEADER + "next"
const val ACTION_PREV = ACTION_HEADER + "prev"
const val ACTION_FAST_FORWARD = ACTION_HEADER + "fast_forward"
const val ACTION_REWIND = ACTION_HEADER + "rewind"

/**
 * Handles broadcast messages from notification button presses.
 * The buttons are configured in MediaService.kt
 */
class MediaServiceReceiver:BroadcastReceiver()
{
    override fun onReceive(context: Context?, intent: Intent?)
    {
        when(intent?.action)
        {
            ACTION_PLAY -> MediaServiceCallback().onPlay()
            ACTION_PAUSE -> MediaServiceCallback().onPause()
            ACTION_NEXT -> MediaServiceCallback().onSkipToNext()
            ACTION_PREV -> MediaServiceCallback().onSkipToPrevious()
            ACTION_FAST_FORWARD -> MediaServiceCallback().onFastForward()
            ACTION_REWIND -> MediaServiceCallback().onRewind()
        }
    }
}