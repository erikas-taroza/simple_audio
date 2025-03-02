// This file is a part of simple_audio
// Copyright (c) 2022-2023 Erikas Taroza <erikastaroza@gmail.com>
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

import '../../cli_command.dart';
import 'subcommands/android.dart';
import 'subcommands/ios.dart';
import 'subcommands/linux.dart';
import 'subcommands/macos.dart';
import 'subcommands/windows.dart';

class BuildCommand extends CliCommand {
  @override
  String get name => "build";

  @override
  List<String> get aliases => ["b"];

  @override
  String get description => "Builds the Rust code for the given target.";

  BuildCommand() {
    addSubcommand(AndroidBuildCommand());

    if (Platform.isLinux) addSubcommand(LinuxBuildCommand());

    if (Platform.isWindows) addSubcommand(WindowsBuildCommand());

    if (Platform.isMacOS) {
      addSubcommand(IosBuildCommand());
      addSubcommand(MacosBuildCommand());
    }
  }
}
