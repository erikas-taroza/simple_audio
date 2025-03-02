import 'package:cli_util/cli_logging.dart';
import 'package:cli_util/cli_logging.dart' as cli_logger;

import '../cli_command.dart';

mixin Logger on CliCommand {
  cli_logger.Logger get logger =>
      argResults?["verbose"] ?? false ? VerboseLogger() : StandardLogger();
}
