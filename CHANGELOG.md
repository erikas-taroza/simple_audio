## 2.0.1

- Support Gradle 8

## 2.0.0

Main changes:

- Removed media controllers
  - Please check the [example project](https://github.com/erikas-taroza/simple_audio/tree/master/simple_audio_flutter/example) to see how you can implement media controllers using other packages
- Decoupled Rust code from FlutterRustBridge.
  - Allows for compatibility with other FlutterRustBridge packages that may have a different version from simple_audio. If you need this functionality, please check how [`simple_audio_flutter`](https://github.com/erikas-taroza/simple_audio/tree/master/simple_audio_flutter/rust) implements a Dart API.
- API changes

How to migrate to V2 from v1.9.0:

- `SimpleAudio.init()` no longer takes any parameters.
- `SimpleAudio()` constructor only takes one optional parameter: `shouldNormalizeVolume`
- `playbackStateStream` is renamed to `playbackState`
- `progressStateStream` is renamed to `progressState`
- Network errors are now handled by listening to the `networkError` stream
- Decode errors are now handled by listening to the `decodeError` stream
- `ProgressState` now represents time via `Duration`. To get the position in seconds, do `state.position.inSeconds`
- `seek` now takes a `Duration`. To seek to a given point in seconds, do `seek(Duration(seconds: seconds)`
- `setMetadata` and `Metadata` are removed. Please use other packages as shown in the [example](https://github.com/erikas-taroza/simple_audio/tree/master/simple_audio_flutter/example) to implement media controllers.

## 1.9.0

- Improved gapless playback
  - The preloaded file is automatically played when the current file is done playing.
  - `clearPreload()` removes the preloaded file so that it doesn't automatically play.
  - `PreloadPlayed` playback state which is sent after the preload was automatically played.

## 1.8.0

- Add `stop` event (thanks @dannyglover)
- Add `uuid` dependency

## 1.7.0

- Improved error handling. Custom errors are thrown in `open()` and `preload()`. The `onNetworkStreamError` and `onDecodeError` callbacks now have a parameter for the error message.
- Updated `flutter_rust_bridge` to `1.82.1`
- Removed useless comments in `pubspec.yaml`

## 1.6.6

- Specify explicit version of `flutter_rust_bridge`

## 1.6.5

- Increase device buffer size, compare buffer size to default instead of `1`

## 1.6.4

- Select correct buffer size based on default output config.
- Extract `ios.zip` to `simple_audio.xcframework` directory (thanks @Yesterday17)
- Add topics to pubspec.yaml

## 1.6.3

- Fix method channels hanging because a result wasn't returned in native code.

## 1.6.2

- Release assets use `tar.gz` for Linux/Android/Windows
- Update README
- Update git/pub ignore

## 1.6.1

- Fixed normalization gain calculation.
- Normalizer has smoother volume changes and don't apply excess positive gain.
- Refactored CD. Built libraries are now release assets instead of files committed to the repo.
- Created a CI.

## 1.6.0

- Opus support (thanks @Yesterday17)

## 1.5.8

- Fix stream playback stopping for a moment when preloading another network stream.
- Fix preloaded output stream breaking the next streams on Windows.
- Fix `playPreload` not auto playing on Windows.

## 1.5.7

- HTTP and HLS streams fallback to downloading the file if `Content-Length` cannot be acquired.
- More CD improvements

## 1.5.6

- Update `.pubignore` to exclude the iOS framework `Info.plist` file.
- Improve CD workflow file.
  - Use different context variables
  - Add `--rebase` flag to `git pull`
- Remove OpenSSL dependency to improve CD build times on iOS.

## 1.5.5

- Fix preloaded file not playing after output device is changed.
- MPRIS: Fix error when hot reloading because name is taken.
- Increase downloaded chunk size for network streams.

## 1.5.4

- Fix Android audio crackling with Bluetooth headphones.
- Fix high CPU usage with MPRIS.
- Improve README
- Make `ACTIVE_LOCK` private.

## 1.5.3

- Removed static global variables that controls playback.
- Refactor MPRIS and SMTC media controllers.
- MPRIS and SMTC now update their position.
- Removed loops that wait on a valid duration for media controllers.
- CD works with different branches.

## 1.5.2

- Add default timebase (thanks @Yesterday17)
- Workflow commits use "CD" instead of "CI".
- Code gen is run when `lib.rs` is changed.

## 1.5.1

- Improved preload performance (stream is created in a different thread).
- Fix `stop()` not working during playback.
- Fix gapless playback for Android.

## 1.5.0

- Add support for gapless playback.
- Fix panic when a file's timebase is `None`.

## 1.4.5

- **BREAKING**: These names are more generic since not every OS has a notification as a media controller.
  - Renamed `showMediaNotification` to `useMediaController` in `SimpleAudio.init()`
  - Renamed `androidCompactPlaybackActions` to `androidCompactActions` in `SimpleAudio.init()`
  - Renamed `NotificationActions` to `MediaControlAction`
- Update `flutter_rust_bridge`
  - Enums are generated in camelCase.
  - Rust can take a list of `MediaControlAction` instead of ints.
- Refactored decoder
  - Prevent multiple playbacks happening after spamming `open()`
- Update example/docs

## 1.4.4

- Handle hot restarts.

## 1.4.3

- Fix method channel being used for incorrect platforms when looping playback.

## 1.4.2

- Add missing Android libs from CI fail.

## 1.4.1

- Fix progress bar on media notifications not changing after loop.
- Fix `simple_audio` blocking when `stop()` is called twice.
- Update example.
- Other changes
  - Formatting (Rust, Kotlin)
  - Refactoring (Rust)

## 1.4.0

- Support changing output devices.
- Remove playback error handling.
  - The main cause of this was the output device changing.

## 1.3.0

- Allow user to handle errors. The following errors can be handled in Dart:
  - Fail to open a file
  - Error in network stream (ex. the URL is no longer valid)
  - Error decoding the file (ex. the file isn't formatted correctly)
  - Error during playback (ex. the device that was outputting is no longer available)
- Fix the playback state not being updated when calling `open()`

## 1.2.1

- Fix Android CMake
- Update Flutter Rust Bridge

## 1.2.0

- Added the option to normalize volume.
- Improve documentation.

## 1.1.0

- Linux: CI builds on older Ubuntu version to support older `glibc` versions.
- Linux: MPRIS notification now opens the app when clicked.
- Android: Support [Android 13's new media style notification](https://developer.android.com/about/versions/13/behavior-changes-13#playback-controls)

## 1.0.7

- Remove some Android setup requirements.
- Fix foreground service keeping Android app alive.
  - The fix uses `endProcess` which is like force stopping the app.
    If you have a better solution to this, submit an issue or PR.

## 1.0.6

- Streaming with HTTP Range header for network files.
- Stream HLS files by downloading the given parts on demand.

## 1.0.5

- Add the ability to loop.
- Update Rust dependencies.
- Improve Rust codebase with clippy suggestions.

## 1.0.4

- Media controllers can take images as bytes.
- **Android:** Playback not working when reopening app

## 1.0.3

- Improved README
- Added license headers to files that were missing it
- iOS calling play/pause quickly stops playback

## 1.0.2

- Build scripts download binaries (for pub.dev)

## 1.0.1

- Set versions in `pubspec.yaml`
- Simplified Linux CMAKE
- Fixed macOS dylib trying to link to a file on GitHub Actions.

## 1.0.0

First version
