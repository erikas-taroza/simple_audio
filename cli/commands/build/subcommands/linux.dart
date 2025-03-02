import 'dart:io';

import 'package:io/io.dart';

import '../../../cli_command.dart';
import '../../../mixins/cli_logger.dart';
import '../../../mixins/package_info.dart';
import '../../../mixins/process_runner.dart';

class LinuxBuildCommand extends CliCommand
    with CliLogger, ProcessRunner, PackageInfo {
  @override
  String get name => "linux";

  @override
  String get description => "Builds the Rust code for Linux.";

  @override
  Future<int> run() async {
    // Ensure rustup targets are installed.
    logger.stdout("Installing rustup targets...");
    int result = await runProcess(
      "rustup",
      [
        "target",
        "add",
        "x86_64-unknown-linux-gnu",
      ],
      logger: logger,
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    logger.stdout("Building Linux binary...");
    result = await runProcess(
      "cargo",
      [
        "build",
        "--release",
        "--target",
        "x86_64-unknown-linux-gnu",
      ],
      logger: logger,
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    final File file =
        File("$projectRootDirectory/$packageName/linux/lib$packageName.so");
    if (await file.exists()) {
      logger.trace("Found existing Linux binary. Deleting...");
      await file.delete();
    }

    File("$projectRootDirectory/target/x86_64-unknown-linux-gnu/release/lib$packageName.so")
        .rename("$projectRootDirectory/$packageName/linux/lib$packageName.so");

    logger.stdout("Done!");
    return ExitCode.success.code;
  }
}
