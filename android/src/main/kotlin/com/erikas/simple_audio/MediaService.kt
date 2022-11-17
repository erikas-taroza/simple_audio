package com.erikas.simple_audio

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import android.content.Intent
import android.graphics.BitmapFactory
import android.net.Uri
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
import java.net.URL

private const val CHANNEL_ID:String = "SimpleAudio::Notification"
private const val NOTIFICATION_ID:Int = 777

class MediaService : MediaBrowserServiceCompat()
{
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
        return NotificationCompat.Builder(baseContext, CHANNEL_ID).apply {
            val metadata = mediaSession!!.controller.metadata
            setContentTitle(metadata.getText(METADATA_KEY_TITLE))
            setContentText(metadata.getText(METADATA_KEY_ARTIST))
            setSubText(metadata.getText(METADATA_KEY_ALBUM))

            val artUrl = metadata.getString(METADATA_KEY_ART_URI)
            if(artUrl != null && artUrl.isNotEmpty()) {
                val uri = Uri.parse(artUrl)

                val bitmap = if(uri.scheme == "http" || uri.scheme == "https")
                {
                    val imageBytes = URL(artUrl).readBytes()
                    BitmapFactory.decodeByteArray(imageBytes, 0, imageBytes.count())
                }
                else { BitmapFactory.decodeFile(artUrl) }
                setLargeIcon(bitmap)
            }

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

        val channel = NotificationChannel(CHANNEL_ID, "SimpleAudio", NotificationManager.IMPORTANCE_HIGH)
        getNotificationManager().createNotificationChannel(channel)

        startForeground(NOTIFICATION_ID, buildNotification())
    }

    fun updateMediaSession(
        title:String?,
        artist:String?,
        album:String?,
        artUrl:String?
    )
    {
        playbackState.setState(PlaybackStateCompat.STATE_PLAYING, 50, 1.0f)

        val metadataBuilder = MediaMetadataCompat.Builder().apply {
            putText(METADATA_KEY_TITLE, title ?: "Unknown Title")
            putText(METADATA_KEY_ARTIST, artist ?: "Unknown Artist")
            putText(METADATA_KEY_ALBUM, album ?: "Unknown Album")
            putLong(METADATA_KEY_DURATION, 100)
            putString(METADATA_KEY_ART_URI, artUrl ?: "")
        }

        mediaSession!!.apply {
            setPlaybackState(playbackState.build())
            setMetadata(metadataBuilder.build())
        }

        getNotificationManager().notify(NOTIFICATION_ID, buildNotification())
    }
}