package com.erikas.simple_audio

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.graphics.BitmapFactory
import android.net.Uri
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
import java.net.URL

private const val CHANNEL_ID:String = "SimpleAudio::Notification"
private const val NOTIFICATION_ID:Int = 777

class MediaService : MediaBrowserServiceCompat()
{
    private var mediaSession:MediaSessionCompat? = null
    private lateinit var playbackState:PlaybackStateCompat.Builder

    var isPlaying:Boolean = false

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
        val actions:ArrayList<NotificationCompat.Action> = arrayListOf()

        actions.add(NotificationCompat.Action(
            R.drawable.rewind,
            "Rewind",
            PendingIntent.getBroadcast(this,
                0,
                Intent(this, MediaServiceReceiver::class.java).apply { action = ACTION_REWIND },
                PendingIntent.FLAG_IMMUTABLE
            )
        ))

        actions.add(NotificationCompat.Action(
            R.drawable.skip_prev,
            "Skip to Previous",
            PendingIntent.getBroadcast(this,
                0,
                Intent(this, MediaServiceReceiver::class.java).apply { action = ACTION_PREV },
                PendingIntent.FLAG_IMMUTABLE
            )
        ))

        if(!isPlaying)
        {
            actions.add(NotificationCompat.Action(
                R.drawable.play,
                "Play",
                PendingIntent.getBroadcast(this,
                    0,
                    Intent(this, MediaServiceReceiver::class.java).apply { action = ACTION_PLAY },
                    PendingIntent.FLAG_IMMUTABLE
                )
            ))
        }
        else
        {
            actions.add(NotificationCompat.Action(
                R.drawable.pause,
                "Pause",
                PendingIntent.getBroadcast(this,
                    0,
                    Intent(this, MediaServiceReceiver::class.java).apply { action = ACTION_PAUSE },
                    PendingIntent.FLAG_IMMUTABLE
                )
            ))
        }

        actions.add(NotificationCompat.Action(
            R.drawable.skip_next,
            "Skip to Next",
            PendingIntent.getBroadcast(this,
                0,
                Intent(this, MediaServiceReceiver::class.java).apply { action = ACTION_NEXT },
                PendingIntent.FLAG_IMMUTABLE
            )
        ))

        actions.add(NotificationCompat.Action(
            R.drawable.fast_forward,
            "Fast Forward",
            PendingIntent.getBroadcast(this,
                0,
                Intent(this, MediaServiceReceiver::class.java).apply { action = ACTION_FAST_FORWARD },
                PendingIntent.FLAG_IMMUTABLE
            )
        ))

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

            for(action in actions)
            { addAction(action) }

            // Required for showing the media style notification.
            setSmallIcon(R.mipmap.ic_launcher)
            setVisibility(NotificationCompat.VISIBILITY_PUBLIC)
            setStyle(androidx.media.app.NotificationCompat.MediaStyle()
                .setMediaSession(mediaSession!!.sessionToken))
        }.build()
    }

    // This is called when the service is created in the user's MainActivity.kt
    @RequiresApi(Build.VERSION_CODES.O)
    override fun onCreate()
    {
        super.onCreate()
        mediaService = this

        // Create the media session which defines the
        // controls and registers the callbacks.
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
                .setState(PlaybackStateCompat.STATE_NONE, 0, 1.0f)

            val metadataBuilder = MediaMetadataCompat.Builder()

            setPlaybackState(playbackState.build())
            setMetadata(metadataBuilder.build())
            setCallback(MediaServiceCallback())
            setSessionToken(sessionToken)
        }

        // A channel needs to be registered. Otherwise, the notification will not display
        // and an error will be thrown.
        val channel = NotificationChannel(CHANNEL_ID, "SimpleAudio", NotificationManager.IMPORTANCE_HIGH)
        getNotificationManager().createNotificationChannel(channel)

        // Start this service as a foreground service by using the notification.
        startForeground(NOTIFICATION_ID, buildNotification())
    }

    fun kill()
    {
        mediaSession!!.isActive = false
        mediaSession!!.release()
        stopForeground(true)
        stopSelf()
    }

    fun setMetadata(
        title:String?,
        artist:String?,
        album:String?,
        artUrl:String?,
        duration:Int?
    )
    {
        val metadataBuilder = MediaMetadataCompat.Builder().apply {
            putText(METADATA_KEY_TITLE, title ?: "Unknown Title")
            putText(METADATA_KEY_ARTIST, artist ?: "Unknown Artist")
            putText(METADATA_KEY_ALBUM, album ?: "Unknown Album")
            putString(METADATA_KEY_ART_URI, artUrl ?: "")
            if(duration != null) putLong(METADATA_KEY_DURATION, duration.toLong() * 1000)
        }

        mediaSession!!.setMetadata(metadataBuilder.build())
        getNotificationManager().notify(NOTIFICATION_ID, buildNotification())
    }

    // See enum type PlaybackState in simple_audio.dart for integer values.
    fun setPlaybackState(state:Int?, position:Int?)
    {
        val translatedState = when(state) {
            0 -> {
                isPlaying = true
                PlaybackStateCompat.STATE_PLAYING
            }
            1 -> {
                isPlaying = false
                PlaybackStateCompat.STATE_PAUSED
            }
            2 -> {
                isPlaying = false
                PlaybackStateCompat.STATE_STOPPED
            }
            else -> {
                isPlaying = false
                PlaybackStateCompat.STATE_NONE
            }
        }

        playbackState.setState(translatedState, (position?.toLong() ?: 0) * 1000, 1.0f)
        mediaSession!!.setPlaybackState(playbackState.build())
        getNotificationManager().notify(NOTIFICATION_ID, buildNotification())
    }
}