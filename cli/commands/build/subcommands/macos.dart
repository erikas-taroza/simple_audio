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

class MacosBuildCommand extends CliCommand
    with CliLogger, ProcessRunner, PackageInfo {
  @override
  String get name => "macos";

  @override
  String get description => "Builds the Rust code for macOS.";

  @override
  Future<int> run() async {
    // Ensure rustup targets are installed.
    logger.info("Installing rustup targets...");
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
    logger.info("Building macOS binaries...");

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
    logger.detail(
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

    // Create xcframework
    logger.detail(
      "Creating xcframework...",
    );

    result = await runProcess(
      "xcodebuild",
      [
        "-create-xcframework",
        "-library",
        "lib$packageName.a",
        "-output",
        "$packageName.xcframework",
      ],
      logger: logger,
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    // Remove useless files
    await File("lib$packageName.a").delete();

    final Directory directory = Directory(
      "$projectRootDirectory/$packageName/macos/Frameworks/$packageName.xcframework",
    );

    if (await directory.exists()) {
      logger.detail("Found existing xcframework. Deleting...");
      await directory.delete(recursive: true);
    }

    // Move the created xcframework.
    await Directory("$projectRootDirectory/$packageName/macos/Frameworks")
        .create(recursive: true);
    await Directory("$packageName.xcframework").rename(
      "$projectRootDirectory/$packageName/macos/Frameworks/$packageName.xcframework",
    );

    logger.success("Done!");
    return ExitCode.success.code;
  }
}
