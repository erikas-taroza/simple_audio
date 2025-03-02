import 'dart:io';

import 'package:io/io.dart';

import '../../../cli_command.dart';
import '../../../mixins/cli_logger.dart';
import '../../../mixins/package_info.dart';
import '../../../mixins/process_runner.dart';

class WindowsBuildCommand extends CliCommand
    with CliLogger, ProcessRunner, PackageInfo {
  @override
  String get name => "windows";

  @override
  String get description => "Builds the Rust code for Windows.";

  @override
  Future<int> run() async {
    // Ensure rustup targets are installed.
    logger.stdout("Installing rustup targets...");
    int result = await runProcess(
      "rustup",
      [
        "target",
        "add",
        "x86_64-pc-windows-msvc",
      ],
      logger: logger,
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    logger.stdout("Building Windows binary...");
    result = await runProcess(
      "cargo",
      [
        "build",
        "--release",
        "--target",
        "x86_64-pc-windows-msvc",
      ],
      logger: logger,
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    final File file =
        File("$projectRootDirectory/$packageName/windows/$packageName.dll");
    if (await file.exists()) {
      logger.trace("Found existing Windows binary. Deleting...");
      await file.delete();
    }

    await File(
      "$projectRootDirectory/target/x86_64-pc-windows-msvc/release/$packageName.dll",
    ).rename("$projectRootDirectory/$packageName/windows/$packageName.dll");

    logger.stdout("Done!");
    return ExitCode.success.code;
  }
}
