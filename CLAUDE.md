# ShoutToUnlock

A joke library. The lock opens when the total time at or above a minimum level reaches a minimum
duration. One Rust core has bindings for C, C++, Python, JS (wasm), Kotlin, and Swift.

The brand name is `ShoutToUnlock`, with the tagline "lemme in. LEMME IIIN." The package name is
`shout_to_unlock` on crates.io, PyPI, and npm. Do not use the brand name as a package name.

The author is Anmol Kathail. The name is in `LICENSE`, `Cargo.toml`, and `pyproject.toml`.

- Repository: `https://github.com/kmolan/ShoutToUnlock` (public, branch `main`).
- Live demo: `https://kmolan.github.io/ShoutToUnlock/`. The second line of the README links to it.

## Rules

- Do not change user-facing docs without the permission of the user. These include `README.md`,
  the web page text, the package descriptions, and all other docs.

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
| `.github/workflows/ci.yml` | The CI. Refer to the CI section. |

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

## CI

| Trigger | Jobs |
|---|---|
| Each push and PR | `test` |
| Push to `main` | `test`, then `pages`. It deploys `web/` to GitHub Pages. |
| Manual run | `test`, then the six build jobs. It publishes nothing. |
| `v*` tag | `test`, the six build jobs, then the GitHub release, crates.io, PyPI, and npm. |

- The tag must match the version in `Cargo.toml`, or `test` fails.
- The four publish jobs need all six build jobs. They run in parallel, so one can fail after
  another published.
- The repository has the secrets `CARGO_REGISTRY_TOKEN`, `PYPI_API_TOKEN`, and `NPM_TOKEN`.
- To set a secret, use `gh secret set <name>` in a normal terminal. A `!` command gives no
  prompt, so it sets an empty value.
- `upload-artifact` and `download-artifact` are at `v4`, which uses Node.js 20. The newer major
  versions change behavior, so test them with a manual run before a tag.

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
gh workflow run ci.yml --ref main   # build-only release run
```

To build the C++ example:

```sh
cargo build --release
g++ -std=c++17 -Wall -Wextra -Werror -Iinclude examples/cpp_example.cpp -Ltarget/release -lshout_to_unlock -o examples/cpp_example
LD_LIBRARY_PATH=target/release ./examples/cpp_example
```

## Current state (2026-11-23, version 0.1.0, released)

Verified:

- `cargo test` passes. Clippy gives 0 warnings for all features and for the wasm target.
- The C++ example runs, and a static C link works.
- The Python tests pass. NumPy arrays work as input.
- The JS API works on Node. The user confirmed that the browser demo works.
- The Kotlin bindings run on the JVM. The Swift bindings run on Swift for Linux.
- `cargo publish --dry-run` packages 11 files. The sdist installs as version 0.1.0.
- On GitHub, `test`, `pages`, and all six build jobs pass. This includes macOS, Windows,
  `android.sh`, and `apple.sh`.
- Version 0.1.0 is on crates.io, PyPI, and npm. The GitHub release `v0.1.0` has the C,
  Android, and Apple archives.
- `pip install shout_to_unlock` installs the manylinux wheel. No Rust toolchain is necessary.
- The npm package imports in Node 24 with `--experimental-wasm-modules`. It needs no `init()`.
- The first npm publish failed: the token had no 2FA bypass. A rerun of only the `npm` job
  published it. npm shows a placeholder version `0.0.0-stage` while it processes a publish.

Not verified:

- `npm install shout_to_unlock` with a bundler.
- The Android and iOS microphone code in the README.
- `examples/microphone.py` with a real microphone.
