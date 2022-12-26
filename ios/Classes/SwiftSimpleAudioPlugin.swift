import Flutter
import UIKit

var simpleAudio:SimpleAudio? = nil

public class SwiftSimpleAudioPlugin: NSObject, FlutterPlugin
{
    public static func register(with registrar: FlutterPluginRegistrar)
    {
        let channel = FlutterMethodChannel(name: "simple_audio", binaryMessenger: registrar.messenger())
        let instance = SwiftSimpleAudioPlugin()
        registrar.addMethodCallDelegate(instance, channel: channel)
        
        let _ = dummy_method_to_enforce_bundling()
        simpleAudio = SimpleAudio(channel: channel)
    }

    public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult)
    {
        simpleAudio!.handle(call, result: result)
    }
}
