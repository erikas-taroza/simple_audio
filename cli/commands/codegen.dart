import 'dart:io';

import 'package:io/io.dart';
import 'package:yaml/yaml.dart';
import 'package:yaml_edit/yaml_edit.dart';

import '../cli_command.dart';
import '../mixins/cli_logger.dart';
import '../mixins/package_info.dart';
import '../mixins/process_runner.dart';

class CodegenCommand extends CliCommand
    with CliLogger, ProcessRunner, PackageInfo {
  @override
  String get name => "codegen";

  @override
  List<String> get aliases => ["c"];

  @override
  String get description =>
      "Generates the FFI bridge using flutter_rust_bridge.";

  @override
  Future<int> run() async {
    logger.stdout("Preparing...");

    final File pubspec =
        File("$projectRootDirectory/$packageName/pubspec.yaml");
    final YamlEditor yamlEditor = YamlEditor(await pubspec.readAsString());
    final YamlMap originalDependencies =
        yamlEditor.parseAt(["dependencies"]).value;

    // Adds the build dependency. This cannot be included in a published package because of the local path.
    final Map addedDependency = Map.from(originalDependencies.value)
      ..addAll(
        {
          "rust_lib_simple_audio": {"path": "rust_builder"},
        },
      );
    yamlEditor.update(
      ["dependencies"],
      YamlMap.wrap(addedDependency),
    );
    logger.trace("Writing to yaml:\n${yamlEditor.toString()}");
    await pubspec.writeAsString(yamlEditor.toString(), flush: true);

    // Install FRB
    int result = await runProcess(
      "cargo",
      ["install", "flutter_rust_bridge_codegen", "--version", frbVersion],
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    logger.stdout("Generating code with flutter_rust_bridge ($frbVersion)...");
    result = await runProcess(
      "flutter_rust_bridge_codegen",
      [
        "generate",
        "--config-file",
        "$projectRootDirectory/$packageName/flutter_rust_bridge.yaml",
      ],
    );

    if (result != ExitCode.success.code) {
      return result;
    }

    logger.stdout("Cleaning up...");
    // Remove the build dependency.
    yamlEditor.update(
      ["dependencies"],
      originalDependencies,
    );
    logger.trace("Writing to yaml:\n${yamlEditor.toString()}");
    await pubspec.writeAsString(yamlEditor.toString(), flush: true);

    logger.stdout("Done!");
    return ExitCode.success.code;
  }
}
