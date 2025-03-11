// This file is a part of simple_audio
// Copyright (c) 2022-2025 Erikas Taroza <erikastaroza@gmail.com>
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program.
// If not, see <https://www.gnu.org/licenses/>.

import "dart:io";

import "package:io/io.dart";

import "../../../cli_command.dart";
import "../../../mixins/cli_logger.dart";
import "../../../mixins/package_info.dart";
import "../../../mixins/process_runner.dart";

class LinuxBuildCommand extends CliCommand
    with CliLogger, ProcessRunner, PackageInfo {
  @override
  String get name => "linux";

  @override
  String get description => "Builds the Rust code for Linux.";

  @override
  Future<int> run() async {
    // Ensure rustup targets are installed.
    logger.info("Installing rustup targets...");
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

    logger.info("Building Linux binary...");
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
      logger.detail("Found existing Linux binary. Deleting...");
      await file.delete();
    }

    await File(
      "$projectRootDirectory/target/x86_64-unknown-linux-gnu/release/lib$packageName.so",
    ).rename("$projectRootDirectory/$packageName/linux/lib$packageName.so");

    logger.success("Done!");
    return ExitCode.success.code;
  }
}
