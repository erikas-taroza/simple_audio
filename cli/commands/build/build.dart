import 'dart:async';
import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:io/src/exit_code.dart';

import '../../cli_command.dart';
import '../../mixins/logger.dart';
import 'android.dart';

class BuildCommand extends CliCommand with Logger {
  @override
  String get name => "build";

  @override
  List<String> get aliases => ["b"];

  @override
  String get description => "Builds the Rust code for the given target.";

  @override
  bool get verbose => false;

  BuildCommand() {
    addSubcommand(AndroidBuildCommand());
  }
}
