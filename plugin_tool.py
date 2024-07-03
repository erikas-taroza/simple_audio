import argparse, re, os, shutil, sys

FRB_VERSION = "1.82.1"
PACKAGE_NAME = "simple_audio_flutter"

parser = argparse.ArgumentParser(
    usage="Put this file in your root project directory and execute the commands.",
    description="A tool to help with building a Flutter plugin with Rust."
)

parser.add_argument(
    "-c", "--code-gen",
    action="store_true",
    help="Generates the FFI bridge using flutter_rust_bridge."
)

parser.add_argument(
    "-b", "--build",
    choices=["android", "linux", "windows", "ios", "macos"],
    nargs="*",
    help="Builds the Rust code. This will have to be run on Linux, Windows, and macOS if you want to target all platforms."
)

parser.add_argument(
    "--bump-version",
    type=str,
    help="Bumps the version of the plugin to the given version. Ex. \"1.0.0\""
)

def code_gen():
    print("Generating code with flutter_rust_bridge...\n")

    os.system(f"cargo install flutter_rust_bridge_codegen --version {FRB_VERSION}")
    os.system("cargo install cargo-expand --version 1.0.70")
    os.system(f'cd {PACKAGE_NAME} && flutter_rust_bridge_codegen \
        --dart-enums-style \
        --rust-input ./rust/src/api.rs \
        --dart-output ./lib/src/bridge_generated.dart \
        --dart-decl-output ./lib/src/bridge_definitions.dart \
        --c-output ./rust/src/bridge_generated.h')

    shutil.copyfile(f"{PACKAGE_NAME}/rust/src/bridge_generated.h", f"{PACKAGE_NAME}/ios/Classes/bridge_generated.h")
    shutil.move(f"{PACKAGE_NAME}/rust/src/bridge_generated.h", f"{PACKAGE_NAME}/macos/Classes/bridge_generated.h")

    # Fix the incorrect import in the generated file.
    # This happens because we are using lib.rs as the entry point.
    generated_text = open(f"{PACKAGE_NAME}/rust/src/bridge_generated.rs", "r").read()
    open(f"{PACKAGE_NAME}/rust/src/bridge_generated.rs", "w").write(generated_text.replace("use crate::lib::*;", "use crate::*;"))

    if "ffi.dart" not in os.listdir(f"{PACKAGE_NAME}/lib/src"):
        package_name = open(f"{PACKAGE_NAME}/rust/Cargo.toml", "r").read().split("name = \"")[1].split("\"")[0]
        pascal_case_package_name = package_name.lower().replace("_", " ").title().replace(" ", "")
        import requests
        file = open(f"{PACKAGE_NAME}/lib/src/ffi.dart", "w")
        file.write(
            requests.get(r"https://raw.githubusercontent.com/Desdaemon/flutter_rust_bridge_template/main/lib/ffi.dart")
                .text
                .replace("native", package_name)
                .replace("Native", pascal_case_package_name)
        )


def build(targets: list[str]):
    print("Building Rust code...\n")

    is_linux = sys.platform == "linux"
    is_windows = sys.platform == "win32"
    is_mac = sys.platform == "darwin"

    if (is_linux or is_windows or is_mac) and "android" in targets:
        print("Building Android libraries...\n")

        os.system("rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android")
        os.system("cargo install cargo-ndk")

        architectures = ["arm64-v8a", "armeabi-v7a", "x86", "x86_64"]
        for architecture in architectures:
            path = f"{PACKAGE_NAME}/android/src/main/jniLibs/{architecture}/lib{PACKAGE_NAME}.so"
            if os.path.exists(path):
                os.remove(path)

        result = os.system(f"cargo ndk -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 -o {PACKAGE_NAME}/android/src/main/jniLibs build --release")
        assert result == 0

    if is_linux and "linux" in targets:
        print("Building Linux libraries...\n")

        os.system("rustup target add x86_64-unknown-linux-gnu")
        result = os.system("cargo build --release --target x86_64-unknown-linux-gnu")
        assert result == 0

        if os.path.exists(f"{PACKAGE_NAME}/linux/lib{PACKAGE_NAME}.so"):
            os.remove(f"{PACKAGE_NAME}/linux/lib{PACKAGE_NAME}.so")

        shutil.move(f"target/x86_64-unknown-linux-gnu/release/lib{PACKAGE_NAME}.so", f"{PACKAGE_NAME}/linux")

    if is_windows and "windows" in targets:
        print("Building Windows libraries...\n")

        os.system("rustup target add x86_64-pc-windows-msvc")
        result = os.system("cargo build --release --target x86_64-pc-windows-msvc")
        assert result == 0

        if os.path.exists(f"{PACKAGE_NAME}/windows/{PACKAGE_NAME}.dll"):
            os.remove(f"{PACKAGE_NAME}/windows/{PACKAGE_NAME}.dll")

        shutil.move(f"target/x86_64-pc-windows-msvc/release/{PACKAGE_NAME}.dll", f"{PACKAGE_NAME}/windows")

    if is_mac:
        if "macos" in targets:
            print("Building macOS libraries...\n")

            # Build for macOS.
            os.system("rustup target add aarch64-apple-darwin x86_64-apple-darwin")
            result = os.system("cargo build --release --target aarch64-apple-darwin")
            assert result == 0
            result = os.system("cargo build --release --target x86_64-apple-darwin")
            assert result == 0
            os.system(f'lipo "target/aarch64-apple-darwin/release/lib{PACKAGE_NAME}.a" "target/x86_64-apple-darwin/release/lib{PACKAGE_NAME}.a" -output "lib{PACKAGE_NAME}.a" -create')

            if os.path.exists(f"{PACKAGE_NAME}/macos/Libs/lib{PACKAGE_NAME}.a"):
                os.remove(f"{PACKAGE_NAME}/macos/Libs/lib{PACKAGE_NAME}.a")

            os.makedirs(f"{PACKAGE_NAME}/macos/Libs", exist_ok=True)
            shutil.move(f"./lib{PACKAGE_NAME}.a", f"{PACKAGE_NAME}/macos/Libs")

        if "ios" in targets:
            # Build for iOS
            print("Building iOS libraries...\n")

            os.system("rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios")
            result = os.system("cargo build --release --target aarch64-apple-ios")
            assert result == 0

            result = os.system(f"cargo build --release --target aarch64-apple-ios-sim")
            assert result == 0

            result = os.system("CMAKE_OSX_SYSROOT=$(xcrun --sdk iphonesimulator --show-sdk-path) cargo build --release --target x86_64-apple-ios")
            assert result == 0
            os.system(f'lipo "target/aarch64-apple-ios-sim/release/lib{PACKAGE_NAME}.a" "target/x86_64-apple-ios/release/lib{PACKAGE_NAME}.a" -output "lib{PACKAGE_NAME}.a" -create')
            os.system(f"xcodebuild -create-xcframework -library target/aarch64-apple-ios/release/lib{PACKAGE_NAME}.a -library ./lib{PACKAGE_NAME}.a -output {PACKAGE_NAME}.xcframework")
            os.remove(f"./lib{PACKAGE_NAME}.a")

            if os.path.exists(f"{PACKAGE_NAME}/ios/Frameworks/{PACKAGE_NAME}.xcframework"):
                shutil.rmtree(f"{PACKAGE_NAME}/ios/Frameworks/{PACKAGE_NAME}.xcframework")

            os.makedirs(f"{PACKAGE_NAME}/ios/Frameworks", exist_ok=True)
            shutil.move(f"./{PACKAGE_NAME}.xcframework", f"{PACKAGE_NAME}/ios/Frameworks")


def bump_version(version: str):
    def replace_string_in_file(file, regex, replacement):
        new_text = re.sub(regex, replacement, file.read(), count=1)
        file.seek(0)
        file.write(new_text)
        file.seek(0)


    # pubspec.yaml
    pubspec = open("simple_audio_flutter/pubspec.yaml", "r+")
    replace_string_in_file(pubspec, r"version: [\d|\.]+\s", f"version: {version}\n")
    pubspec.close()

    # Cargo.toml
    cargo = open("simple_audio_flutter/rust/Cargo.toml", "r+")
    replace_string_in_file(cargo, r'version = "[\d|\.]+"\s', f'version = "{version}"\n')
    cargo.close()

    # Cargo.toml in main Rust project
    cargo = open("simple_audio/Cargo.toml", "r+")
    replace_string_in_file(cargo, r'version = "[\d|\.]+"\s', f'version = "{version}"\n')
    cargo.close()

    # Android CMake
    android_cmake = open("simple_audio_flutter/android/CMakeLists.txt", "r+")
    replace_string_in_file(android_cmake, r'set\(Version "[\d|\.]+"\)\s', f'set(Version "{version}")\n')
    android_cmake.close()

    # Linux CMake
    linux_cmake = open("simple_audio_flutter/linux/CMakeLists.txt", "r+")
    replace_string_in_file(linux_cmake, r'set\(Version "[\d|\.]+"\)\s', f'set(Version "{version}")\n')
    linux_cmake.close()

    # Windows CMake
    windows_cmake = open("simple_audio_flutter/windows/CMakeLists.txt", "r+")
    replace_string_in_file(windows_cmake, r'set\(Version "[\d|\.]+"\)\s', f'set(Version "{version}")\n')
    windows_cmake.close()

    pubspec_text = open("simple_audio_flutter/pubspec.yaml", "r").read()
    package_name = pubspec_text.split("name: ")[1].split("\n")[0].strip()

    # macOS podspec
    macos_podspec = open(f"simple_audio_flutter/macos/{package_name}.podspec", "r+")
    replace_string_in_file(macos_podspec, r'version = "[\d|\.]+"\s', f'version = "{version}"\n')
    replace_string_in_file(macos_podspec, r"s\.version\s+= '[\d|\.]+'\s", f"s.version          = '{version}'\n")
    macos_podspec.close()

    # iOS podspec
    ios_podspec = open(f"simple_audio_flutter/ios/{package_name}.podspec", "r+")
    replace_string_in_file(ios_podspec, r'version = "[\d|\.]+"\s', f'version = "{version}"\n')
    replace_string_in_file(ios_podspec, r"s\.version\s+= '[\d|\.]+'\s", f"s.version          = '{version}'\n")
    ios_podspec.close()


if __name__ == "__main__":
    args = parser.parse_args()

    if args.code_gen:
        code_gen()

    if args.build != None:
        targets = args.build
        if len(args.build) == 0:
            targets = ["android", "linux", "windows", "ios", "macos"]
        build(targets)

    if args.bump_version:
        bump_version(args.bump_version)