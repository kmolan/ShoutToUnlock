# ShoutToUnlock

A joke library. The lock opens when the total time at or above a minimum level reaches a minimum
duration. One Rust core has bindings for C, C++, Python, JS (wasm), Kotlin, and Swift.

The brand name is `ShoutToUnlock`, with the tagline "lemme in. LEMME IIIN." The package name is
`shout_to_unlock` on crates.io, PyPI, and npm. Do not use the brand name as a package name.

The author is Anmol Kathail. The name is in `LICENSE`, `Cargo.toml`, and `pyproject.toml`.

## Rules for this project

- **Project text is lulz style.** This applies to the README, the doc comments, the code comments,
  the package descriptions, the error messages, the script output, and the web page text. Use
  lowercase, "u/ur/bestie", and jokes. Formulas and invariants must stay correct.
- **Never use "we", "us", or "our"** in project text. The narrator is "I".
- **Emojis are minimal.** Only 📢🔓 in the README title and the 🔒/🔓 padlock on the web page are
  permitted. Do not add more.
- **Chat replies and this file use STE**, as the global `CLAUDE.md` tells.
- **Git:** do not init, branch, or tag. Commit and push without further permission.
- **Commit messages:** one short line, with a maximum of seven words. Do not add a body.
- **Attribution:** never add attribution to Claude or to an AI in any text. This prohibition
  includes a `Co-Authored-By` trailer and a "Generated with Claude Code" footer.
- **Keep it simple.** Do not add features, knobs, or layers that the user did not ask for.

## Layout

| Path | Contents |
|---|---|
| `src/lib.rs` | The core `ShoutLock`, `level_dbfs`, the unit tests, and one doctest. |
| `src/ffi.rs` | The C ABI (`shout_to_unlock_*`). Not compiled for `target_family = "wasm"`. |
| `src/python.rs` | PyO3 class (feature `python`). |
| `src/wasm.rs` | wasm-bindgen class with camelCase names (feature `wasm`). |
| `src/mobile.rs` | UniFFI object for Kotlin and Swift (feature `mobile`). |
| `include/` | `shout_to_unlock.h` (C) and `shout_to_unlock.hpp` (C++17 RAII wrapper). |
| `examples/` | `cpp_example.cpp` and `microphone.py` (sounddevice). |
| `tests/test_python.py` | Python tests against the installed module. |
| `web/` | The browser demo. `web/pkg` is the wasm-pack output and is gitignored. |
| `mobile/` | `android.sh`, `apple.sh`, and the `uniffi-bindgen` binary source. |
| `.github/workflows/ci.yml` | Tests on each push and PR. A push to `main` deploys `web/` to GitHub Pages. A `v*` tag builds and publishes everything. |

## Core invariants

- Level of one buffer: RMS in dBFS, `20·log₁₀(√(Σx²/N))`. Silence gives `-∞`.
- A buffer with a level `>= minimum_level_dbfs` adds its duration to `loud_duration`.
- `loud_duration` is a total. The loud time does not need to be continuous.
- `is_unlocked = !loud_duration.is_zero() && loud_duration >= minimum_duration`.
- After the lock opens, it stays open until `reset`. `reset` keeps the limits.
- Durations are `std::time::Duration`, so the sum is exact. A negative, non-finite, or too large
  duration is an error in the constructor. The C ABI returns `NULL` for it, and
  `process_level` ignores it.

## Binding notes

- **UniFFI:** the struct in `src/mobile.rs` must have the name `ShoutLock`. A `#[uniffi(name)]`
  rename breaks the constructor ("Constructor return type must be Self or Arc<Self>"). Refer to the
  core as `crate::ShoutLock` in that file.
- **UniFFI bindgen:** use `--no-format`, or ktlint gives a warning.
- **Browser demo:** analyser windows overlap. Each frame adds the elapsed time, with a maximum of
  one analyser window. The demo turns off automatic gain control, noise suppression, and echo
  cancellation.
- **Cargo `include`:** the patterns start with `/`. Without it, `README.md` also matches
  `web/pkg/README.md`.
- **Version:** `Cargo.toml` is the only source. `pyproject.toml` uses `dynamic = ["version"]`.

## Commands

```sh
cargo fmt --check
cargo clippy --all-targets --features python,uniffi-bindgen -- -D warnings
cargo clippy --target wasm32-unknown-unknown --features wasm -- -D warnings
cargo test
pip install . && python3 -m unittest -v tests/test_python.py
wasm-pack build --target web --out-dir web/pkg --features wasm
python3 -m http.server --directory web 8000
./mobile/android.sh   # needs cargo-ndk and ANDROID_NDK_HOME
./mobile/apple.sh     # needs macOS and Xcode
```

To build the C++ example:

```sh
cargo build --release
g++ -std=c++17 -Wall -Wextra -Werror -Iinclude examples/cpp_example.cpp -Ltarget/release -lshout_to_unlock -o examples/cpp_example
LD_LIBRARY_PATH=target/release ./examples/cpp_example
```

## Current state (2026-10-30, version 0.1.0, not released)

Verified:

- `cargo test` passes. Clippy gives 0 warnings for all features and for the wasm target.
- The C++ example runs, and a static C link works.
- The Python tests pass. NumPy arrays work as input.
- The JS API works on Node. The user confirmed that the browser demo works.
- The Kotlin bindings run on the JVM. The Swift bindings run on Swift for Linux.
- `android.sh` builds three ABIs.
- `cargo publish --dry-run` packages 11 files. The sdist installs as version 0.1.0.

Not verified:

- `mobile/apple.sh`. It needs a Mac.
- The Android and iOS microphone code in the README.
- `examples/microphone.py` with a real microphone.
- `ci.yml` on GitHub. Actionlint did not run. The macOS and Windows jobs have the most risk.

Open items:

- The repository is `https://github.com/kmolan/ShoutToUnlock`. It is public.
- The live demo is `https://kmolan.github.io/ShoutToUnlock/`. The README links to it.
- The release needs the repository secrets `CARGO_REGISTRY_TOKEN`, `PYPI_API_TOKEN`, and
  `NPM_TOKEN`.
- `shout_to_unlock` was a free name on crates.io, PyPI, and npm on 2026-10-30.

## Intended future state

The goal is maximum visibility for a joke. The live browser demo is the main hook. Most people
will not install a package, but they will click a link.

### 1. First release (user steps, then agent steps)

1. The user gives a GitHub URL.
2. Add `repository` to `Cargo.toml` and `pyproject.toml`.
3. The user creates the repository, adds the three secrets, and pushes.
4. Fix the CI jobs that fail on the first run.
5. The user pushes the `v0.1.0` tag.

### 2. Demo changes

Done: the Pages deploy job, the line "audio stays on ur device", and the README link.

Proposed, not approved:

- After the lock opens, show a share line with the peak level and a copy button.

### 3. Launch (user steps)

- Record a 10 to 15 second video of a person who yells until the padlock opens. Put a GIF of it at
  the top of the README.
- Add the GitHub description, demo URL, topics, and social preview image.
- Day 1: post a Show HN and an r/rust post. Day 2: post the video on social sites.
- Suggest the crate for "Crate of the Week" in This Week in Rust.

### Out of scope unless the user asks

- A leaderboard or any server. The page sends no data.
- Frequency analysis, voice detection, or other ways to measure a shout.
- More language bindings.
