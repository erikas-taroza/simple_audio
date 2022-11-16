package com.erikas.simple_audio_example

import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.ServiceConnection
import android.os.Bundle
import android.os.IBinder
import com.erikas.simple_audio.MediaService
import com.erikas.simple_audio.mediaService
import io.flutter.embedding.android.FlutterActivity

class MainActivity: FlutterActivity() {
    private val serviceConnection = object:ServiceConnection
    {
        override fun onServiceConnected(name: ComponentName?, service: IBinder?) {
            val binder = service as MediaService.MediaServiceBinder
            mediaService = binder.getService()
        }

        override fun onServiceDisconnected(name: ComponentName?) { }
    }

    override fun onCreate(savedInstanceState: Bundle?)
    {
        super.onCreate(savedInstanceState)
        bindService(Intent(applicationContext, MediaService::class.java), serviceConnection, Context.BIND_AUTO_CREATE)
    }

    override fun onDestroy()
    {
        super.onDestroy()
        unbindService(serviceConnection)
    }
}
