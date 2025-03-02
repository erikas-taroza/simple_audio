import 'dart:io';

import '../cli_command.dart';

mixin PackageInfo on CliCommand {
  String get frbVersion => "2.7.0";
  String get packageName => "simple_audio_flutter";

  String get projectRootDirectory => _projectRootDirectory;
  static String _projectRootDirectory = "";
  static void initProjectRootDirectory() {
    Uri path = Uri.parse(Directory.current.path);

    // Starts from the current directory and goes up until it finds the root project directory.
    while (true) {
      List<String> items = Directory.fromUri(path)
          .listSync()
          .map((e) => e.uri.pathSegments.last)
          .toList();

      if (items.contains("pubspec.yaml") && items.contains("Cargo.toml")) {
        _projectRootDirectory = path.toString();
        break;
      } else {
        path = path.replace(pathSegments: path.pathSegments..removeLast());
      }
    }
  }
}
