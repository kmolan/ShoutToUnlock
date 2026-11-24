#!/bin/sh
# builds a swift package with an XCFramework for iOS, the iOS simulator, and macOS (apple silicon) into target/apple
# needs macOS with Xcode and `rustup target add aarch64-apple-ios aarch64-apple-ios-sim aarch64-apple-darwin`.
set -eu
cd "$(dirname "$0")/.."
output=target/apple/ShoutToUnlock
bindings=target/apple/bindings
targets="aarch64-apple-ios aarch64-apple-ios-sim aarch64-apple-darwin"

for target in $targets; do
    cargo build --release --features mobile --target "$target"
done
cargo run --release --features uniffi-bindgen --bin uniffi-bindgen -- generate \
    --library target/aarch64-apple-darwin/release/libshout_to_unlock.dylib \
    --language swift --no-format --out-dir "$bindings"

rm -rf "$output"
mkdir -p "$bindings/headers" "$output/Sources/ShoutToUnlock"
cp "$bindings/shout_to_unlockFFI.h" "$bindings/headers/"
cp "$bindings/shout_to_unlockFFI.modulemap" "$bindings/headers/module.modulemap"
cp "$bindings/shout_to_unlock.swift" "$output/Sources/ShoutToUnlock/"

set --
for target in $targets; do
    set -- "$@" -library "target/$target/release/libshout_to_unlock.a" -headers "$bindings/headers"
done
xcodebuild -create-xcframework "$@" -output "$output/shout_to_unlockFFI.xcframework"

cat > "$output/Package.swift" <<'PACKAGE'
// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "ShoutToUnlock",
    platforms: [.iOS(.v15), .macOS(.v12)],
    products: [.library(name: "ShoutToUnlock", targets: ["ShoutToUnlock"])],
    targets: [
        .binaryTarget(name: "shout_to_unlockFFI", path: "shout_to_unlockFFI.xcframework"),
        .target(name: "ShoutToUnlock", dependencies: ["shout_to_unlockFFI"]),
    ]
)
PACKAGE

echo "done bestie. in Xcode, add $output as a local package. then 'import ShoutToUnlock' and start screaming."
