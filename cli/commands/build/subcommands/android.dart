import 'package:io/io.dart';

import '../../../cli_command.dart';

class AndroidBuildCommand extends CliCommand {
  @override
  String get name => "android";

  @override
  String get description => "Builds the Rust code for Android.";

  @override
  Future<int> run() async {
    return ExitCode.success.code;
  }
}
