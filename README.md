# Simple Audio

A cross-platform solution for playing audio in Flutter.

This project's goal is to be as simple as possible, meaning it offers the core functionality only (ex. play/pause).
It also aims to be stable and relatively bug free.

I created this plugin for my music app so that I don't have to use different packages for different
platforms (audio_service, dart_vlc). This made it hard to deal with bugs from different packages.

## Features
- Simple API
- Cross platform (Android, Linux, Windows, iOS, macOS)
- Media controllers on all platforms
  - **Linux:** MPRIS
  - **Android:** MediaSessionCompat
  - **Windows:** SystemMediaTransportControls
  - **iOS/macOS:** Control center
- Playback of local and online resources

## Usage
- Add this plugin as a dependency in ``pubspec.yaml``
- Call ``SimpleAudio.init()`` in ``main()`` and configure as you need.
- Instantiate a ``SimpleAudio`` object in your player controller.
- Use the APIs provided in the instantiated object.

To see a sample player project, please check the [example](https://github.com/erikas-taroza/simple_audio/tree/master/example).

```dart
// Initialize SimpleAudio
void main() {
  SimpleAudio.init();
  runApp(const MyApp());
}

// Instantiate a SimpleAudio object.
final SimpleAudio player = SimpleAudio();

// Stop the current playback.
await player.stop();

// Open a file for playing.
player.open("path/to/your/file.mp3", autoplay: true);
// OR
player.open("https://my-files.com/file.mp3", autoplay: true);

// Play
player.play();

// Pause
player.pause();

// Set the volume (0.0 - 1.0)
player.setVolume(0.5);

// Seek to the 5th second.
player.seek(5);

// Set the OS's media controller metadata.
player.setMetadata(Metadata(
  title: "My Title",
  artist: "My Artist",
  album: "My Album",
  artUri: "https://my-files.com/image.jpg"
));
```

## Setup
Some platform specific things have to be set up in order for this plugin to function properly.

### Windows/Linux
No setup is needed for these platforms.

### Android
You will need to edit the Android manifest located in the ``android/app/src/main`` directory.

#### AndroidManifest.xml
```xml
<!-- Add this permission. -->
<uses-permission android:name="android.permission.FOREGROUND_SERVICE"/>
<application>
  <!-- ... -->
  
  <!-- This is required if you want to be able to see the notification. -->
  <service
      android:name="com.erikas.simple_audio.SimpleAudioService"
      android:foregroundServiceType="mediaPlayback"
      android:exported="false">
      <intent-filter>
          <action android:name="android.media.browse.MediaBrowserService" />
      </intent-filter>
  </service>

  <!-- This is required to interact with the notification buttons. -->
  <receiver
      android:name="com.erikas.simple_audio.SimpleAudioReceiver">
  </receiver>
  
</application>
```

### iOS
You will have to add a dependency to your Xcode project.

- Open ``Runner.xcworkspace`` in the ``ios`` folder.
- At the top of your project hierarchy view, select the ``Runner`` project.
- Select the ``Runner`` target, go to the ``General`` tab and scroll down until you see ``Frameworks, Libraries, Embedded Content``
- Press the ``+`` icon and add the ``AudioToolbox.framework`` framework. Select ``Do Not Embed`` under the ``Embed`` column.

Add this to your ``Info.plist`` file located in ``ios/Runner``:
```xml
<dict>
  <!-- ... -->
  <key>UIBackgroundModes</key>
  <array>
    <string>audio</string>
  </array>
</dict>
```

### macOS
You will need to update the macOS build versions to ``10.13``

- Open ``Runner.xcworkspace`` in the ``macos`` folder.
- At the top of your project hierarchy view, select the ``Runner`` project.
- In your ``Runner`` project, go to the ``Info`` tab and set ``macOS Deployment Target`` to ``10.13``
- Select the ``Runner`` target and set the ``Minimum Deployments`` macOS version to ``10.13``
