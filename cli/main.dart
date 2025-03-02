import 'package:args/command_runner.dart';

import 'commands/build/build.dart';
import 'commands/codegen.dart';

void main(List<String> args) {
  final CommandRunner<int> runner =
      CommandRunner("package", "A tool to help with maintaining the package.");

  runner
    ..addCommand(BuildCommand())
    ..addCommand(CodegenCommand())
    ..run(args);
}
