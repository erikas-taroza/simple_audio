import 'dart:io';

import 'package:io/io.dart';

import '../../../cli_command.dart';
import '../../../mixins/cli_logger.dart';
import '../../../mixins/package_info.dart';
import '../../../mixins/process_runner.dart';

class MacosBuildCommand extends CliCommand
    with CliLogger, ProcessRunner, PackageInfo {
  @override
  String get name => "macos";

  @override
  String get description => "Builds the Rust code for macOS.";

  @override
  Future<int> run() async {
    // Ensure rustup targets are installed.
    logger.stdout("Installing rustup targets...");
    int result = await runProcess(
      "rustup",
      [
        "target",
        "add",
        "aarch64-apple-darwin",
        "x86_64-apple-darwin",
      ],
      logger: logger,
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    // Build binaries
    logger.stdout("Building macOS binaries...");

    result = await runProcess(
      "cargo",
      [
        "build",
        "--release",
        "--target",
        "aarch64-apple-darwin",
      ],
      logger: logger,
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    result = await runProcess(
      "cargo",
      [
        "build",
        "--release",
        "--target",
        "x86_64-apple-darwin",
      ],
      logger: logger,
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    // Merge aarch64-apple-darwin and x86_64-apple-darwin targets
    logger.trace(
      "Merging aarch64-apple-darwin and x86_64-apple-darwin with lipo...",
    );

    result = await runProcess(
      "lipo",
      [
        "$projectRootDirectory/target/aarch64-apple-darwin/release/lib$packageName.a",
        "$projectRootDirectory/target/x86_64-apple-darwin/release/lib$packageName.a",
        "-output",
        "lib$packageName.a",
        "-create",
      ],
      logger: logger,
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    final File file =
        File("$projectRootDirectory/$packageName/macos/Libs/lib$packageName.a");
    if (await file.exists()) {
      logger.trace("Found existing macOS binary. Deleting...");
      await file.delete();
    }

    await Directory("$projectRootDirectory/$packageName/macos/Libs")
        .create(recursive: true);
    await File("lib$packageName.a").rename(
      "$projectRootDirectory/$packageName/macos/Libs/lib$packageName.a",
    );

    logger.stdout("Done!");
    return ExitCode.success.code;
  }
}
