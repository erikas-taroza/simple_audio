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
