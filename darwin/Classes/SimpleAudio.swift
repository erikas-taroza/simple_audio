// This file is a part of simple_audio
// Copyright (c) 2022 Erikas Taroza <erikastaroza@gmail.com>
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of 
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program.
// If not, see <https://www.gnu.org/licenses/>.

#if os(macOS)
import FlutterMacOS
#else
import Flutter
import AVFoundation
#endif

import MediaPlayer

@available(macOS 10.12.2, *)
public class SimpleAudio
{
    var channel:FlutterMethodChannel
    var currentMetadata:[String: Any] = [:]
    
    var showMediaNotification:Bool = true

    init(channel:FlutterMethodChannel?)
    {
        self.channel = channel!
    }

    public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult)
    {
        switch call.method
        {
            case "init":
                let args:[String: Any] = call.arguments as! [String: Any]
            
                showMediaNotification = args["showMediaNotification"] as! Bool
                if(!showMediaNotification) { return }

                let actions = args["actions"] as! [Int]
                let preferSkipButtons = args["preferSkipButtons"] as! Bool
            
                initialize(actions: actions.map { try! Actions.fromInt(i: $0) }, preferSkipButtons: preferSkipButtons)
            case "setMetadata":
                let args:[String: Any] = call.arguments as! [String: Any]
                    
                if(!showMediaNotification) { return }
            
                setMetadata(
                    title: args["title"] as? String,
                    artist: args["artist"] as? String,
                    album: args["album"] as? String,
                    artUri: args["artUri"] as? String,
                    artBytes: args["artBytes"] as? FlutterStandardTypedData,
                    duration: args["duration"] as? Int
                )
            case "setPlaybackState":
                let args:[String: Any] = call.arguments as! [String: Any]
                    
                if(!showMediaNotification) { return }
            
                setPlaybackState(
                    state: args["state"] as? Int,
                    position: args["position"] as? Int
                )
            default:
                result(FlutterMethodNotImplemented)
        }
    }
    
    func initialize(actions:[Actions], preferSkipButtons:Bool)
    {
        #if os(iOS)
        let session = AVAudioSession.sharedInstance()
        try! session.setCategory(.playback)
        try! session.setActive(true)
        #endif
        
        let commandCenter = MPRemoteCommandCenter.shared()
        
        if(actions.contains(Actions.playPause))
        {
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
        }
        
        if(actions.contains(Actions.skipPrev))
        {
            commandCenter.previousTrackCommand.isEnabled = true
            commandCenter.previousTrackCommand.addTarget { event in
                self.channel.invokeMethod("previous", arguments: nil)
                return .success
            }
        }
        
        if(actions.contains(Actions.skipNext))
        {
            commandCenter.nextTrackCommand.isEnabled = true
            commandCenter.nextTrackCommand.addTarget { event in
                self.channel.invokeMethod("next", arguments: nil)
                return .success
            }
        }
        
        if(actions.contains(Actions.fastForward) && !preferSkipButtons)
        {
            commandCenter.skipForwardCommand.isEnabled = true
            commandCenter.skipForwardCommand.preferredIntervals = [10.0]
            commandCenter.skipForwardCommand.addTarget { event in
                self.channel.invokeMethod("seekRelative", arguments: true)
                return .success
            }
        }
        
        if(actions.contains(Actions.rewind) && !preferSkipButtons)
        {
            commandCenter.skipBackwardCommand.isEnabled = true
            commandCenter.skipBackwardCommand.preferredIntervals = [10.0]
            commandCenter.skipBackwardCommand.addTarget { event in
                self.channel.invokeMethod("seekRelative", arguments: false)
                return .success
            }
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
        artBytes:FlutterStandardTypedData?,
        duration:Int?
    )
    {
        currentMetadata = [
            MPNowPlayingInfoPropertyDefaultPlaybackRate: 1.0,
            MPMediaItemPropertyTitle: title ?? "Unknown Title",
            MPMediaItemPropertyArtist: artist ?? "Unknown Artist",
            MPMediaItemPropertyAlbumTitle: album ?? "Unknown Album",
            MPMediaItemPropertyPlaybackDuration: duration ?? 0,
        ]
        
        if(artUri != nil || artBytes != nil)
        {
            let size = CGSize(width: 200, height: 200)
            
            #if os(macOS)
            if #available(macOS 10.13.2, *) {
                let artwork = MPMediaItemArtwork(boundsSize: size, requestHandler: { size in
                    return NSImage(contentsOf: URL(string: artUri!)!)!
                })
                currentMetadata[MPMediaItemPropertyArtwork] = artwork
            }
            #else
            let artwork = MPMediaItemArtwork(boundsSize: size, requestHandler: { size in
                if(artUri != nil) {
                    let data = try! Data(contentsOf: URL(string: artUri!)!)
                    return UIImage(data: data)!
                }
                else if(artBytes != nil) {
                    return UIImage(data: artBytes!.data)!
                }
                
                return UIImage()
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
        let session = AVAudioSession.sharedInstance()

        if state == 0
        {
            currentMetadata[MPNowPlayingInfoPropertyPlaybackRate] = 1.0
            try? session.setActive(true)
        }
        else
        {
            currentMetadata[MPNowPlayingInfoPropertyPlaybackRate] = 0.0
            // Allow some time for the Rust code to execute
            // to pause the stream.
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.05) {
                // setActive will throw an error (and stop playback) if
                // it is given `false` when audio is still playing.
                // Call this only if there is nothing playing.
                self.channel.invokeMethod("isPlaying", arguments: nil) { res in
                    if(res is Bool && !(res as! Bool)) {
                        try? session.setActive(false)
                    }
                }
            }
        }
        #endif
        
        currentMetadata[MPNowPlayingInfoPropertyElapsedPlaybackTime] = String(position ?? 0)
        nowPlaying.nowPlayingInfo = currentMetadata
    }
}

enum Actions
{
    case rewind
    case skipPrev
    case playPause
    case skipNext
    case fastForward
                
    static func fromInt(i:Int) throws -> Actions
    {
        switch(i)
        {
            case 0: return .rewind
            case 1: return .skipPrev
            case 2: return .playPause
            case 3: return .skipNext
            case 4: return .fastForward
            default: throw InvalidActionError.invalid
        }
    }
}

enum InvalidActionError:Error
{
    case invalid
}
