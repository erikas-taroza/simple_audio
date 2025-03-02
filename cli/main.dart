import 'package:args/command_runner.dart';

import 'commands/build/build.dart';
import 'commands/update.dart';
import 'commands/codegen.dart';
import 'mixins/package_info.dart';

void main(List<String> args) {
  final CommandRunner<int> runner =
      CommandRunner("package", "A tool to help with maintaining the package.");

  PackageInfo.initProjectRootDirectory();

  runner
    ..addCommand(BuildCommand())
    ..addCommand(CodegenCommand())
    ..addCommand(UpdateCommand())
    ..run(args);
}
