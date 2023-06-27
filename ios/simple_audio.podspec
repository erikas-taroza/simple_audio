# Download the binaries from GitHub.
version = "1.5.8"
lib_url = "https://github.com/erikas-taroza/simple_audio/blob/v#{version}/ios/Frameworks/simple_audio.xcframework"

`
mkdir Frameworks
cd Frameworks
if [ ! -d simple_audio.xcframework ]
then
  mkdir simple_audio.xcframework
  cd simple_audio.xcframework
  mkdir ios-arm64
  mkdir ios-arm64_x86_64-simulator
  curl -L "#{lib_url}/Info.plist?raw=true" -o Info.plist
  curl -L "#{lib_url}/ios-arm64/libsimple_audio.a?raw=true" -o ios-arm64/libsimple_audio.a
  curl -L "#{lib_url}/ios-arm64_x86_64-simulator/libsimple_audio.a?raw=true" -o ios-arm64_x86_64-simulator/libsimple_audio.a
fi
cd ../..
`

Pod::Spec.new do |s|
  s.name             = 'simple_audio'
  s.version          = '1.5.8'
  s.summary          = 'A simple cross-platform solution for playing audio in Flutter.'
  s.description      = <<-DESC
  A simple cross-platform solution for playing audio in Flutter.
                       DESC
  s.homepage         = 'https://github.com/erikas-taroza/simple_audio'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Erikas Taroza' => 'erikastaroza@gmail.com' }
  s.source           = { :path => '.' }
  s.source_files = 'Classes/**/*'
  s.dependency 'Flutter'
  s.platform = :ios, '9.0'
  s.vendored_frameworks = 'Frameworks/**/*.xcframework'
  s.static_framework = true
  # s.public_header_files = 'Classes/**/*.h'

  # Flutter.framework does not contain a i386 slice.
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386' }
  s.swift_version = '5.0'
end
