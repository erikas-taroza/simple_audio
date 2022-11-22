import Cocoa
import FlutterMacOS
import MediaPlayer

@available(macOS 10.12.2, *)
public class SimpleAudioPlugin: NSObject, FlutterPlugin
{
    public static func register(with registrar: FlutterPluginRegistrar)
    {
        let channel = FlutterMethodChannel(name: "simple_audio", binaryMessenger: registrar.messenger)
        let instance = SimpleAudioPlugin()
        registrar.addMethodCallDelegate(instance, channel: channel)
        
        let commandCenter = MPRemoteCommandCenter.shared()
        commandCenter.changePlaybackPositionCommand.isEnabled = true
        
        commandCenter.playCommand.isEnabled = true
        commandCenter.playCommand.addTarget { event in
            print("Play pressed")
            
            return .success
        }
        
        commandCenter.pauseCommand.isEnabled = true
        commandCenter.pauseCommand.addTarget { event in
            print("Pause pressed")
            
            return .success
        }
        
        commandCenter.togglePlayPauseCommand.isEnabled = true
        commandCenter.togglePlayPauseCommand.addTarget { event in
            print("Play/Pause pressed")
            
            return .success
        }
        
        let nowPlaying = MPNowPlayingInfoCenter.default()
        nowPlaying.nowPlayingInfo = [MPMediaItemPropertyTitle: "Testing"]
    }

    public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult)
    {
        switch call.method {
            case "getPlatformVersion":
                result("macOS " + ProcessInfo.processInfo.operatingSystemVersionString)
            default:
                result(FlutterMethodNotImplemented)
        }
    }
}
