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

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.support.v4.media.MediaBrowserCompat
import android.support.v4.media.MediaMetadataCompat
import android.support.v4.media.MediaMetadataCompat.*
import android.support.v4.media.session.MediaSessionCompat
import android.support.v4.media.session.PlaybackStateCompat
import androidx.core.app.NotificationCompat
import androidx.media.MediaBrowserServiceCompat
import java.net.URL

private const val CHANNEL_ID:String = "SimpleAudio::Notification"
private const val NOTIFICATION_ID:Int = 777

class SimpleAudioService : MediaBrowserServiceCompat()
{
    private var mediaSession:MediaSessionCompat? = null
    private lateinit var playbackState:PlaybackStateCompat.Builder

    // Set in the init callback in SimpleAudioPlugin.kt
    var iconPath:String = "mipmap/ic_launcher"
    var playbackActions:List<PlaybackActions> = PlaybackActions.values().toList()
    var compactPlaybackActions:List<Int> = listOf()

    var isPlaying:Boolean = false

    override fun onGetRoot(
        clientPackageName: String,
        clientUid: Int,
        rootHints: Bundle?
    ): BrowserRoot {
        return BrowserRoot("root", null)
    }

    override fun onLoadChildren(
        parentId: String,
        result: Result<MutableList<MediaBrowserCompat.MediaItem>>
    ) {
        result.sendResult(null)
    }

    // This is called when the service is created in the user's MainActivity.kt
    override fun onCreate()
    {
        super.onCreate()
        simpleAudioService = this
    }

    private fun getNotificationManager():NotificationManager
    { return getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager }

    private fun buildNotification():Notification
    {
        fun buildAction(icon:Int, name:String, action:String):NotificationCompat.Action
        {
            return NotificationCompat.Action(
                icon,
                name,
                PendingIntent.getBroadcast(this,
                    0,
                    Intent(this, SimpleAudioReceiver::class.java).apply { this.action = action },
                    if(Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) PendingIntent.FLAG_IMMUTABLE else 0
                )
            )
        }

        val actions:ArrayList<NotificationCompat.Action> = arrayListOf()

        for(action in playbackActions)
        {
            if(action == PlaybackActions.PlayPause)
            {
                actions.add(buildAction(
                    if(!isPlaying) R.drawable.play else R.drawable.pause,
                    if(!isPlaying) "Play" else "Pause",
                    if(!isPlaying) ACTION_PLAY else ACTION_PAUSE
                ))

                continue
            }

            actions.add(buildAction(
                action.data.icon,
                action.data.name,
                action.data.notificationAction
            ))
        }

        return NotificationCompat.Builder(baseContext, CHANNEL_ID).apply {
            val metadata = mediaSession!!.controller.metadata
            setContentTitle(metadata.getText(METADATA_KEY_TITLE))
            setContentText(metadata.getText(METADATA_KEY_ARTIST))
            setSubText(metadata.getText(METADATA_KEY_ALBUM))
            setOngoing(true)
            setContentIntent(mediaSession!!.controller.sessionActivity)

            val bitmap = metadata.getBitmap(METADATA_KEY_ART)
            if(bitmap != null)
            {
                // Handle rectangular images and scale them in
                // so that they are in a square format.
                if(bitmap.width > bitmap.height)
                { setLargeIcon(Bitmap.createScaledBitmap(bitmap, bitmap.height, bitmap.height, false)) }
                else { setLargeIcon(bitmap) }
            }

            for(action in actions)
            { addAction(action) }

            // Required for showing the media style notification.
            val split = iconPath.split("/")
            val type = split[0] // Ex: mipmap
            val name = split[1] // Ex: ic_launcher
            setSmallIcon(resources.getIdentifier(name, type, applicationContext.packageName))

            setVisibility(NotificationCompat.VISIBILITY_PUBLIC)
            setStyle(androidx.media.app.NotificationCompat.MediaStyle()
                .setMediaSession(mediaSession!!.sessionToken)
                .setShowActionsInCompactView(*compactPlaybackActions.toIntArray()))
        }.build()
    }

    fun init()
    {
        if(mediaSession != null) return

        // Create the media session which defines the
        // controls and registers the callbacks.
        mediaSession = MediaSessionCompat(baseContext, "SimpleAudio").apply {
            var actions:Long = 0
            for(action in playbackActions)
            { actions = actions.or(action.data.sessionAction) }

            playbackState = PlaybackStateCompat.Builder()
                .setActions(actions
                        // Defaults (ACTION_PLAY_PAUSE is added above via playbackActions)
                        or PlaybackStateCompat.ACTION_SEEK_TO
                        or PlaybackStateCompat.ACTION_PLAY
                        or PlaybackStateCompat.ACTION_PAUSE
                )
                .setState(PlaybackStateCompat.STATE_NONE, 0, 1.0f)

            val metadataBuilder = MediaMetadataCompat.Builder()

            setPlaybackState(playbackState.build())
            setMetadata(metadataBuilder.build())
            setCallback(SimpleAudioServiceCallback())
            setSessionToken(sessionToken)

            // This makes it so that when the notification is clicked,
            // the app will be opened. This is used when building the
            // notification.
            var flags = PendingIntent.FLAG_UPDATE_CURRENT
            if(Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) flags = flags.or(PendingIntent.FLAG_IMMUTABLE)

            setSessionActivity(PendingIntent.getActivity(
                applicationContext,
                0,
                notificationClickedIntent,
                flags
            ))
        }

        // A channel needs to be registered. Otherwise, the notification will not display
        // and an error will be thrown.
        if(Build.VERSION.SDK_INT >= Build.VERSION_CODES.O)
        {
            val channel = NotificationChannel(CHANNEL_ID, "SimpleAudio", NotificationManager.IMPORTANCE_LOW)
            getNotificationManager().createNotificationChannel(channel)
        }

        // Start this service as a foreground service by using the notification.
        startForeground(NOTIFICATION_ID, buildNotification())
    }

    fun kill()
    {
        mediaSession!!.isActive = false
        mediaSession!!.release()
        mediaSession = null
        stopForeground(true)
        stopSelf()
    }

    fun setMetadata(
        title:String?,
        artist:String?,
        album:String?,
        artUri:String?,
        artBytes:ByteArray?,
        duration:Int?
    )
    {
        // If a URI was received.
        val bitmap:Bitmap? = if(artUri != null) {
            val uri = Uri.parse(artUri)

            val bitmap = if(uri.scheme == "http" || uri.scheme == "https")
            {
                val imageBytes = URL(artUri).readBytes()
                BitmapFactory.decodeByteArray(imageBytes, 0, imageBytes.size)
            }
            else { BitmapFactory.decodeFile(artUri) }

            bitmap
        }
        // If bytes were received.
        else if(artBytes != null) {
            BitmapFactory.decodeByteArray(artBytes, 0, artBytes.size)
        }
        else { null }

        val metadataBuilder = MediaMetadataCompat.Builder().apply {
            putText(METADATA_KEY_TITLE, title ?: "Unknown Title")
            putText(METADATA_KEY_ARTIST, artist ?: "Unknown Artist")
            putText(METADATA_KEY_ALBUM, album ?: "Unknown Album")
            putBitmap(METADATA_KEY_ART, bitmap)
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