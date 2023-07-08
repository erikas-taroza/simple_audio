cmake_minimum_required(VERSION 3.10)

# Download the binaries from GitHub.
set(Version "1.6.0")
set(LibPath "${CMAKE_CURRENT_SOURCE_DIR}/src/main/jniLibs")

if(NOT EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/android.zip")
  file(DOWNLOAD
    "https://github.com/erikas-taroza/simple_audio/releases/download/v${Version}/android.zip"
    "${CMAKE_CURRENT_SOURCE_DIR}/android.zip"
    TLS_VERIFY ON
  )

  file(ARCHIVE_EXTRACT INPUT android.zip DESTINATION ${LibPath})
endif()