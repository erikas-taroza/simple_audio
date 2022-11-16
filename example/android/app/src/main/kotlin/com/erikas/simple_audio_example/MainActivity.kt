package com.erikas.simple_audio_example

import android.content.Intent
import android.os.Bundle
import com.erikas.simple_audio.MediaService
import io.flutter.embedding.android.FlutterActivity

class MainActivity: FlutterActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        startService(Intent(applicationContext, MediaService::class.java))
    }
}
