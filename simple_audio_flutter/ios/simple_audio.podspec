# Download the binaries from GitHub.
version = "2.0.1"
lib_url = "https://github.com/erikas-taroza/simple_audio/releases/download/v#{version}/ios.zip"

`
mkdir Frameworks
cd Frameworks
if [ ! -d ios.zip ]
then
  curl -L "#{lib_url}" -o ios.zip
  unzip ios.zip -d 'simple_audio.xcframework'
fi
cd ..
`

Pod::Spec.new do |s|
  s.name             = 'simple_audio'
  s.version          = '2.0.1'
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
