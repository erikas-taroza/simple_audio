package com.erikas.simple_audio

import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import android.os.Build
import android.os.Bundle
import android.support.v4.media.MediaBrowserCompat
import android.support.v4.media.MediaMetadataCompat
import android.support.v4.media.MediaMetadataCompat.*
import android.support.v4.media.session.MediaSessionCompat
import android.support.v4.media.session.PlaybackStateCompat
import androidx.annotation.RequiresApi
import androidx.core.app.NotificationCompat
import androidx.media.MediaBrowserServiceCompat

class MediaService : MediaBrowserServiceCompat()
{
    private val channelId:String = "SimpleAudio::Notification"

    private var mediaSession:MediaSessionCompat? = null
    private lateinit var stateBuilder:PlaybackStateCompat.Builder
    private lateinit var metadataBuilder:MediaMetadataCompat.Builder

    private val callback = object:MediaSessionCompat.Callback() {
        override fun onPlay() {
            println("Play")
            super.onPlay()
        }

        override fun onPause() {
            println("Pause")
            super.onPause()
        }
    }

    override fun onGetRoot(
        clientPackageName: String,
        clientUid: Int,
        rootHints: Bundle?
    ): BrowserRoot? {
        return BrowserRoot("root", null)
    }

    override fun onLoadChildren(
        parentId: String,
        result: Result<MutableList<MediaBrowserCompat.MediaItem>>
    ) {
        result.sendResult(null)
    }

    @RequiresApi(Build.VERSION_CODES.O)
    override fun onCreate()
    {
        super.onCreate()

        mediaSession = MediaSessionCompat(baseContext, "SimpleAudio").apply {
            stateBuilder = PlaybackStateCompat.Builder()
                .setActions(PlaybackStateCompat.ACTION_PLAY or PlaybackStateCompat.ACTION_PAUSE)

            metadataBuilder = MediaMetadataCompat.Builder()
            metadataBuilder.putText(METADATA_KEY_TITLE, "Test")
            metadataBuilder.putText(METADATA_KEY_ARTIST, "TEST TEST")
            metadataBuilder.putLong(METADATA_KEY_DURATION, 100)

            setPlaybackState(stateBuilder.build())
            setMetadata(metadataBuilder.build())
            setCallback(callback)
            setSessionToken(sessionToken)
        }

        val channel = NotificationChannel(channelId, "SimpleAudio", NotificationManager.IMPORTANCE_HIGH)

        val manager = getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        manager.createNotificationChannel(channel)

        val notification = NotificationCompat.Builder(baseContext, channelId).apply {
            val metadata = mediaSession!!.controller.metadata
            setContentTitle(metadata.getText(METADATA_KEY_TITLE))
            setContentText(metadata.getText(METADATA_KEY_ARTIST))
            setContentIntent(mediaSession!!.controller.sessionActivity)

            // Required for showing the media style notification.
            setSmallIcon(R.mipmap.ic_launcher)
            setVisibility(NotificationCompat.VISIBILITY_PUBLIC)
            setStyle(androidx.media.app.NotificationCompat.MediaStyle()
                .setMediaSession(mediaSession!!.sessionToken))
        }

        startForeground(777, notification.build())
    }
}