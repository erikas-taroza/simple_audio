import argparse, re, os, shutil, requests, sys

parser = argparse.ArgumentParser(
    usage="Put this file in your root project directory and execute the commands.",
    description="A tool to help you with initializing and building a Flutter plugin with Rust."
)

parser.add_argument(
    "-i", "--init",
    action="store_true",
    help="Initialize the Flutter plugin project for development with Rust."
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

parser.add_argument(
    "--ios-ssl",
    action="store",
    help="Used to fix the build for OpenSSL if the vendored feature is being used on aarch64-apple-ios-sim target. Please provide the path to the openssl include directory."
)

def init():
    print("Initializing your project...")

    # Add dependencies to pubspec.yaml.
    print("Adding dependencies to pubspec.yaml...")

    pubspec_text = open("./pubspec.yaml", "r").read()
    package_name = pubspec_text.split("name: ")[1].split("\n")[0].strip()
    pascal_case_package_name = package_name.lower().replace("_", " ").title().replace(" ", "")
    
    with open("./pubspec.yaml", "w") as pubspec:
        def add_dependency(dependency:str, version:str = "", dev:bool = False) -> str:
            key = ("dev_" if dev else "") + "dependencies"
            split:list[str] = re.split(rf"\s{key}:\s", pubspec_text)
            lines:list[str] = split[1].split("\n")
            
            for i in range(0, len(lines)):
                if lines[i].isspace() or len(lines[i]) == 0:
                    break

            lines.insert(i, f"  {dependency}: {version}")

            return split[0] + f"\n{key}:\n" + "\n".join(lines)

        if "ffi:" not in pubspec_text:
            pubspec_text = add_dependency("ffi", version="^2.0.1", dev=False)

        if "flutter_rust_bridge:" not in pubspec_text:
            pubspec_text = add_dependency("flutter_rust_bridge", version="^1.71.0", dev=False)
        
        if "ffigen:" not in pubspec_text:
            pubspec_text = add_dependency("ffigen", version="^6.1.2", dev=True)

        pubspec.write(pubspec_text)

    # Start the Rust project.
    print(f"Creating the Rust project with the name \"{package_name}\"...")
    
    os.makedirs("rust", exist_ok=True)
    path = f"./rust/{package_name}"
    os.system(f"cargo new {path} --lib")

    for item in os.listdir(path):
        shutil.move(f"{path}/{item}", f"./rust")

    os.rmdir(path)

    toml_text = open("./rust/Cargo.toml", "r").read()
    with open("./rust/Cargo.toml", "w") as toml:
        split = toml_text.split("\n\n")
        toml_text = split[0] + '\n\n[lib]\ncrate-type = ["staticlib", "cdylib"]\n\n' + "\n".join(split[1:])
        toml.write(toml_text)

    # Initialize the Flutter platform specific things.
    print("Initializing platform specific project files...\n")
    
    # Android
    gradle_text = open("./android/build.gradle", "r").read()
    if "main.jniLibs.srcDirs = ['src/main/jniLibs']" not in gradle_text:
        with open("./android/build.gradle", "w") as gradle:
            split = gradle_text.split("sourceSets {")
            split[1] = "\t\tmain.jniLibs.srcDirs = ['src/main/jniLibs']" + split[1]
            gradle.write(split[0] + "sourceSets {\n" + split[1])

    # Linux
    linux_cmake_text = open("./linux/CMakeLists.txt", "r").read()
    if f'set(CRATE_NAME, "{package_name}")' not in linux_cmake_text:
        with open("./linux/CMakeLists.txt", "w") as cmake:
            split = linux_cmake_text.split(f"set({package_name}_bundled_libraries")
            split[1] = split[1].replace('""', r'"${CMAKE_CURRENT_SOURCE_DIR}/lib' + f'{package_name}.so"')
            linux_cmake_text = split[0] + f"\nset({package_name}_bundled_libraries" + split[1]
            cmake.write(linux_cmake_text)

    # Windows
    windows_cmake_text = open("./windows/CMakeLists.txt", "r").read()
    if f'set(CRATE_NAME, "{package_name}")' not in windows_cmake_text:
        with open("./windows/CMakeLists.txt", "w") as cmake:
            split = windows_cmake_text.split(f"set({package_name}_bundled_libraries")
            split[1] = split[1].replace('""', r'"${CMAKE_CURRENT_SOURCE_DIR}/pkgname.dll"')
            windows_cmake_text = split[0] + f"set({package_name}_bundled_libraries" + split[1].replace("pkgname", package_name)
            cmake.write(windows_cmake_text)

    # macOS
    mac_podspec = open(f"./macos/{package_name}.podspec", "r").read()
    if "s.vendored_libraries" not in mac_podspec:
        with open(f"./macos/{package_name}.podspec", "w") as podspec:
            # Remove the end keyword ----------v
            mac_podspec = mac_podspec.strip()[:-3] + "  s.vendored_libraries = 'Libs/**/*'\nend"
            podspec.write(mac_podspec)

    swift_text = open(f"./macos/Classes/{pascal_case_package_name}Plugin.swift", "r").read()
    if "dummy_method" not in swift_text:
            with open(f"./macos/Classes/{pascal_case_package_name}Plugin.swift", "w") as swift:
                entry_point = "public static func register(with registrar: FlutterPluginRegistrar) {"
                split = swift_text.split(entry_point)
                split[1] = "\n\t\tlet _ = dummy_method_to_enforce_bundling()" + split[1]
                swift_text = split[0] + entry_point + split[1]
                swift.write(swift_text)

    # iOS
    ios_podspec = open(f"./ios/{package_name}.podspec", "r").read()
    if "s.vendored_frameworks" not in ios_podspec:
        with open(f"./ios/{package_name}.podspec", "w") as podspec:
            ios_podspec = ios_podspec.strip()[:-3] + "  s.vendored_frameworks = 'Frameworks/**/*.xcframework'\n  s.static_framework = true\nend"
            podspec.write(ios_podspec)

    # If this is a Swift project
    if os.path.exists(f"./ios/Classes/Swift{pascal_case_package_name}Plugin.swift"):
        swift_text = open(f"./ios/Classes/Swift{pascal_case_package_name}Plugin.swift", "r").read()
        
        if "dummy_method" not in swift_text:
            with open(f"./ios/Classes/Swift{pascal_case_package_name}Plugin.swift", "w") as swift:
                entry_point = "public static func register(with registrar: FlutterPluginRegistrar) {"
                split = swift_text.split(entry_point)
                split[1] = "\n\t\tlet _ = dummy_method_to_enforce_bundling()" + split[1]
                swift_text = split[0] + entry_point + split[1]
                swift.write(swift_text)
    # Obj-C project
    else:
        objc_text = open(f"./ios/Classes/{pascal_case_package_name}Plugin.m", "r").read()

        if "dummy_method" not in objc_text:
            with open(f"./ios/Classes/{pascal_case_package_name}Plugin.m", "w") as objc:
                objc_text = '#import "../Classes/bridge_generated.h"' + objc_text
                entry_point = "+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {"
                split = objc_text.split(entry_point)
                split[1] = "\n\tdummy_method_to_enforce_bundling();" + split[1]
                objc_text = split[0] + entry_point + split[1]
                objc.write(objc_text)


def code_gen():
    print("Generating code with flutter_rust_bridge...\n")

    os.system("cargo install flutter_rust_bridge_codegen --version 1.71.0")
    os.system('flutter_rust_bridge_codegen \
        --dart-enums-style \
        --rust-input ./rust/src/lib.rs \
        --dart-output ./lib/src/bridge_generated.dart \
        --dart-decl-output ./lib/src/bridge_definitions.dart \
        --c-output ./rust/src/bridge_generated.h')

    shutil.copyfile("./rust/src/bridge_generated.h", "./ios/Classes/bridge_generated.h")
    shutil.move("./rust/src/bridge_generated.h", "./macos/Classes/bridge_generated.h")

    # Fix the incorrect import in the generated file.
    # This happens because we are using lib.rs as the entry point.
    generated_text = open("./rust/src/bridge_generated.rs", "r").read()
    open("./rust/src/bridge_generated.rs", "w").write(generated_text.replace("use crate::lib::*;", "use crate::*;"))

    if "ffi.dart" not in os.listdir("./lib/src"):
        package_name = open("./rust/Cargo.toml", "r").read().split("name = \"")[1].split("\"")[0]
        pascal_case_package_name = package_name.lower().replace("_", " ").title().replace(" ", "")

        file = open("./lib/src/ffi.dart", "w")
        file.write(
            requests.get(r"https://raw.githubusercontent.com/Desdaemon/flutter_rust_bridge_template/main/lib/ffi.dart")
                .text
                .replace("native", package_name)
                .replace("Native", pascal_case_package_name)
        )


def build(targets:list[str], openssl_path:str = None):
    print("Building Rust code...\n")

    package_name = open("./rust/Cargo.toml", "r").read().split("name = \"")[1].split("\"")[0]
    is_linux = sys.platform == "linux"
    is_windows = sys.platform == "win32"
    is_mac = sys.platform == "darwin"

    if (is_linux or is_windows or is_mac) and "android" in targets:
        print("Building Android libraries...\n")

        os.system("rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android")
        os.system("cargo install cargo-ndk")

        architectures = ["arm64-v8a", "armeabi-v7a", "x86", "x86_64"]
        for architecture in architectures:
            path = f"./android/src/main/jniLibs/{architecture}/lib{package_name}.so"
            if os.path.exists(path):
                os.remove(path)

        os.system("cd rust && cargo ndk -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 -o ../android/src/main/jniLibs build --release && cd ..")

    if is_linux and "linux" in targets:
        print("Building Linux libraries...\n")

        os.system("rustup target add x86_64-unknown-linux-gnu")
        os.system("cargo build --release --target x86_64-unknown-linux-gnu --manifest-path ./rust/Cargo.toml")

        if os.path.exists(f"./linux/lib{package_name}.so"):
            os.remove(f"./linux/lib{package_name}.so")

        shutil.move(f"./rust/target/x86_64-unknown-linux-gnu/release/lib{package_name}.so", f"./linux")

    if is_windows and "windows" in targets:
        print("Building Windows libraries...\n")

        os.system("rustup target add x86_64-pc-windows-msvc")
        os.system("cargo build --release --target x86_64-pc-windows-msvc --manifest-path ./rust/Cargo.toml")

        if os.path.exists(f"./windows/{package_name}.dll"):
            os.remove(f"./windows/{package_name}.dll")

        shutil.move(f"./rust/target/x86_64-pc-windows-msvc/release/{package_name}.dll", "./windows")

    if is_mac:
        if "macos" in targets:
            print("Building macOS libraries...\n")

            # Build for macOS.
            os.system("rustup target add aarch64-apple-darwin x86_64-apple-darwin")
            os.system("cargo build --release --target aarch64-apple-darwin --manifest-path ./rust/Cargo.toml")
            os.system("cargo build --release --target x86_64-apple-darwin --manifest-path ./rust/Cargo.toml")
            os.system(f'lipo "./rust/target/aarch64-apple-darwin/release/lib{package_name}.a" "./rust/target/x86_64-apple-darwin/release/lib{package_name}.a" -output "lib{package_name}.a" -create')

            if os.path.exists(f"./macos/Libs/lib{package_name}.a"):
                os.remove(f"./macos/Libs/lib{package_name}.a")

            shutil.move(f"./lib{package_name}.a", "./macos/Libs")

        if "ios" in targets:
            # Build for iOS
            print("Building iOS libraries...\n")

            os.system("rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios")
            os.system("cargo build --release --target aarch64-apple-ios --manifest-path ./rust/Cargo.toml")

            env_vars = f"OPENSSL_STATIC=1 OPENSSL_LIB_DIR=/usr/local/lib OPENSSL_INCLUDE_DIR={openssl_path} OPENSSL_NO_VENDOR=1 " if openssl_path is not None else ""
            os.system(f"{env_vars} CMAKE_OSX_SYSROOT=$(xcrun --sdk iphonesimulator --show-sdk-path) cargo build --release --target aarch64-apple-ios-sim --manifest-path ./rust/Cargo.toml")

            os.system("cargo build --release --target x86_64-apple-ios --manifest-path ./rust/Cargo.toml")
            os.system(f'lipo "./rust/target/aarch64-apple-ios-sim/release/lib{package_name}.a" "./rust/target/x86_64-apple-ios/release/lib{package_name}.a" -output "lib{package_name}.a" -create')
            os.system(f"xcodebuild -create-xcframework -library ./rust/target/aarch64-apple-ios/release/lib{package_name}.a -library ./lib{package_name}.a -output {package_name}.xcframework")
            os.remove(f"./lib{package_name}.a")

            if os.path.exists(f"./ios/Frameworks/{package_name}.xcframework"):
                shutil.rmtree(f"./ios/Frameworks/{package_name}.xcframework")

            shutil.move(f"./{package_name}.xcframework", "./ios/Frameworks")


def bump_version(version: str):
    def replace_string_in_file(file, regex, replacement):
        new_text = re.sub(regex, replacement, file.read(), count=1)
        file.seek(0)
        file.write(new_text)
        file.seek(0)


    # pubspec.yaml
    pubspec = open("./pubspec.yaml", "r+")
    replace_string_in_file(pubspec, r"version: [\d|\.]+\s", f"version: {version}\n")
    pubspec.close()

    # Cargo.toml
    cargo = open("./rust/Cargo.toml", "r+")
    replace_string_in_file(cargo, r'version = "[\d|\.]+"\s', f'version = "{version}"\n')
    cargo.close()

    # Android CMake
    android_cmake = open("./android/CMakeLists.txt", "r+")
    replace_string_in_file(android_cmake, r'set\(Version "[\d|\.]+"\)\s', f'set(Version "{version}")\n')
    android_cmake.close()

    # Linux CMake
    linux_cmake = open("./linux/CMakeLists.txt", "r+")
    replace_string_in_file(linux_cmake, r'set\(Version "[\d|\.]+"\)\s', f'set(Version "{version}")\n')
    linux_cmake.close()

    # Windows CMake
    windows_cmake = open("./windows/CMakeLists.txt", "r+")
    replace_string_in_file(windows_cmake, r'set\(Version "[\d|\.]+"\)\s', f'set(Version "{version}")\n')
    windows_cmake.close()

    pubspec_text = open("./pubspec.yaml", "r").read()
    package_name = pubspec_text.split("name: ")[1].split("\n")[0].strip()

    # macOS podspec
    macos_podspec = open(f"./macos/{package_name}.podspec", "r+")
    replace_string_in_file(macos_podspec, r'version = "[\d|\.]+"\s', f'version = "{version}"\n')
    replace_string_in_file(macos_podspec, r"s\.version\s+= '[\d|\.]+'\s", f"s.version          = '{version}'\n")
    macos_podspec.close()

    # iOS podspec
    ios_podspec = open(f"./ios/{package_name}.podspec", "r+")
    replace_string_in_file(ios_podspec, r'version = "[\d|\.]+"\s', f'version = "{version}"\n')
    replace_string_in_file(ios_podspec, r"s\.version\s+= '[\d|\.]+'\s", f"s.version          = '{version}'\n")
    ios_podspec.close()


if __name__ == "__main__":
    args = parser.parse_args()

    if args.init:
        init()

    if args.code_gen:
        code_gen()

    if args.build != None:
        targets = args.build
        if len(args.build) == 0:
            targets = ["android", "linux", "windows", "ios", "macos"]
        build(targets, args.ios_ssl)

    if args.bump_version:
        bump_version(args.bump_version)