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
