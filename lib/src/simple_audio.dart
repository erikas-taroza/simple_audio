export 'bridge_definitions.dart' show ProgressState, Metadata;

import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import './ffi.dart';

late final Player _player;

class SimpleAudio
{
    static MethodChannel methodChannel = const MethodChannel("simple_audio");

    // Maybe subscribe to this stream for native media notifications.
    late Stream<PlaybackState> playbackStateStream = Player.playbackStateStream(bridge: api)
        .map((event) => PlaybackState.values[event]).asBroadcastStream(); // Map the int event to a dart enum.
    late Stream<ProgressState> progressStateStream = Player.progressStateStream(bridge: api).asBroadcastStream();

    Future<bool> get isPlaying => _player.isPlaying();
    Future<ProgressState> get _progress => _player.getProgress();

    bool get _usingNative => Platform.isAndroid || Platform.isIOS || Platform.isMacOS;

    void Function()? onPreviousCallback;
    void Function()? onNextCallback;

    /// **[onPreviousCallback]** Callback for when the user wants to skip to the previous media
    /// (via a media notification).
    /// 
    /// **[onNextCallback]** Callback for when the user wants to skip to the next media
    /// (via a media notification).
    SimpleAudio({
        this.onPreviousCallback,
        this.onNextCallback
    })
    {
        Player.metadataCallbackStream(bridge: api).listen((event) {
            if(!event) { onPreviousCallback?.call(); }
            else { onNextCallback?.call(); }
        });
        
        methodChannel.setMethodCallHandler((call) async {
            switch(call.method)
            {
                case "play":
                    play();
                    break;
                case "pause":
                    pause();
                    break;
                case "stop":
                    stop();
                    break;
                case "next":
                    onNextCallback?.call();
                    break;
                case "previous":
                    onPreviousCallback?.call();
                    break;
                case "seekRelative":
                    int position = (await _progress).position;
                    int seekTo = call.arguments as bool ?
                        position + 10
                        : (position - 10).clamp(0, double.maxFinite).toInt();

                    seek(seekTo);
                    break;
                case "seek":
                    seek(call.arguments);
                    break;
            }
        });
    }

    /// Initialize [SimpleAudio]. This should only be done once in the `main` method.
    /// 
    /// **[mprisName]** The name of the MPRIS metadata handler. The name has to follow these requirements:
    /// - Be less than or equal to 255 characters in length.
    /// - Cannot start with a number.
    /// - Can only contain these characters: "[A-Z][a-z][0-9]_"
    /// 
    /// MPRIS is a D-Bus interface for controlling media players. See: https://wiki.archlinux.org/title/MPRIS
    /// 
    /// **[showMediaNotification]** Whether or not to show the media notification when playing
    /// audio.
    /// 
    /// **[actions]** A list of actions that the media notification will use.
    /// If [showMediaNotification] is false, this value does not matter. Otherwise, you will
    /// need to include [NotificationActions.playPause] in the list.
    /// 
    /// **[androidNotificationIconPath]** A path that points to the icon the Android media
    /// notification will use. This icon should be stored in `./android/app/src/main/res/mipmap-xxx`.
    /// You will want to create images that are sized accordingly to the pixel density.
    /// The icon should also be monochrome so that it renders properly. To ensure that this
    /// icon is bundled properly, see https://developer.android.com/studio/build/shrink-code#keep-resources
    /// 
    /// To create an icon, see: https://developer.android.com/studio/write/image-asset-studio#create-adaptive
    /// 
    /// **[androidCompactPlaybackActions]** A list of numbers that represent the buttons
    /// to show in the compact media notification. The indicies match with the ones
    /// in [actions].
    /// 
    /// For example, to use the middle 3:
    /// 
    /// androidPlaybackActions = [Rewind, SkipPrev, PlayPause, SkipNext, FastForward]
    /// 
    /// androidCompactPlaybackActions = [1, 2, 3]
    /// 
    /// **[applePreferSkipButtons]** For the macOS and iOS media notifications. If set to true,
    /// the notification will show [NotificationActions.skipPrev] and [NotificationActions.skipNext]
    /// instead of [NotificationActions.rewind] and [NotificationActions.fastForward]
    /// when all 4 values are provided in [actions].
    static Future<void> init({
        String mprisName = "SimpleAudio",
        bool showMediaNotification = true,
        List<NotificationActions> actions = NotificationActions.values,
        String androidNotificationIconPath = "mipmap/ic_launcher",
        List<int> androidCompactPlaybackActions = const [1, 2, 3],
        bool applePreferSkipButtons = true
    }) async
    {
        // You must include this action.
        if(showMediaNotification) assert(actions.contains(NotificationActions.playPause));

        _player = await Player.newPlayer(
            bridge: api,
            actions: showMediaNotification ?
                Int32List.fromList(actions.map((e) => e.index).toList())
                : Int32List(0),
            mprisName: mprisName,
            hwnd: Platform.isWindows ? getHWND() : null
        );

        if(Platform.isAndroid)
        {
            methodChannel.invokeMethod("init", {
                "showMediaNotification": showMediaNotification,
                "actions": actions.map((e) => e.index).toList(),
                "compactActions": androidCompactPlaybackActions,
                "icon": androidNotificationIconPath
            });
        }
        else if(Platform.isIOS || Platform.isMacOS)
        {
            methodChannel.invokeMethod("init", {
                "showMediaNotification": showMediaNotification,
                "actions": actions.map((e) => e.index).toList(),
                "preferSkipButtons": applePreferSkipButtons
            });
        }
    }

    Future<void> open(String path, [bool autoplay = true]) async
    {
        await _player.open(path: path, autoplay: autoplay);

        if(_usingNative && autoplay)
        {
            methodChannel.invokeMethod("setPlaybackState", {
                "state": PlaybackState.play.index,
                "position": 0
            });
        }
    }

    Future<void> play() async
    {
        if(_usingNative)
        {
            methodChannel.invokeMethod("setPlaybackState", {
                "state": PlaybackState.play.index,
                "position": (await _progress).position
            });
        }

        return await _player.play();
    }

    Future<void> pause() async
    {
        if(_usingNative)
        {
            methodChannel.invokeMethod("setPlaybackState", {
                "state": PlaybackState.pause.index,
                "position": (await _progress).position
            });
        }

        return await _player.pause();
    }

    Future<void> stop() async
    {
        if(_usingNative)
        {
            methodChannel.invokeMethod("setPlaybackState", {
                "state": -1,
                "position": 0
            });
        }

        return await _player.stop();
    }

    Future<void> setVolume(double volume) async
    {
        return await _player.setVolume(volume: volume);
    }

    Future<void> seek(int seconds) async
    {
        if(_usingNative)
        {
            methodChannel.invokeMethod("setPlaybackState", {
                "state": (await isPlaying ? PlaybackState.play : PlaybackState.pause).index,
                "position": seconds
            });
        }

        return await _player.seek(seconds: seconds);
    }

    void setMetadata(Metadata metadata)
    {
        if(Platform.isLinux || Platform.isWindows)
        {
            _player.setMetadata(metadata: metadata);
        }
        else if(_usingNative)
        {
            // Prevent users from awaiting this method
            // and blocking their program infintely
            Future<void> _() async
            {
                // Wait for a valid duration.
                ProgressState progress = await _progress;
                while(progress.duration == 0)
                {
                    progress = await _progress;
                    continue;
                }

                await methodChannel.invokeMethod("setMetadata", {
                    "title": metadata.title,
                    "artist": metadata.artist,
                    "album": metadata.album,
                    "artUri": metadata.artUri,
                    "duration": progress.duration
                });
            }
            _();
        }
    }
}

enum PlaybackState
{
    play,
    pause,
    done
}

// NOTE: When updating the enum values, they need to be updated
// in Kotlin and in Rust.
enum NotificationActions
{
    rewind,
    skipPrev,
    playPause,
    skipNext,
    fastForward
}