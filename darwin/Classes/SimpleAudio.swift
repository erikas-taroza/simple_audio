#if os(macOS)
import FlutterMacOS
#else
import Flutter
#endif

import MediaPlayer

@available(macOS 10.12.2, *)
public class SimpleAudio
{
    var channel:FlutterMethodChannel
    var currentMetadata:[String: Any] = [:]

    init(channel:FlutterMethodChannel?)
    {
        self.channel = channel!
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
                    artUri: args["artUri"] as? String,
                    duration: args["duration"] as? Int
                )
            case "setPlaybackState":
                let args:[String: Any] = call.arguments as! [String: Any]
            
                setPlaybackState(
                    state: args["state"] as? Int,
                    position: args["position"] as? Int
                )
            default:
                result(FlutterMethodNotImplemented)
        }
    }
    
    func initialize()
    {
        #if os(iOS)
        let session = AVAudioSession.sharedInstance()
        try! session.setCategory(.playback)
        try! session.setActive(true)
        #endif
        
        let commandCenter = MPRemoteCommandCenter.shared()
        
        commandCenter.playCommand.isEnabled = true
        commandCenter.playCommand.addTarget { event in
            self.channel.invokeMethod("play", arguments: nil)
            return .success
        }
        
        commandCenter.pauseCommand.isEnabled = true
        commandCenter.pauseCommand.addTarget { event in
            self.channel.invokeMethod("pause", arguments: nil)
            return .success
        }
        
        commandCenter.previousTrackCommand.isEnabled = true
        commandCenter.previousTrackCommand.addTarget { event in
            self.channel.invokeMethod("previous", arguments: nil)
            return .success
        }
        
        commandCenter.nextTrackCommand.isEnabled = true
        commandCenter.nextTrackCommand.addTarget { event in
            self.channel.invokeMethod("next", arguments: nil)
            return .success
        }
        
        commandCenter.skipForwardCommand.isEnabled = true
        commandCenter.skipForwardCommand.preferredIntervals = [10.0]
        commandCenter.skipForwardCommand.addTarget { event in
            self.channel.invokeMethod("seekRelative", arguments: true)
            return .success
        }
        
        commandCenter.skipBackwardCommand.isEnabled = true
        commandCenter.skipBackwardCommand.preferredIntervals = [10.0]
        commandCenter.skipBackwardCommand.addTarget { event in
            self.channel.invokeMethod("seekRelative", arguments: false)
            return .success
        }
        
        commandCenter.changePlaybackPositionCommand.isEnabled = true
        commandCenter.changePlaybackPositionCommand.addTarget { event in
            let seconds = (event as! MPChangePlaybackPositionCommandEvent).positionTime
            self.channel.invokeMethod("seek", arguments: Int(seconds.rounded(.down)))
            return .success
        }
        
        #if os(macOS)
        let nowPlaying = MPNowPlayingInfoCenter.default()
        nowPlaying.playbackState = MPNowPlayingPlaybackState.unknown
        #endif
    }
    
    func setMetadata(
        title:String?,
        artist:String?,
        album:String?,
        artUri:String?,
        duration:Int?
    )
    {
        currentMetadata = [
            MPNowPlayingInfoPropertyDefaultPlaybackRate: 1.0,
            MPMediaItemPropertyMediaType: MPNowPlayingInfoMediaType.audio,
            MPMediaItemPropertyTitle: title ?? "Unknown Title",
            MPMediaItemPropertyArtist: artist ?? "Unknown Artist",
            MPMediaItemPropertyAlbumTitle: album ?? "Unknown Album",
            MPMediaItemPropertyPlaybackDuration: String(duration ?? 0),
        ]
        
        if(artUri != nil)
        {
            #if os(macOS)
            if #available(macOS 10.13.2, *) {
                let size = CGSize(width: 200, height: 200)
                let artwork = MPMediaItemArtwork(boundsSize: size, requestHandler: { size in
                    return NSImage(contentsOf: URL(string: artUri!)!)!
                })
                currentMetadata[MPMediaItemPropertyArtwork] = artwork
            }
            #else
            let size = CGSize(width: 200, height: 200)
            let artwork = MPMediaItemArtwork(boundsSize: size, requestHandler: { size in
                let data = try! Data(contentsOf: URL(string: artUri!)!)
                return UIImage(data: data)!
            })
            currentMetadata[MPMediaItemPropertyArtwork] = artwork
            #endif
        }
        
        let state = MPNowPlayingInfoCenter.default()
        state.nowPlayingInfo = currentMetadata
    }
    
    // See enum type PlaybackState in simple_audio.dart for integer values.
    func setPlaybackState(state:Int?, position:Int?)
    {
        let nowPlaying = MPNowPlayingInfoCenter.default()
        
        #if os(macOS)
        var translatedState:MPNowPlayingPlaybackState
        
        switch(state)
        {
            case 0: translatedState = MPNowPlayingPlaybackState.playing
            case 1: translatedState = MPNowPlayingPlaybackState.paused
            case 2: translatedState = MPNowPlayingPlaybackState.stopped
            default: translatedState = MPNowPlayingPlaybackState.unknown
        }
        
        nowPlaying.playbackState = translatedState
        #else
        if state == 0
        {
            currentMetadata[MPNowPlayingInfoPropertyPlaybackRate] = 1.0
            currentMetadata[MPMediaItemPropertyMediaType] = MPNowPlayingInfoMediaType.audio
        }
        else
        {
            currentMetadata[MPNowPlayingInfoPropertyPlaybackRate] = 0.0
            currentMetadata[MPMediaItemPropertyMediaType] = MPNowPlayingInfoMediaType.none
        }
        #endif
        
        currentMetadata[MPNowPlayingInfoPropertyElapsedPlaybackTime] = String(position ?? 0)
        nowPlaying.nowPlayingInfo = currentMetadata
    }
}
