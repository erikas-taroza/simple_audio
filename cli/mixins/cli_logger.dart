import 'package:cli_util/cli_logging.dart';

import '../cli_command.dart';

mixin CliLogger on CliCommand {
  Logger get logger =>
      argResults?["verbose"] ?? false ? VerboseLogger() : StandardLogger();
}
