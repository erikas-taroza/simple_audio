import 'package:io/io.dart';

import '../../../cli_command.dart';

class LinuxBuildCommand extends CliCommand {
  @override
  String get name => "linux";

  @override
  String get description => "Builds the Rust code for Linux.";

  @override
  Future<int> run() async {
    return ExitCode.success.code;
  }
}
