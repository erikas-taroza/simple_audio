import Flutter
import UIKit

public class SwiftSimpleAudioPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(name: "simple_audio", binaryMessenger: registrar.messenger())
    let instance = SwiftSimpleAudioPlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)
    dummy_method_to_enforce_bundling()
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    result("iOS " + UIDevice.current.systemVersion)
  }
}
