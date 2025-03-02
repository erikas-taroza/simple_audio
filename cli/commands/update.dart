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

import 'dart:io';

import 'package:mason_logger/mason_logger.dart';
import 'package:toml/toml.dart';
import 'package:yaml_edit/yaml_edit.dart';

import '../cli_command.dart';
import '../mixins/cli_logger.dart';
import '../mixins/package_info.dart';

class UpdateCommand extends CliCommand with CliLogger, PackageInfo {
  @override
  String get name => "update";

  @override
  List<String> get aliases => ["u"];

  @override
  String get description => "Updates the version shown in files.";

  UpdateCommand() {
    argParser
      ..addOption(
        "level",
        abbr: "l",
        mandatory: true,
        help: "The semver version level to change.",
        allowed: ["major", "minor", "patch"],
        allowedHelp: {
          "major": "Updates the first number in the version (x.0.0)",
          "minor": "Updates the second number in the version (0.x.0)",
          "patch": "Updates the third number in the version (0.0.x)",
        },
      )
      ..addFlag(
        "dry-run",
        abbr: "d",
        help:
            "Performs the steps to update the version but doesn't commit the changes.",
      );
  }

  @override
  Future<int> run() async {
    final bool isDryRun = argResults?.flag("dry-run") ?? false;
    final String? option = argResults?.option("level");
    if (option == null) {
      logger.err('Missing argument for "--level".');
      return ExitCode.usage.code;
    }

    final File pubspec =
        File("$projectRootDirectory/$packageName/pubspec.yaml");
    final YamlEditor yamlEditor = YamlEditor(await pubspec.readAsString());

    final _Level level = _Level.fromString(option);
    final _Version currentVersion =
        _Version.fromString(yamlEditor.parseAt(["version"]).value);
    final _Version newVersion = currentVersion.increment(level);

    if (!isDryRun) {
      String answer = logger.prompt(
        "Update ${level.name} version to $newVersion from $currentVersion?",
        defaultValue: "y",
      );

      if (answer.toLowerCase() != "y") {
        return ExitCode.success.code;
      }
    } else {
      logger.info(
        "Updating ${level.name} version to $newVersion from $currentVersion.",
      );
    }

    // pubspec.yaml
    yamlEditor.update(["version"], newVersion.toString());
    if (!isDryRun) {
      await pubspec.writeAsString(yamlEditor.toString());
      logger.detail("Updated ${pubspec.path}");
    } else {
      logger.info(styleBold.wrap(pubspec.path) ?? pubspec.path);
      logger.info(
        darkGray.wrap(
              yamlEditor.toString().replaceFirst(
                    newVersion.toString(),
                    "${green.wrap(newVersion.toString()) ?? newVersion.toString()}${darkGray.escape}",
                  ),
            ) ??
            yamlEditor.toString(),
      );
      logger.info("\n");
    }

    // Cargo.toml
    final List<String> cargoPaths = [
      "$projectRootDirectory/simple_audio/Cargo.toml",
      "$projectRootDirectory/simple_audio_flutter/rust/Cargo.toml",
    ];

    for (String path in cargoPaths) {
      final Map toml = (await TomlDocument.load(
        path,
      ))
          .toMap();
      toml["package"]["version"] = newVersion.toString();

      final String encodedToml = TomlDocument.fromMap(toml).toString();

      if (!isDryRun) {
        await File(path).writeAsString(encodedToml);
        logger.detail("Updated $path");
      } else {
        logger.info(styleBold.wrap(path) ?? path);
        logger.info(
          darkGray.wrap(
                encodedToml.replaceFirst(
                  newVersion.toString(),
                  "${green.wrap(newVersion.toString()) ?? newVersion.toString()}${darkGray.escape}",
                ),
              ) ??
              encodedToml,
        );
        logger.info("\n");
      }
    }

    // CMake
    final List<String> cmakePaths = [
      "$projectRootDirectory/$packageName/android/CMakeLists.txt",
      "$projectRootDirectory/$packageName/linux/CMakeLists.txt",
      "$projectRootDirectory/$packageName/windows/CMakeLists.txt",
    ];

    for (String path in cmakePaths) {
      final File cmake = File(path);
      final String text = (await cmake.readAsString()).replaceFirst(
        RegExp(r'set\(Version "[\d|\.]+"\)\s'),
        'set(Version "${newVersion.toString()}")\n',
      );

      if (!isDryRun) {
        await cmake.writeAsString(text);
        logger.detail("Updated $path");
      } else {
        logger.info(styleBold.wrap(path) ?? path);
        logger.info(
          darkGray.wrap(
                text.replaceFirst(
                  newVersion.toString(),
                  "${green.wrap(newVersion.toString()) ?? newVersion.toString()}${darkGray.escape}",
                ),
              ) ??
              text,
        );
        logger.info("\n");
      }
    }

    // Podspec
    final String podspecName = yamlEditor.parseAt(["name"]).toString();
    final List<String> podspecPaths = [
      "$projectRootDirectory/$packageName/macos/$podspecName.podspec",
      "$projectRootDirectory/$packageName/ios/$podspecName.podspec",
    ];

    for (String path in podspecPaths) {
      final File podspec = File(path);
      String text = await podspec.readAsString();
      text = text.replaceFirst(
        RegExp(r'version = "[\d|\.]+"\s'),
        'version = "$newVersion"\n',
      );
      text = text.replaceFirst(
        RegExp(r"s\.version\s+= '[\d|\.]+'\s"),
        "s.version          = '$newVersion'\n",
      );

      if (!isDryRun) {
        await podspec.writeAsString(text);
        logger.detail("Updated $path");
      } else {
        logger.info(styleBold.wrap(path) ?? path);
        logger.info(
          darkGray.wrap(
                text.replaceFirst(
                  newVersion.toString(),
                  "${green.wrap(newVersion.toString()) ?? newVersion.toString()}${darkGray.escape}",
                ),
              ) ??
              text,
        );
        logger.info("\n");
      }
    }

    logger.success("Done!");
    return ExitCode.success.code;
  }
}

class _Version {
  final int major;
  final int minor;
  final int patch;

  _Version(this.major, this.minor, this.patch);

  _Version increment(_Level level) {
    switch (level) {
      case _Level.major:
        return _Version(major + 1, minor, patch);
      case _Level.minor:
        return _Version(major, minor + 1, patch);
      case _Level.patch:
        return _Version(major, minor, patch + 1);
    }
  }

  static _Version fromString(String value) {
    final List<int> numbers =
        value.split(".").map((e) => int.parse(e)).toList();
    return _Version(numbers[0], numbers[1], numbers[2]);
  }

  @override
  String toString() {
    return "$major.$minor.$patch";
  }
}

enum _Level {
  major,
  minor,
  patch;

  static _Level fromString(String? value) =>
      _Level.values.firstWhere((e) => e.name == value);
}
