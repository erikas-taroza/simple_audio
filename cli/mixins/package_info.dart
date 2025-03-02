import 'dart:io';

import '../cli_command.dart';

mixin PackageInfo on CliCommand {
  String get frbVersion => "2.7.0";
  String get packageName => "simple_audio_flutter";
  String get projectRootDirectory {
    final String current = Directory.current.path;
    return "${current.split("simple_audio")[0]}simple_audio";
  }
}
