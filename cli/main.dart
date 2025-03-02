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

import 'package:args/command_runner.dart';

import 'commands/build/build.dart';
import 'commands/publish.dart';
import 'commands/update.dart';
import 'commands/codegen.dart';
import 'mixins/package_info.dart';

void main(List<String> args) {
  final CommandRunner<int> runner =
      CommandRunner("package", "A tool to help with maintaining the package.");

  PackageInfo.init();

  runner
    ..addCommand(BuildCommand())
    ..addCommand(CodegenCommand())
    ..addCommand(UpdateCommand())
    ..addCommand(PublishCommand())
    ..run(args);
}
