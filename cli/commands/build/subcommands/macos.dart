import 'package:io/io.dart';

import '../../../cli_command.dart';

class MacosBuildCommand extends CliCommand {
  @override
  String get name => "macos";

  @override
  String get description => "Builds the Rust code for macOS.";

  @override
  Future<int> run() async {
    return ExitCode.success.code;
  }
}
