// This file is a part of simple_audio
// Copyright (c) 2022 Erikas Taroza <erikastaroza@gmail.com>
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
 * The buttons are configured in SimpleAudioService.kt
 */
class SimpleAudioReceiver:BroadcastReceiver()
{
    override fun onReceive(context: Context?, intent: Intent?)
    {
        when(intent?.action)
        {
            ACTION_PLAY -> SimpleAudioServiceCallback().onPlay()
            ACTION_PAUSE -> SimpleAudioServiceCallback().onPause()
            ACTION_NEXT -> SimpleAudioServiceCallback().onSkipToNext()
            ACTION_PREV -> SimpleAudioServiceCallback().onSkipToPrevious()
            ACTION_FAST_FORWARD -> SimpleAudioServiceCallback().onFastForward()
            ACTION_REWIND -> SimpleAudioServiceCallback().onRewind()
        }
    }
}