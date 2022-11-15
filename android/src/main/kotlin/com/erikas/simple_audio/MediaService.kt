package com.erikas.simple_audio

import android.os.Bundle
import android.support.v4.media.MediaBrowserCompat
import android.support.v4.media.MediaMetadataCompat
import android.support.v4.media.MediaMetadataCompat.METADATA_KEY_TITLE
import android.support.v4.media.session.MediaSessionCompat
import android.support.v4.media.session.PlaybackStateCompat
import androidx.media.MediaBrowserServiceCompat

class MediaService : MediaBrowserServiceCompat()
{
    private var mediaSession:MediaSessionCompat? = null
    private lateinit var stateBuilder:PlaybackStateCompat.Builder
    private lateinit var metadataBuilder:MediaMetadataCompat.Builder

    private val callback = object:MediaSessionCompat.Callback() {
        override fun onPlay() {
            println("Play")
            super.onPlay()
        }
    }

    override fun onGetRoot(
        clientPackageName: String,
        clientUid: Int,
        rootHints: Bundle?
    ): BrowserRoot? {
        return MediaBrowserServiceCompat.BrowserRoot("root", null)
    }

    override fun onLoadChildren(
        parentId: String,
        result: Result<MutableList<MediaBrowserCompat.MediaItem>>
    ) {
        result.sendResult(null)
    }

    override fun onCreate()
    {
        super.onCreate()

        mediaSession = MediaSessionCompat(baseContext, "SimpleAudio").apply {
            stateBuilder = PlaybackStateCompat.Builder().setActions(PlaybackStateCompat.ACTION_PLAY or PlaybackStateCompat.ACTION_PAUSE)
            metadataBuilder = MediaMetadataCompat.Builder().putString(METADATA_KEY_TITLE, "Test")
            setPlaybackState(stateBuilder.build())
            setMetadata(metadataBuilder.build())

            setCallback(callback)

            setSessionToken(sessionToken)

            isActive = true
        }
    }
}