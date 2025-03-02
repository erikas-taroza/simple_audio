import 'package:io/io.dart';

import '../../../cli_command.dart';

class WindowsBuildCommand extends CliCommand {
  @override
  String get name => "windows";

  @override
  String get description => "Builds the Rust code for Windows.";

  @override
  Future<int> run() async {
    return ExitCode.success.code;
  }
}
