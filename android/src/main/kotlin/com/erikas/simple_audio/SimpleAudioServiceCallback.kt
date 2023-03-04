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

import android.os.Bundle
import android.support.v4.media.session.MediaSessionCompat

/**
 * Handles the common callbacks from the media
 * notification.
 */
class SimpleAudioServiceCallback: MediaSessionCompat.Callback()
{
    override fun onPlay() {
        if(simpleAudioService!!.isPlaying)
        {
            onPause()
            return
        }

        channel?.invokeMethod("play", null)
        super.onPlay()
    }

    override fun onPause() {
        if(!simpleAudioService!!.isPlaying)
        {
            onPlay()
            return
        }

        channel?.invokeMethod("pause", null)
        super.onPause()
    }

    override fun onStop() {
        simpleAudioService!!.kill()
        super.onStop()
    }

    override fun onSkipToNext() {
        channel?.invokeMethod("next", null)
        super.onSkipToNext()
    }

    override fun onSkipToPrevious() {
        channel?.invokeMethod("previous", null)
        super.onSkipToPrevious()
    }

    override fun onFastForward() {
        channel?.invokeMethod("seekRelative", true)
        super.onFastForward()
    }

    override fun onRewind() {
        channel?.invokeMethod("seekRelative", false)
        super.onRewind()
    }

    override fun onSeekTo(pos: Long) {
        channel?.invokeMethod("seek", pos / 1000)
        super.onSeekTo(pos)
    }

    override fun onCustomAction(action: String?, extras: Bundle?)
    {
        when(action)
        {
            PlaybackActions.FastForward.data.notificationAction -> onFastForward()
            PlaybackActions.Rewind.data.notificationAction -> onRewind()
            else -> {}
        }

        super.onCustomAction(action, extras)
    }
}