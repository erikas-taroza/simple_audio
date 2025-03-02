import 'dart:io';

import 'package:io/io.dart';

import '../../../cli_command.dart';
import '../../../mixins/cli_logger.dart';
import '../../../mixins/package_info.dart';
import '../../../mixins/process_runner.dart';

class AndroidBuildCommand extends CliCommand
    with CliLogger, PackageInfo, ProcessRunner {
  @override
  String get name => "android";

  @override
  String get description => "Builds the Rust code for Android.";

  @override
  Future<int> run() async {
    // Ensure rustup targets are installed.
    logger.stdout("Installing rustup targets...");
    int result = await runProcess(
      "rustup",
      [
        "target",
        "add",
        "aarch64-linux-android",
        "armv7-linux-androideabi",
        "x86_64-linux-android",
        "i686-linux-android",
      ],
      logger: logger,
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    // Install cargo-ndk
    logger.stdout("Installing cargo-ndk...");
    result = await runProcess(
      "cargo",
      [
        "install",
        "cargo-ndk",
      ],
      logger: logger,
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    final List<String> architectures = [
      "arm64-v8a",
      "armeabi-v7a",
      "x86",
      "x86_64",
    ];

    // Delete existing binaries.
    for (String arch in architectures) {
      final File file = File(
        "$projectRootDirectory/$packageName/android/src/main/jniLibs/$arch/lib$packageName.so",
      );

      if (await file.exists()) {
        logger.trace("Found existing $arch binary. Deleting...");
        await file.delete();
      }
    }

    // Build the binaries
    logger.stdout("Building Android binaries...");
    result = await runProcess(
      "cargo",
      [
        "ndk",
        "-t",
        "arm64-v8a",
        "-t",
        "armeabi-v7a",
        "-t",
        "x86",
        "-t",
        "x86_64",
        "-o",
        "$projectRootDirectory/$packageName/android/src/main/jniLibs",
        "build",
        "--release",
      ],
      logger: logger,
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    logger.stdout("Done!");
    return ExitCode.success.code;
  }
}
