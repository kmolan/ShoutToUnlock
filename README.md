# ShoutToUnlock 📢🔓

**lemme in. LEMME IIIN.**

no password. no face id. no fingerprint. just ur LUNGS

scream at ur computer until it says `true`. breaks for air are allowed, I am not a monster. the total counts, not the streak. ur neighbours will hate it. ur cat will leave. the lock will open.

the brand is ShoutToUnlock. the package is `shout_to_unlock` everywhere, bc package names are allergic to capital letters.

| Language | Install |
|---|---|
| Rust | `shout_to_unlock = { path = "…" }` in `Cargo.toml` |
| Python | `pip install .` |
| JavaScript / browser | `wasm-pack build --target web --out-dir web/pkg --features wasm` |
| C / C++ | `cargo build --release`, then use `include/` and `target/release` |
| Kotlin (Android) | `./mobile/android.sh` |
| Swift (iOS, macOS) | `./mobile/apple.sh` |

every language gets its own little section below. scroll, bestie

## how it works (science)

the lock opens when u are loud enough for long enough. u pick two numbers:

- `minimum_level_dbfs`: how loud. the minimum RMS level of a buffer, in dBFS.
- `minimum_duration`: how long. the minimum total time at or above that level.

the loud time does NOT need to be one long scream. it all goes in the loud-time jar. the lock starts closed (`false`). when the jar hits `minimum_duration`, the lock opens (`true`) and stays open until `reset`no take-backs.

each buffer gets a vibe check: `20·log₁₀(√(Σx²/N))`. samples are mono `f32` in `[-1, 1]`. a full-scale sine wave is ~−3 dBFS. silence is `-∞` (dramatic). the whole buffer gets ONE level, so buffer length = time resolution. big buffer, blurry vibes.

one rust core rules them all. everyone else just borrows it:

- C and C++: the C ABI, for the classics
- Python: PyO3 (feature `python`)
- JavaScript: WebAssembly + wasm-bindgen (feature `wasm`)
- Kotlin and Swift: UniFFI (feature `mobile`)

## the API (what u can poke)

JavaScript, Kotlin, and Swift are camelCase girlies: `processLevel`, `isUnlocked`, `loudSeconds`, etc. in Kotlin and Swift the queries are methods, so `lock.isUnlocked()` with the parens.

| Rust | C / C++ / Python | the vibe |
|---|---|---|
| `process(samples, sample_rate)` | `process` | feed it a buffer. returns `is_unlocked`. |
| `process_level(level_dbfs, duration)` | `process_level` | measured it urself? overachiever. hand it over. |
| `is_unlocked()` | `is_unlocked` | `true` once u've screamed enough. THE question. |
| `loud_duration()` | `loud_seconds` | ur scream résumé. total time loud enough. |
| `remaining_duration()` | `remaining_seconds` | how much more screaming until freedom. |
| `progress()` | `progress` | `loud / minimum`, clamped to `[0, 1]`. progress bar for ur lungs. |
| `total_duration()` | `total_seconds` | total time of everything u fed it, loud or sad. |
| `last_level_dbfs()` | `last_level_dbfs` | how loud u are rn. |
| `peak_level_dbfs()` | `peak_level_dbfs` | ur personal best since the last reset. |
| `minimum_level_dbfs()`, `minimum_duration()` | `minimum_level_dbfs`, `minimum_seconds` | ur standards. |
| `reset()` | `reset` | slams it shut, forgets everything. same standards. |
| `level_dbfs(samples)` | `level_dbfs` | vibe-check any buffer. |

## Rust

```rust
use std::time::Duration;
use shout_to_unlock::ShoutLock;

let mut lock = ShoutLock::new(-20.0, Duration::from_secs(2));
if lock.process(&samples, 48_000) {
    println!("ur in");
}
```

## C and C++ (for the classics)

`cargo build --release` drops `libshout_to_unlock.so` (shared) and `libshout_to_unlock.a` (static) into `target/release`. macOS and Windows call them something else bc they're special.

- C: include `include/shout_to_unlock.h`. free ur own memory like an adult.
- C++17: include `include/shout_to_unlock.hpp`. `shout_to_unlock::ShoutLock` frees itself in its destructor. self care

```sh
cargo build --release
g++ -std=c++17 -Iinclude examples/cpp_example.cpp -Ltarget/release -lshout_to_unlock -o examples/cpp_example
LD_LIBRARY_PATH=target/release ./examples/cpp_example
```

linking the static lib? add whatever system libs `cargo rustc --release --lib -- --print native-static-libs` tells u. don't argue with it.

## Python

u need a rust toolchain. then this builds + installs it, ez:

```sh
pip install .
```

```python
from shout_to_unlock import ShoutLock

lock = ShoutLock(minimum_level_dbfs=-15.0, minimum_duration_seconds=2.0)
lock.process(block, 48_000)  # list of floats or 1-D NumPy array, I'm not picky
print(lock.is_unlocked, lock.progress, lock.last_level_dbfs)
```

a negative or NaN duration raises `ValueError`. time only goes forward bestie.

to scream into ur actual microphone (warn ur roommates first):

```sh
pip install . sounddevice numpy
python3 examples/microphone.py
```

## JavaScript + the browser

u need [`wasm-pack`](https://github.com/rustwasm/wasm-pack). this spits an ES module into `web/pkg`:

```sh
wasm-pack build --target web --out-dir web/pkg --features wasm
```

```js
import init, { ShoutLock } from "./pkg/shout_to_unlock.js";

await init();
const lock = new ShoutLock(-15, 2);
lock.process(float32Samples, 48000);
console.log(lock.isUnlocked, lock.progress, lock.lastLevelDbfs);
lock.free();
```

a bad duration throws an `Error`. on Node or a bundler? use `--target nodejs` or `--target bundler`.

the demo page in `web/` listens to ur mic and judges u:

```sh
python3 -m http.server --directory web 8000
```

open <http://localhost:8000>. browsers only give mic access on `localhost` or HTTPS, so no sketchy http links. the page turns off automatic gain control, bc AGC makes a scream and a whisper sound the same and I want CHAOS.

## Android (Kotlin)

u need:

- the Android NDK, with `ANDROID_NDK_HOME` pointing at it
- `cargo install cargo-ndk`
- `rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android`

```sh
./mobile/android.sh
```

it drops this stuff in `target/android`:

- `jniLibs/<abi>/libshout_to_unlock.so` for `arm64-v8a`, `armeabi-v7a`, and `x86_64`. yeet the folder into `app/src/main/jniLibs`.
- `kotlin/uniffi/shout_to_unlock/shout_to_unlock.kt`. yeet it into `app/src/main/kotlin`.

the Kotlin bindings need JNA. add it to `app/build.gradle.kts`:

```kotlin
implementation("net.java.dev.jna:jna:5.19.1@aar")
```

using R8? don't let it eat the bindings: `-keep class com.sun.jna.** { *; }` and `-keep class uniffi.** { *; }`.

add `<uses-permission android:name="android.permission.RECORD_AUDIO" />` and ask nicely at run time. read the mic on a background thread, not the UI thread, I'm begging u:

```kotlin
import uniffi.shout_to_unlock.ShoutLock

val sampleRate = 48_000
// UNPROCESSED = no automatic gain control = real screams. if
// AudioManager.PROPERTY_SUPPORT_AUDIO_SOURCE_UNPROCESSED is not "true", settle for VOICE_RECOGNITION.
val recorder = AudioRecord(
    MediaRecorder.AudioSource.UNPROCESSED, sampleRate,
    AudioFormat.CHANNEL_IN_MONO, AudioFormat.ENCODING_PCM_FLOAT,
    AudioRecord.getMinBufferSize(sampleRate, AudioFormat.CHANNEL_IN_MONO, AudioFormat.ENCODING_PCM_FLOAT),
)
val buffer = FloatArray(1024)
recorder.startRecording()
ShoutLock(-15f, 2.0).use { lock ->
    while (!lock.isUnlocked()) {
        val count = recorder.read(buffer, 0, buffer.size, AudioRecord.READ_BLOCKING)
        if (count > 0) lock.process(buffer.asList().subList(0, count), sampleRate.toUInt())
    }
}
recorder.release()
```

a bad duration throws `ShoutLockException.InvalidDuration`. time only goes forward bestie.

## iOS + macOS (Swift)

u need a Mac with Xcode (sorry linux gang) and `rustup target add aarch64-apple-ios aarch64-apple-ios-sim aarch64-apple-darwin`.

```sh
./mobile/apple.sh
```

u get a Swift package at `target/apple/ShoutToUnlock`, with an XCFramework for iOS, the iOS simulator, and macOS on Apple silicon inside. in Xcode, add it as a local package and ur done.

add `NSMicrophoneUsageDescription` to `Info.plist` ("i need to hear u scream" is a valid reason). then:

```swift
import AVFoundation
import ShoutToUnlock

// .measurement mode turns off automatic gain control. real screams only.
let session = AVAudioSession.sharedInstance()
try session.setCategory(.record, mode: .measurement)
try session.setActive(true)

let lock = try ShoutLock(minimumLevelDbfs: -15, minimumDurationSeconds: 2)
let engine = AVAudioEngine()
let format = engine.inputNode.outputFormat(forBus: 0)
engine.inputNode.installTap(onBus: 0, bufferSize: 1024, format: format) { buffer, _ in
    let samples = Array(UnsafeBufferPointer(start: buffer.floatChannelData![0], count: Int(buffer.frameLength)))
    // the tap runs on an audio thread. stop the engine + update the UI on the main thread.
    if lock.process(samples: samples, sampleRate: UInt32(format.sampleRate)) {
        DispatchQueue.main.async { engine.stop() }
    }
}
try engine.start()
```

`ShoutLock` is thread-safe, scream from any thread. a bad duration throws `ShoutLockError.InvalidDuration`.

## tests (they pass, I checked)

```sh
cargo test
pip install . && python3 -m unittest -v tests/test_python.py
```

## shipping it

`.github/workflows/ci.yml` runs the tests on every push and PR. push a `v*` tag and it builds everything and ships it everywhere. all or nothing: if one build fails, nothing gets published.

1. bump `version` in `Cargo.toml`. it's the only place, `pyproject.toml` reads it from there.
2. `git tag v0.1.0 && git push --tags`. the tag must match the version or CI will call u out.

the repo needs three secrets first: `CARGO_REGISTRY_TOKEN` (crates.io), `PYPI_API_TOKEN` (PyPI), and `NPM_TOKEN` (npm).

| where | what lands there |
|---|---|
| crates.io | the crate |
| PyPI | wheels for linux x86_64/aarch64, macOS x86_64/arm64, windows x64, plus the sdist |
| npm | the wasm package (bundler target) |
| GitHub release | C/C++ libs + headers for linux, macOS, and windows. the android `jniLibs` + kotlin. the swift package with the XCFramework. |
