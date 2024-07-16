cmake_minimum_required(VERSION 3.10)

# Download the binaries from GitHub.
set(Version "2.0.1")
set(LibPath "${CMAKE_CURRENT_SOURCE_DIR}/src/main/jniLibs")
set(ArchivePath "${CMAKE_CURRENT_SOURCE_DIR}/android.tar.gz")

if(NOT EXISTS ${ArchivePath})
  file(DOWNLOAD
    "https://github.com/erikas-taroza/simple_audio/releases/download/v${Version}/android.tar.gz"
    ${ArchivePath}
    TLS_VERIFY ON
  )
endif()

if(NOT EXISTS "${LibPath}/arm64-v8a/libsimple_audio_flutter.so")
  execute_process(
    COMMAND ${CMAKE_COMMAND} -E tar -xzf ${ArchivePath}
    WORKING_DIRECTORY ${LibPath}
  )
endif()