import Cocoa
import FlutterMacOS
import MediaPlayer

var channel:FlutterMethodChannel? = nil

@available(macOS 10.12.2, *)
public class SimpleAudioPlugin: NSObject, FlutterPlugin
{
    public static func register(with registrar: FlutterPluginRegistrar)
    {
        channel = FlutterMethodChannel(name: "simple_audio", binaryMessenger: registrar.messenger)
        let instance = SimpleAudioPlugin()
        registrar.addMethodCallDelegate(instance, channel: channel!)
    }

    public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult)
    {
        switch call.method
        {
            case "init":
                initialize()
            case "setMetadata":
                let args:[String: Any] = call.arguments as! [String: Any]
            
                setMetadata(
                    title: args["title"] as? String,
                    artist: args["artist"] as? String,
                    album: args["album"] as? String,
                    artUrl: args["artUrl"] as? String,
                    duration: args["duration"] as? Int
                )
            case "setPlaybackState":
                print("")
            default:
                result(FlutterMethodNotImplemented)
        }
    }
    
    func initialize()
    {
        let commandCenter = MPRemoteCommandCenter.shared()
        
        commandCenter.playCommand.isEnabled = true
        commandCenter.playCommand.addTarget { event in
            channel!.invokeMethod("play", arguments: nil)
            return .success
        }
        
        commandCenter.pauseCommand.isEnabled = true
        commandCenter.pauseCommand.addTarget { event in
            channel!.invokeMethod("pause", arguments: nil)
            return .success
        }
        
        commandCenter.previousTrackCommand.isEnabled = true
        commandCenter.previousTrackCommand.addTarget { event in
            channel!.invokeMethod("previous", arguments: nil)
            return .success
        }
        
        commandCenter.nextTrackCommand.isEnabled = true
        commandCenter.nextTrackCommand.addTarget { event in
            channel!.invokeMethod("next", arguments: nil)
            return .success
        }
        
        commandCenter.skipForwardCommand.isEnabled = true
        commandCenter.skipForwardCommand.preferredIntervals = [10.0]
        commandCenter.skipForwardCommand.addTarget { event in
            channel!.invokeMethod("seekRelative", arguments: true)
            return .success
        }
        
        commandCenter.skipBackwardCommand.isEnabled = true
        commandCenter.skipBackwardCommand.preferredIntervals = [10.0]
        commandCenter.skipBackwardCommand.addTarget { event in
            channel!.invokeMethod("seekRelative", arguments: false)
            return .success
        }
        
        commandCenter.changePlaybackPositionCommand.isEnabled = true
        commandCenter.changePlaybackPositionCommand.addTarget { event in
            let seconds = (event as! MPChangePlaybackPositionCommandEvent).positionTime
            channel!.invokeMethod("seek", arguments: Int(seconds.rounded(.down)))
            return .success
        }
        
        let nowPlaying = MPNowPlayingInfoCenter.default()
        nowPlaying.playbackState = MPNowPlayingPlaybackState.playing
    }
    
    func setMetadata(
        title:String?,
        artist:String?,
        album:String?,
        artUrl:String?,
        duration:Int?
    )
    {
        var metadata = [
            MPMediaItemPropertyTitle: title ?? "Unknown Title",
            MPMediaItemPropertyArtist: artist ?? "Unknown Artist",
            MPMediaItemPropertyAlbumTitle: album ?? "Unknown Album",
            MPMediaItemPropertyPlaybackDuration: String(duration ?? 0)
        ]
        
        if #available(macOS 10.13.2, *) {
            //metadata[MPMediaItemPropertyArtwork] =
        }
        
        let nowPlaying = MPNowPlayingInfoCenter.default()
        nowPlaying.nowPlayingInfo = metadata
    }
}
