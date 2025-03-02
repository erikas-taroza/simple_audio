import 'dart:io';

import '../cli_command.dart';

mixin PackageInfo on CliCommand {
  String get frbVersion => "2.7.0";
  String get packageName => "simple_audio_flutter";

  String get projectRootDirectory => _projectRootDirectory;
  static String _projectRootDirectory = "";
  static void initProjectRootDirectory() {
    Directory directory = Directory.current;

    // Starts from the current directory and goes up until it finds the root project directory.
    while (true) {
      List<String> items = directory
          .listSync()
          .map((e) => e.uri.pathSegments[e.uri.pathSegments.length - 2])
          .toList();

      if (items.contains("simple_audio_flutter") &&
          items.contains("simple_audio")) {
        _projectRootDirectory = directory.path;
        break;
      } else {
        directory = directory.parent;
      }
    }
  }
}
