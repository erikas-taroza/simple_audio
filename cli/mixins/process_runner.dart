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

import 'package:cli_util/cli_logging.dart';
import 'package:io/io.dart';

import '../cli_command.dart';

mixin ProcessRunner on CliCommand {
  Future<int> runProcess(
    String executable,
    List<String> arguments, {
    String? workingDirectory,
    Map<String, String>? environment,
    Logger? logger,
  }) async {
    Process process = await Process.start(
      executable,
      arguments,
      workingDirectory: workingDirectory,
      environment: environment,
    );

    final List<String> errs = [];

    process.stdout
        .transform(utf8.decoder)
        .transform(LineSplitter())
        .listen((out) {
      logger?.trace(out);
    });

    process.stderr
        .transform(utf8.decoder)
        .transform(LineSplitter())
        .listen((out) {
      errs.add(out);
    });

    int exitCode = await process.exitCode;

    for (String err in errs) {
      if (exitCode != ExitCode.success.code) {
        logger?.stderr(err);
      } else {
        logger?.trace(err);
      }
    }

    return process.exitCode;
  }
}
