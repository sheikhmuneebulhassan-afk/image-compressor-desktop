# Image Compressor Desktop v1.0

A simple offline Windows + macOS image utility built with Tauri 2 and Rust.

## Included features

- JPG / JPEG / PNG / WebP / AVIF input
- JPG / PNG / WebP / AVIF output
- Quality control for JPG, WebP and AVIF
- Target-KB compression (automatically lowers quality and, only when necessary, dimensions)
- Resize with optional aspect-ratio preservation
- Batch processing
- Drag and drop in the desktop app
- Custom output folder
- Duplicate-safe filenames
- Local history using app localStorage
- Light / dark / system theme
- No account, server, cloud upload or internet dependency during image processing

## Build requirements

- Node.js 20+ (22 recommended)
- Rust stable (Tauri requires Rust 1.77.2+)
- Tauri OS prerequisites for the platform

## Windows

Run `build-windows.bat`.

Expected output is under:

`src-tauri/target/release/bundle/nsis/`
`src-tauri/target/release/bundle/msi/`

## macOS

Run:

`./build-macos.sh`

Expected output is under:

`src-tauri/target/release/bundle/dmg/`
`src-tauri/target/release/bundle/macos/`

Unsigned macOS builds may show Gatekeeper warnings. Public distribution should use Apple code signing + notarization. Windows public distribution should also use code signing to reduce SmartScreen warnings.

## Automatic Windows + macOS builds

The repository includes `.github/workflows/build-desktop.yml`. Push it to GitHub and run **Build Desktop Installers** from Actions. It builds Windows and macOS artifacts on their native runners.

## Development

```bash
npm install
npm run dev
```

## Notes

The broad Tauri asset-protocol scope is used only to preview user-selected local images. The app itself does not upload them anywhere.
