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

import '../cli_command.dart';

mixin PackageInfo on CliCommand {
  String get frbVersion => "2.7.0";
  String get packageName => "simple_audio_flutter";

  String get projectRootDirectory => _projectRootDirectory;
  static String _projectRootDirectory = "";
  static void initProjectRootDirectory() {
    Directory directory = Directory.current;

    // Starts from the current directory and goes up until it finds the root project directory.
    while (true) {
      List<String> items = directory
          .listSync()
          .map((e) => e.uri.pathSegments[e.uri.pathSegments.length - 2])
          .toList();

      if (items.contains("simple_audio_flutter") &&
          items.contains("simple_audio")) {
        _projectRootDirectory = directory.path;
        break;
      } else {
        directory = directory.parent;
      }
    }
  }
}
