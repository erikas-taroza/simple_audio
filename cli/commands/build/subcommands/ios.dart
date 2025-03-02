import 'package:io/io.dart';

import '../../../cli_command.dart';

class IosBuildCommand extends CliCommand {
  @override
  String get name => "ios";

  @override
  String get description => "Builds the Rust code for iOS.";

  @override
  Future<int> run() async {
    return ExitCode.success.code;
  }
}
