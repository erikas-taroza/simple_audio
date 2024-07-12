# Simple Audio

A cross-platform solution for playing audio in Flutter.

This project's goal is to be as simple as possible, meaning it offers the core functionality only (ex. play/pause).
It also aims to be stable and relatively bug free.

I created this plugin for my music app so that I don't have to use different packages for different
platforms (audio_service, dart_vlc). This made it hard to deal with bugs from different packages.

## Features

- Simple API
- Cross platform (Android, Linux, Windows, iOS, macOS)
- Playback of local and online resources
- Gapless playback and preloading
- Volume normalization

## Documentation

The documentation is hosted by `pub.dev`. You can find it [here](https://pub.dev/documentation/simple_audio/latest/simple_audio/simple_audio-library.html).

## Usage

- Add this plugin as a dependency in `pubspec.yaml`
- Follow the [setup steps](https://github.com/erikas-taroza/simple_audio#setup).
- Call `SimpleAudio.init()` in `main()` ([see below](https://github.com/erikas-taroza/simple_audio#initialization)).
- Instantiate a `SimpleAudio` object in your player controller ([see below](https://github.com/erikas-taroza/simple_audio#create-player)).
- Use the APIs provided in the instantiated object ([see below](https://github.com/erikas-taroza/simple_audio#open-and-play)).

An example player project is located [here](https://github.com/erikas-taroza/simple_audio/tree/master/example).

### Initialization

```dart
void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  await SimpleAudio.init();

  runApp(const MyApp());
}
```

### Create Player

```dart
final SimpleAudio player = SimpleAudio(shouldNormalizeVolume: false);
```

### Open and Play

```dart
player.playbackState.listen((state) => print(state));
player.progressState.listen((state) => print(state));
// Autoplays by default
player.open(path);
```

## Setup

Some platform specific things have to be set up in order for this plugin to function properly.

### Windows, Linux, macOS, Android

No setup is needed.

### iOS

You will have to add a dependency to your Xcode project.

- Open `Runner.xcworkspace` in the `ios` folder.
- At the top of your project hierarchy view, select the `Runner` project.
- Select the `Runner` target, go to the `General` tab and scroll down until you see `Frameworks, Libraries, Embedded Content`
- Press the `+` icon and add the `AudioToolbox.framework` framework. Select `Do Not Embed` under the `Embed` column.

### Special Thanks

- [sanihaq](https://github.com/sanihaq) - Reported multiple bugs in `simple_audio`'s first releases.
  These bugs would not have been fixed for a long time without their help.
