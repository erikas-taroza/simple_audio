package com.erikas.simple_audio

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import android.content.Intent
import android.os.Binder
import android.os.Build
import android.os.Bundle
import android.os.IBinder
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
    private val notificationId:Int = 777

    private var mediaSession:MediaSessionCompat? = null
    private lateinit var playbackState:PlaybackStateCompat.Builder

    private val callback = object:MediaSessionCompat.Callback() {
        override fun onPlay() {
            println("Play")
            super.onPlay()
        }

        override fun onPause() {
            println("Pause")
            super.onPause()
        }

        override fun onStop() {
            stopSelf()
            super.onStop()
        }
    }

    // This binder allows us to call instance methods to this service.
    // Without this, calling MediaService().updateMediaSession will not work
    // because the mediaSession won't be initialized.
    inner class MediaServiceBinder:Binder()
    { fun getService():MediaService = this@MediaService }

    override fun onBind(intent: Intent?): IBinder? = MediaServiceBinder()

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

    private fun getNotificationManager():NotificationManager
    { return getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager }

    private fun buildNotification():Notification
    {
        return NotificationCompat.Builder(baseContext, channelId).apply {
            val metadata = mediaSession!!.controller.metadata
            setContentTitle(metadata.getText(METADATA_KEY_TITLE))
            setContentText(metadata.getText(METADATA_KEY_ARTIST))
            setSubText(metadata.getText(METADATA_KEY_ALBUM))

            // Required for showing the media style notification.
            setSmallIcon(R.mipmap.ic_launcher)
            setVisibility(NotificationCompat.VISIBILITY_PUBLIC)
            setStyle(androidx.media.app.NotificationCompat.MediaStyle()
                .setMediaSession(mediaSession!!.sessionToken))
        }.build()
    }

    @RequiresApi(Build.VERSION_CODES.O)
    override fun onCreate()
    {
        super.onCreate()

        mediaSession = MediaSessionCompat(baseContext, "SimpleAudio").apply {
            playbackState = PlaybackStateCompat.Builder()
                .setActions(
                    PlaybackStateCompat.ACTION_PLAY
                    or PlaybackStateCompat.ACTION_PAUSE
                    or PlaybackStateCompat.ACTION_PLAY_PAUSE
                    or PlaybackStateCompat.ACTION_SKIP_TO_NEXT
                    or PlaybackStateCompat.ACTION_SKIP_TO_PREVIOUS
                    or PlaybackStateCompat.ACTION_FAST_FORWARD
                    or PlaybackStateCompat.ACTION_REWIND
                    or PlaybackStateCompat.ACTION_SEEK_TO
                )
                .setState(PlaybackStateCompat.STATE_PLAYING, 0, 1.0f)

            val metadataBuilder = MediaMetadataCompat.Builder()

            setPlaybackState(playbackState.build())
            setMetadata(metadataBuilder.build())
            setCallback(callback)
            setSessionToken(sessionToken)
        }

        val channel = NotificationChannel(channelId, "SimpleAudio", NotificationManager.IMPORTANCE_HIGH)
        getNotificationManager().createNotificationChannel(channel)

        startForeground(notificationId, buildNotification())
    }

    fun updateMediaSession()
    {
        playbackState.setState(PlaybackStateCompat.STATE_PLAYING, 50, 1.0f)

        val metadataBuilder = MediaMetadataCompat.Builder().apply {
            putText(METADATA_KEY_TITLE, "Title")
            putText(METADATA_KEY_ARTIST, "Artist")
            putText(METADATA_KEY_ALBUM, "Album")
            putLong(METADATA_KEY_DURATION, 100)
        }

        mediaSession!!.apply {
            setPlaybackState(playbackState.build())
            setMetadata(metadataBuilder.build())
        }

        getNotificationManager().notify(notificationId, buildNotification())
    }
}