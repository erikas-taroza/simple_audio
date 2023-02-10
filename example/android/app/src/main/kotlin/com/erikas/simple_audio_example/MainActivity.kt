package com.erikas.simple_audio_example

import android.content.Intent
import android.os.Bundle
import com.erikas.simple_audio.SimpleAudioService
import com.erikas.simple_audio.stopSimpleAudioService
import com.erikas.simple_audio.notificationClickedIntent
import io.flutter.embedding.android.FlutterActivity

class MainActivity: FlutterActivity() {
    override fun onCreate(savedInstanceState: Bundle?)
    {
        super.onCreate(savedInstanceState)

        notificationClickedIntent = Intent(applicationContext, MainActivity::class.java)
        startService(Intent(applicationContext, SimpleAudioService::class.java))
    }

    override fun onStop() {
        super.onStop()
        stopSimpleAudioService()
    }
}
