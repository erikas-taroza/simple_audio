# Simple Audio

A cross-platform solution for playing audio in Flutter.

This project's goal is to be as simple as possible, meaning it offers the core functionality only (ex. play/pause).
It also aims to be stable and relatively bug free.

I created this plugin for my music app so that I don't have to use different packages for different
platforms (audio_service, dart_vlc). This made it hard to deal with bugs from different packages.

## Features
- Simple API
- Cross platform (Android, Linux, Windows, iOS, macOS)
- Media notifications on all platforms to access controls (iOS and macOS use the control center)
- Playback of local and online resources

## Usage
- Add this plugin as a dependency in ``pubspec.yaml``
- Call ``SimpleAudio.init()`` in ``main()`` and configure as you need.
- Instantiate a ``SimpleAudio`` object in your player controller.
- Use the APIs provided in the instantiated object.

To see a sample player project, please check the [example](https://github.com/erikas-taroza/simple_audio/tree/master/example).

## Setup
Some platform specific things have to be set up in order for this plugin to function properly.

### Windows/Linux
No setup is needed for these platforms.

### Android
You will need to edit 2 files located in the ``android/app/src/main`` directory.

#### AndroidManifest.xml
```xml
<application>
  <!-- Default Flutter stuff -->
  
  <!-- This is required if you want to be able to see the notification. -->
  <service
      android:name="com.erikas.simple_audio.SimpleAudioService">
  </service>

  <!-- This is required to interact with the notification buttons. -->
  <receiver
      android:name="com.erikas.simple_audio.SimpleAudioReceiver">
  </receiver>
  
</application>
```

#### MainActivity.kt
```kt
// ...
// Add these imports.
import android.content.Intent
import android.os.Bundle
import com.erikas.simple_audio.SimpleAudioService
import com.erikas.simple_audio.notificationClickedIntent

class MainActivity: FlutterActivity() {
    override fun onCreate(savedInstanceState: Bundle?)
    {
        super.onCreate(savedInstanceState)

        // This line is optional. What it does is open your app when the notification
        // is clicked.
        notificationClickedIntent = Intent(applicationContext, MainActivity::class.java)
        // This line starts the SimpleAudioService which starts a foreground service
        // and creates a new media session.
        startService(Intent(applicationContext, SimpleAudioService::class.java))
    }
}

```

### iOS
You will have to add a dependency to your Xcode project.

- Open ``Runner.xcworkspace`` in the ``ios`` folder.
- At the top of your project hierarchy view, select the ``Runner`` project.
- Select the ``Runner`` target, go to the ``General`` tab and scroll down until you see ``Frameworks, Libraries, Embedded Content``
- Press the ``+`` icon and add the ``AudioToolbox.framework`` framework. Select ``Do Not Embed`` under the ``Embed`` column.

### macOS
You will need to update the macOS build versions to ``10.13``

- Open ``Runner.xcworkspace`` in the ``macos`` folder.
- At the top of your project hierarchy view, select the ``Runner`` project.
- In your ``Runner`` project, go to the ``Info`` tab and set ``macOS Deployment Target`` to ``10.13``
- Select the ``Runner`` target and set the ``Minimum Deployments`` macOS version to ``10.13``
