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

## CLI

I have written a CLI to easily run common commands.
This tool can be used to run the codegen and build the libraries.
If there are any missing dependencies (ex. `flutter_rust_bridge_codegen`), they will be installed.

Show the help section:

```
dart cli/main.dart help
```

**NOTE**: The command was run from the root directory. It is possible to run the CLI from anywhere you want in the project.

**NOTE**: The builds are OS specific. For example, you can't build macOS libraries from Linux.
When you run the `build` command, only options available on your OS are shown.
This isn't a big issue because the CI handles the building, but keep this in mind if you are testing.

## Project Structure

This project has 3 parts:

- cli
  - Contains the CLI for running commands as discussed above.
- simple_audio
  - Main Rust codebase that provides all the functionality.
- simple_audio_flutter
  - Dart bindings to the `simple_audio` project.

When contributing, you will most likely be writing code in Rust. Depending on your changes, you may need to generate code so that Dart can use the Rust API.

Depending on what you write, some changes may have to be made the the Dart side. In this case, please create
an easy to use public API and make sure that any new types are exported if they are needed.

## Formatting Rust Code

This project contains a `.rustfmt.toml` file. Some of the configurations require the
`nightly` channel to be installed.

To install:

```
rustup toolchain install nightly
```

To run the formatter:

```
cargo +nightly fmt
```

## Code Guidelines

- Readable
- Proper naming
- Comments where needed
- Doc comments for new API elements

## Submit a PR

If the CI is failing, do fix the errors.
Maintaniers will be able to review your code to make sure that it functions as expected and also follows the guidelines above.
If your PR fixes open issues, please make sure you include them in the description.
