#!/bin/sh
# builds the android libraries + kotlin bindings into target/android, so ur phone can get yelled at
# needs ANDROID_NDK_HOME, `cargo install cargo-ndk`, and
# `rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android`.
set -eu
cd "$(dirname "$0")/.."
output=target/android

cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -o "$output/jniLibs" build --release --features mobile
cargo run --release --features uniffi-bindgen --bin uniffi-bindgen -- generate \
    --library target/aarch64-linux-android/release/libshout_to_unlock.so \
    --language kotlin --no-format --out-dir "$output/kotlin"

echo "done bestie. copy $output/jniLibs to app/src/main/jniLibs and $output/kotlin to app/src/main/kotlin."
