# Contributing
Any contributions are valued and welcome. If you want to make a big contribution, please create an issue ahead of time.
This will allow us to discuss and determine if your work fits into the project's goals, as well as making sure your effort is not wasted.

All contributions must be licensed under the LGPL-3.0 license to be accepted.

## Project Setup
This project uses `flutter_rust_bridge` which does all the hard work for allowing Dart to use the Rust codebase.
This will have to be installed (not manually, see below). The versions of `flutter_rust_bridge` must match with every instance. For example,
there is `flutter_rust_bridge_codegen` (CLI tool) and `flutter_rust_bridge` (Rust + Dart dependency).
These must all have the same version for proper functionality.

If you need it, the documentation for `flutter_rust_bridge` is [here](https://cjycode.com/flutter_rust_bridge/).

### Platform Specific Development
`simple_audio` has some platform specific code. Here is what I recommend to easily edit that code:
- Linux: VSCode on a Linux desktop computer.
- Android: Android Studio on a Linux desktop computer.
- macOS/iOS: XCode on a Mac desktop computer (or a [VM](https://github.com/notAperson535/OneClick-macOS-Simple-KVM)).
- Windows: VSCode on a Windows desktop computer.

**NOTE**: If you are testing/building for Linux, please install the ALSA development files.

### OpenSSL
This project uses OpenSSL. If you are building and you get an error related to OpenSSL follow these steps:
- Install Perl with CPAN.
- You may need to clone [`openssl`](https://github.com/openssl/openssl) and run the `Configure` script.

On iOS, running the `Configure` script is necessary because the `vendored` feature for the `openssl` crate
does not work for the `aarch64-apple-ios-sim` target.

## Plugin Tool
I have written a tool in Python that allows one to easily run common commands.
This tool can be used to run the codegen and build the libraries.
This tool should only be run from the root directory.
If there are any missing dependencies (ex. `flutter_rust_bridge_codegen`), they will be installed.

Show the help section:
```
python plugin_tool.py --help
```

Generate code:
```
python plugin_tool.py -c
```

Build:
```
python plugin_tool.py -b platform
```

**NOTE**: The builds are OS specific. For example, you can't build macOS libraries from Linux.
This isn't a big issue because the CI handles the building, but keep this in mind if you are testing.

**NOTE**: By default, the build does `Link-Time Optimization` which increases the build times.
This can be annoying when testing so you can comment `lto = true` in the `Cargo.toml` file.

## Project Structure
This project has 3 parts:
- Dart (`./lib`)
  - Contains the public API.
- Rust (`./rust`)
  - Main codebase that provides all the functionality.
  - `lib.rs`: The API that is used in Dart. Only public items are exported.
  - `./utils`: Contains things like Rust->Dart streams. Helpful things that don't have a better place to go.
  - `./metadata`: Code that handles the MPRIS (Linux) and SMTC (Windows) media controllers.
  - `./audio`: Playback functionality.
    - `./audio/dsp`: Digital Signal Processing. Handles things like normalizing and resampling.
    - `./audio/streaming`: Custom sources for streaming online files.
- Native (`./darwin/Classes`, `./macos/Classes`, `./ios/Classes`, `./android/src/main/kotlin/com/erikas/simple_audio`)
  - Handles functionality that can't be done from Rust.
  - Used for the media controllers.

When contributing, you will most likely be writing code in Rust.
If needed, you will have to run the codegen. There are some cases where you
will not need to run the codegen (ex. method body was not modified). If you are unsure, run the codegen anyways.

Depending on what you write, some changes may have to be made the the Dart side. In this case, please create
an easy to use public API and make sure that any new types are exported if they are needed.

## Code Guidelines
- Readable
- Proper naming
- Comments where needed
- Doc comments for new API elements

## Submit a PR
Maintaniers will be able to review your code to make sure that it functions as expected and also follows the guidelines above.
If your PR fixes open issues, please make sure you include them in the description.
