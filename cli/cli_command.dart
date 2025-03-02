import 'package:args/command_runner.dart';

abstract class CliCommand extends Command<int> {
  bool get verbose => true;

  CliCommand() {
    if (verbose) {
      argParser.addFlag("verbose", abbr: "v", defaultsTo: false);
    }
  }
}
