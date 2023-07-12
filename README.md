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
- Gapless playback and preloading
- Volume normalization

## Documentation

The documentation is hosted by `pub.dev`. You can find it [here](https://pub.dev/documentation/simple_audio/latest/simple_audio/simple_audio-library.html).

## Usage

- Add this plugin as a dependency in `pubspec.yaml`
- Follow the [setup steps](https://github.com/erikas-taroza/simple_audio#setup).
- Call `SimpleAudio.init()` in `main()` and configure as you need ([see below](https://github.com/erikas-taroza/simple_audio#initialization)).
- Instantiate a `SimpleAudio` object in your player controller ([see below](https://github.com/erikas-taroza/simple_audio#create-player)).
- Use the APIs provided in the instantiated object.

An example player project is located [here](https://github.com/erikas-taroza/simple_audio/tree/master/example).

### Initialization

```dart
void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize with default values.
  await SimpleAudio.init(
    useMediaController: true,
    shouldNormalizeVolume: false,
    dbusName: "com.erikas.SimpleAudio",
    actions: [
      MediaControlAction.rewind,
      MediaControlAction.skipPrev,
      MediaControlAction.playPause,
      MediaControlAction.skipNext,
      MediaControlAction.fastForward
    ],
    androidNotificationIconPath: "mipmap/ic_launcher",
    androidCompactActions: [1, 2, 3],
    applePreferSkipButtons: true
  );

  runApp(const MyApp());
}
```

### Create Player

```dart
final SimpleAudio player = SimpleAudio(
  onSkipPrevious: (SimpleAudio player) {},
  onSkipNext: (SimpleAudio player) {},
  onNetworkStreamError: (SimpleAudio player) {},
  onDecodeError: (SimpleAudio player) {},
);
```

## Setup

Some platform specific things have to be set up in order for this plugin to function properly.

### Windows

No setup is needed.

### Linux

Add this code to `my_application.cc` in the `linux` directory. This will allow
your app to open when the MPRIS notification is clicked.

```diff
static void my_application_activate(GApplication* application) {
+ GList* windows = gtk_application_get_windows(GTK_APPLICATION(application));
+ if(windows)
+ {
+   gtk_window_present_with_time(
+     GTK_WINDOW(windows->data),
+     g_get_monotonic_time() / 1000
+   );
+   return;
+ }
+
  MyApplication* self = MY_APPLICATION(application);
// ...
}

// ...

MyApplication* my_application_new() {
  return MY_APPLICATION(g_object_new(my_application_get_type(),
    "application-id", APPLICATION_ID, "flags",
-   G_APPLICATION_NON_UNIQUE
+   G_APPLICATION_HANDLES_COMMAND_LINE | G_APPLICATION_HANDLES_OPEN,
    nullptr
  ));
}
```

### Android

You will need to edit the Android manifest located in the `android/app/src/main` directory.

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

- Open `Runner.xcworkspace` in the `ios` folder.
- At the top of your project hierarchy view, select the `Runner` project.
- Select the `Runner` target, go to the `General` tab and scroll down until you see `Frameworks, Libraries, Embedded Content`
- Press the `+` icon and add the `AudioToolbox.framework` framework. Select `Do Not Embed` under the `Embed` column.

Add this to your `Info.plist` file located in `ios/Runner`:

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

You will need to update the macOS build versions to `10.13`

- Open `Runner.xcworkspace` in the `macos` folder.
- At the top of your project hierarchy view, select the `Runner` project.
- In your `Runner` project, go to the `Info` tab and set `macOS Deployment Target` to `10.13`
- Select the `Runner` target and set the `Minimum Deployments` macOS version to `10.13`

### Special Thanks

- [sanihaq](https://github.com/sanihaq) - Reported multiple bugs in `simple_audio`'s first releases.
  These bugs would not have been fixed for a long time without their help.
