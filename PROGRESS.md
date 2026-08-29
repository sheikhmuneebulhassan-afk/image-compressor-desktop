# Progress Log — Image Compressor Desktop

## Environment Setup

- Mac: Intel x86_64, macOS 14.0 (Sonoma)
- Node.js v22.23.1 / npm 10.9.8 — already installed
- Rust — was missing, installed via rustup (stable, rustc 1.98.0)
- Xcode Command Line Tools — already installed
- `gh` CLI — installed but needed re-authentication (`gh auth login`, then `gh auth refresh -s workflow` for Actions workflow pushes)

## Bugs Found and Fixed

1. **Missing dialog permission** — `src-tauri/capabilities/default.json` only granted `core:default` and `core:webview:default`. `tauri-plugin-dialog` needs `dialog:default` explicitly, or "Choose Images" / "Choose output folder" fail at runtime with a permission error. Fixed by adding `"dialog:default"`.

2. **Missing `protocol-asset` Cargo feature** — `tauri.conf.json` enables the asset-protocol scope (used for image thumbnail previews), but `src-tauri/Cargo.toml` didn't have `features = ["protocol-asset"]` on the `tauri` dependency. This was a hard compile error. Fixed.

3. **Batch processing aborted on first failure** — `process_images` used `.collect()` on a `Result`, so one unreadable/corrupt file cancelled the entire batch instead of just reporting that one file as failed. Reworked `ProcessResult` (added `error: Option<String>`, `output_path` became `Option<String>`) so each file succeeds or fails independently; updated `src/app.js` to show per-file errors and only summarize successful results.

4. **CI: universal macOS build silently uploaded nothing** — after switching the GitHub Actions macOS job to `--target universal-apple-darwin` (see below), the artifact-upload step's glob (`src-tauri/target/release/bundle/**/*`) didn't match the universal target's actual output path (`src-tauri/target/universal-apple-darwin/release/bundle/**/*`). Combined with `if-no-files-found: warn`, the job reported "success" while uploading zero macOS files. Fixed the glob to `src-tauri/target/**/release/bundle/**/*` and changed `if-no-files-found` to `error` so this class of bug fails loudly instead of silently.

5. **CI macOS build was Apple Silicon–only** — `macos-latest` GitHub runners now default to arm64 hardware, so the first CI build produced an arm64-only `.dmg` that would not run on Intel Macs. Fixed by adding `rustup target add aarch64-apple-darwin x86_64-apple-darwin` and building with `--target universal-apple-darwin`, producing a single fat binary for both architectures (verified with `lipo -info`).

Minor cleanup: removed two unused-import/variable compiler warnings in `lib.rs`.

## Testing

- Added a `#[cfg(test)]` module directly in `src-tauri/src/lib.rs` (12 tests) exercising the real production functions:
  - Resize: width-only (aspect preserved), width+height without aspect lock (stretches), no dimensions (no-op)
  - JPG output blends transparency to white, not black; PNG output preserves transparency
  - Target-KB compression stays near target and terminates on an impossible target (no infinite loop)
  - Duplicate-safe filenames (`photo-2`, `photo-3`) and overwrite mode
  - One failed image no longer cancels the batch (regression test for bug #3 above)
  - PNG→WebP conversion round-trips; image info reads correct dimensions/format
  - All 12 pass (`cargo test --manifest-path src-tauri/Cargo.toml`)
- `cargo check` / `cargo build --release` — clean, no warnings
- `npm run build:web` — passes
- `npm run dev` (`tauri dev`) — native window launches and runs
- Manual UI click-through (file picker, drag-drop, live processing) — done by the user, since no native-macOS GUI-automation tool was available in this environment (only Chrome-tab automation)
- Sample test images generated at `~/Desktop/ic-test-samples/` (photo JPG, transparent PNG, solid WebP, wide PNG) for manual testing

## Builds

**Local macOS build** (`npm run build` → `tauri build`):
- `src-tauri/target/release/bundle/macos/Image Compressor.app` — x86_64 only, unsigned, launched and verified standalone
- `src-tauri/target/release/bundle/dmg/Image Compressor_1.0.0_x64.dmg` — mounted, checksummed, contents verified

Copied into `../IMAGE APP BUILD/`:
- `Image Compressor.app`
- `Image Compressor_1.0.0_x64.dmg`
- `Windows/Image Compressor_1.0.0_x64-setup.exe`
- `Windows/Image Compressor_1.0.0_x64_en-US.msi`

**CI builds (GitHub Actions, `.github/workflows/build-desktop.yml`)** — Windows can't be cross-compiled from macOS, so this repo was pushed to GitHub and built on native `windows-latest` / `macos-latest` runners:
- Repo: https://github.com/sheikhmuneebulhassan-afk/image-compressor-desktop (public)
- Final successful run: https://github.com/sheikhmuneebulhassan-afk/image-compressor-desktop/actions/runs/33274711244
- Windows: NSIS `.exe` and `.msi`, both x64
- macOS: universal `.dmg` (arm64 + x86_64 fat binary)

## Release

**v1.0.0** published at https://github.com/sheikhmuneebulhassan-afk/image-compressor-desktop/releases/tag/v1.0.0 with three public download assets (no login required):
- `Image.Compressor_1.0.0_universal.dmg`
- `Image.Compressor_1.0.0_x64-setup.exe`
- `Image.Compressor_1.0.0_x64_en-US.msi`

None of the builds are code-signed — macOS Gatekeeper / Windows SmartScreen may warn on first launch (expected for local/CI testing; public distribution would need a Developer ID / code-signing certificate + notarization).

## Rebuild Commands

```bash
# Local macOS build
npm run build

# Trigger a fresh CI build for both platforms (after pushing changes)
gh workflow run build-desktop.yml --ref main

# Cut a new tagged release (also auto-triggers CI via the v* tag push)
git tag vX.Y.Z && git push origin vX.Y.Z
```
