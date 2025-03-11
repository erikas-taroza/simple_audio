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

import 'dart:convert';
import 'dart:io';

import 'package:io/io.dart';

import '../../../cli_command.dart';
import '../../../mixins/cli_logger.dart';
import '../../../mixins/package_info.dart';
import '../../../mixins/process_runner.dart';

class IosBuildCommand extends CliCommand
    with CliLogger, ProcessRunner, PackageInfo {
  @override
  String get name => "ios";

  @override
  String get description => "Builds the Rust code for iOS.";

  @override
  Future<int> run() async {
    // Ensure rustup targets are installed.
    logger.info("Installing rustup targets...");
    int result = await runProcess(
      "rustup",
      [
        "target",
        "add",
        "aarch64-apple-ios",
        "aarch64-apple-ios-sim",
        "x86_64-apple-ios",
      ],
      logger: logger,
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    // Build binaries
    logger.info("Building iOS binaries...");

    result = await runProcess(
      "cargo",
      [
        "build",
        "--release",
        "--target",
        "aarch64-apple-ios",
      ],
      logger: logger,
      environment: {"IPHONEOS_DEPLOYMENT_TARGET": "10.0"},
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
        "aarch64-apple-ios-sim",
      ],
      logger: logger,
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    // Get simulator SDK path
    Process process = await Process.start(
      "xcrun",
      [
        "--sdk",
        "iphonesimulator",
        "--show-sdk-path",
      ],
    );

    final String simulatorSdkPath = await process.stdout
        .transform(utf8.decoder)
        .transform(LineSplitter())
        .first;

    result = await runProcess(
      "cargo",
      [
        "build",
        "--release",
        "--target",
        "x86_64-apple-ios",
      ],
      logger: logger,
      environment: {
        "CMAKE_OSX_SYSROOT": simulatorSdkPath,
      },
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    // Merge aarch64-apple-ios-sim and x86_64-apple-ios targets
    logger.detail(
      "Merging aarch64-apple-ios-sim and x86_64-apple-ios with lipo...",
    );

    result = await runProcess(
      "lipo",
      [
        "$projectRootDirectory/target/aarch64-apple-ios-sim/release/lib$packageName.a",
        "$projectRootDirectory/target/x86_64-apple-ios/release/lib$packageName.a",
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
        "$projectRootDirectory/target/aarch64-apple-ios/release/lib$packageName.a",
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
      "$projectRootDirectory/$packageName/ios/Frameworks/$packageName.xcframework",
    );

    if (await directory.exists()) {
      logger.detail("Found existing xcframework. Deleting...");
      await directory.delete(recursive: true);
    }

    // Move the created xcframework.
    await Directory("$projectRootDirectory/$packageName/ios/Frameworks")
        .create(recursive: true);
    await Directory("$packageName.xcframework").rename(
      "$projectRootDirectory/$packageName/ios/Frameworks/$packageName.xcframework",
    );

    logger.success("Done!");
    return ExitCode.success.code;
  }
}
