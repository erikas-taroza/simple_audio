#import "SimpleAudioPlugin.h"
#if __has_include(<simple_audio/simple_audio-Swift.h>)
#import <simple_audio/simple_audio-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "simple_audio-Swift.h"
#endif

@implementation SimpleAudioPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftSimpleAudioPlugin registerWithRegistrar:registrar];
}
@end
