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
    Logger? logger,
  }) async {
    Process process = await Process.start(
      executable,
      arguments,
      workingDirectory: workingDirectory,
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
