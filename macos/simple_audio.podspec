# Download the binary from GitHub.
version = "1.4.3"
lib_url = "https://github.com/erikas-taroza/simple_audio/blob/v#{version}/macos/Libs/libsimple_audio.a?raw=true"

`
mkdir Libs
cd Libs
if [ ! -f libsimple_audio.a ]
then
  curl -L "#{lib_url}" -o libsimple_audio.a
fi
cd ..
`

Pod::Spec.new do |s|
  s.name             = 'simple_audio'
  s.version          = '1.4.3'
  s.summary          = 'A simple cross-platform solution for playing audio in Flutter.'
  s.description      = <<-DESC
  A simple cross-platform solution for playing audio in Flutter.
                       DESC
  s.homepage         = 'https://github.com/erikas-taroza/simple_audio'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Erikas Taroza' => 'erikastaroza@gmail.com' }

  s.source           = { :path => '.' }
  s.source_files     = 'Classes/**/*'
  s.dependency 'FlutterMacOS'
  s.vendored_libraries = 'Libs/**/*'

  s.platform = :osx, '10.11'
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES' }
  s.swift_version = '5.0'
end
