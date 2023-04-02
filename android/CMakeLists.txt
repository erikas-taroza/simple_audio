cmake_minimum_required(VERSION 3.10)

# Download the binaries from GitHub.
set(Version "1.5.1")
set(LibPath "${CMAKE_CURRENT_SOURCE_DIR}/src/main/jniLibs")

if(NOT EXISTS "${LibPath}/arm64-v8a/libsimple_audio.so")
  file(DOWNLOAD
    "https://github.com/erikas-taroza/simple_audio/blob/v${Version}/android/src/main/jniLibs/arm64-v8a/libsimple_audio.so?raw=true"
    "${LibPath}/arm64-v8a/libsimple_audio.so"
    TLS_VERIFY ON
  )

  file(DOWNLOAD
    "https://github.com/erikas-taroza/simple_audio/blob/v${Version}/android/src/main/jniLibs/armeabi-v7a/libsimple_audio.so?raw=true"
    "${LibPath}/armeabi-v7a/libsimple_audio.so"
    TLS_VERIFY ON
  )

  file(DOWNLOAD
    "https://github.com/erikas-taroza/simple_audio/blob/v${Version}/android/src/main/jniLibs/x86/libsimple_audio.so?raw=true"
    "${LibPath}/x86/libsimple_audio.so"
    TLS_VERIFY ON
  )

  file(DOWNLOAD
    "https://github.com/erikas-taroza/simple_audio/blob/v${Version}/android/src/main/jniLibs/x86_64/libsimple_audio.so?raw=true"
    "${LibPath}/x86_64/libsimple_audio.so"
    TLS_VERIFY ON
  )
endif()