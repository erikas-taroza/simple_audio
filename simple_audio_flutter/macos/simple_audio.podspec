# Download the binary from GitHub.
version = "2.0.1"
lib_url = "https://github.com/erikas-taroza/simple_audio/releases/download/v#{version}/macos.zip"

`
mkdir Libs
cd Libs
if [ ! -f macos.zip ]
then
  curl -L "#{lib_url}" -o macos.zip
  unzip macos.zip
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
  s.source_files     = 'Classes/**/*'
  s.dependency 'FlutterMacOS'
  s.vendored_libraries = 'Libs/**/*'

  s.platform = :osx, '10.11'
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES' }
  s.swift_version = '5.0'
end
