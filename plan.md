# Image Compressor Desktop — Development Plan

## Project
- Platforms: macOS + Windows
- Stack: Tauri 2 + Rust + HTML/CSS/JavaScript
- Processing: 100% local/offline
- Current stable release: v1.0.0

## Product Direction
Grow the app from a basic compressor/converter into an **All-in-One Offline Image Utility**.

Core principles:
- Simple for normal users
- Powerful batch workflow for advanced users
- No login or cloud dependency
- User images never leave the computer
- Fast local processing
- Lightweight desktop package
- Clear before/after results
- Stable Windows + macOS support

---

# v1.0 — Stable Baseline

Existing functionality:
- Drag and drop
- File picker
- Multiple image selection
- JPG/JPEG, PNG, WebP
- Compression
- Conversion
- Resize
- Aspect ratio lock
- Target file size
- Batch processing
- Per-file failures
- Duplicate-safe output filenames
- Custom output folder
- Open output folder
- Before/after file size
- Saving percentage
- Local history
- Light/Dark/System theme
- macOS APP/DMG
- Windows EXE/MSI
- GitHub Actions
- Universal macOS build
- Rust automated tests

Rule: do not rewrite stable v1.0 functionality unless fixing a confirmed problem.

---

# v1.1 — Core Utility Upgrade

## Goal
Improve the compressor/converter while keeping the main workflow simple.

## 1. HEIC / HEIF Support
Add:
- HEIC → JPG
- HEIC → PNG
- HEIC → WebP
- HEIF conversion

Requirements:
- Correct iPhone orientation
- Correct colors
- Batch support
- Graceful unsupported-file errors

## 2. AVIF Support
Add:
- AVIF input
- AVIF output
- AVIF quality control
- JPG/PNG/WebP ↔ AVIF conversions

Requirements:
- Transparency where supported
- Reasonable encoding time
- Correct colors

## 3. Folder Import
Add:
- `Add Folder`
- Scan supported images
- Ignore unsupported files safely

## 4. Include Subfolders
Option:
- `Include Subfolders`

Requirements:
- Scan nested folders
- Optional folder-structure preservation
- Never recursively process the output directory

## 5. Smart Optimize
Add output mode:
- Smart
- JPG
- PNG
- WebP
- AVIF

Suggested rules:
- Photo → WebP/AVIF
- Transparent graphic → WebP/PNG
- Already-small optimized file → avoid unnecessary recompression
- Never silently output a larger file without warning

Analyze:
- Source format
- Transparency
- Dimensions
- File size
- Expected saving
- Visual quality

## 6. Compression Presets
Add:
- Recommended
- High Quality
- Small File
- Web Optimized
- Lossless
- Custom

## 7. Target Size Presets
Add:
- 50 KB
- 100 KB
- 200 KB
- 500 KB
- 1 MB
- Custom

Requirements:
- Bounded attempts
- No infinite loop
- Resize only when needed
- Warn if requested target is unrealistic

## 8. Before / After Comparison
Show:
- Original preview
- Processed preview
- Slider comparison
- Zoom
- Pan
- Original/final size
- Dimensions
- Format
- Reduction percentage

## 9. Better Batch Progress
Show:
- Total
- Current file
- Completed
- Failed
- Remaining
- Overall percentage

One failed image must not stop the batch.

## 10. Saved Presets
Allow users to save settings such as:
- Format
- Quality
- Resize
- Target size
- Metadata behavior
- Naming pattern

Store presets locally.

## 11. Metadata Cleaner
Options:
- Keep Metadata
- Remove All Metadata
- Remove GPS Only

Handle where available:
- EXIF
- GPS
- Camera/device information
- Date/time
- IPTC
- XMP

## 12. Better Results Summary
After processing show:
- Images processed
- Images failed
- Total original size
- Total final size
- Space saved
- Reduction %

Actions:
- Open Output Folder
- Process More Images
- Clear Completed

---

# v1.1 UI

Main areas:
- Compress
- Convert
- Resize
- Batch
- Presets
- History
- Settings

Keep the workflow:

`Select Images → Settings → Process → Results → Open Folder`

Do not create unnecessary screens.

---

# v1.2 — Professional Batch Tools

## 1. Bulk Rename
Tokens:
- `{original}`
- `{number}`
- `{width}`
- `{height}`
- `{format}`
- `{date}`

Features:
- Prefix
- Suffix
- Start number
- Number padding
- Filename preview
- Duplicate-safe output

## 2. Watermark
Support:
- Text watermark
- Logo/image watermark

Controls:
- Position
- Opacity
- Size
- Margin
- Rotation

Must work in batch mode.

## 3. Auto Rotate
Read orientation and rotate correctly before processing.

## 4. Lossless Optimization
Reduce file size without visible quality reduction where supported.

## 5. DPI Tool
Presets:
- 72
- 96
- 150
- 300
- Custom

Changing DPI must not resize pixels unless explicitly requested.

## 6. Image Information
Show:
- Filename
- Format
- File size
- Width/height
- Aspect ratio
- Color space
- Transparency
- DPI
- EXIF presence
- GPS presence

## 7. Background Fill
For transparent → JPG:
- White
- Black
- Custom color

Default: white.

## 8. Processing Pipeline
Allow ordered operations:

1. Auto Rotate
2. Resize
3. Convert
4. Compress
5. Remove Metadata
6. Rename
7. Watermark

Allow saving pipeline as a preset.

---

# v1.3 — Ready-Made Workflows

## Website Optimizer
Suggested:
- Smart format
- Max width around 1920
- Smart quality
- Metadata cleanup
- sRGB
- No unnecessary enlargement

## Shopify Optimizer
Possible workflow:
- Product image optimization
- Compression
- Consistent output
- Metadata cleanup
- Naming support

## WordPress Optimizer
Presets for:
- Blog images
- Hero images
- General content images
- Responsive widths later

## Social Media Resize
Presets:
- Instagram Square 1080×1080
- Instagram Portrait 1080×1350
- Instagram Story 1080×1920
- Facebook Post
- Facebook Cover
- LinkedIn Post
- YouTube Thumbnail

Modes:
- Fit
- Fill
- Crop

Never distort the source.

## Multi-Size Export
One image can create multiple output sizes in one run.

---

# v1.4 — Automation

## Folder Watch
User selects:
- Input Folder
- Output Folder
- Preset

When new image arrives:
- Detect
- Process
- Save automatically

Requirements:
- No repeated processing
- No output-folder loop
- Activity log
- Pause/resume
- Completely optional

## Auto Preset by Folder
Example:
- Product Photos → Shopify Preset
- Website → Website Optimizer
- Social → Social Preset

## CLI Mode
Optional terminal interface for automation.

Do not make CLI functionality complicate the normal desktop UI.

---

# v2.0 — Advanced Image Utility Suite

Consider only after v1.x is stable:
- Duplicate image finder
- Similar image finder
- Crop
- Rotate
- Flip
- RAW conversion
- BMP
- TIFF
- ICO
- JPEG XL if practical
- Images → PDF
- PDF pages → images
- Color profile conversion
- Advanced EXIF editor
- Advanced watermark templates

Do not add features only because they look advanced. Each feature must solve a clear user problem.

---

# Performance Requirements

Test with:
- 1 image
- 20 images
- 100 images
- 500 images
- 20–50 MB source photos

Rules:
- Do not load full-resolution previews unnecessarily
- Use thumbnails in lists
- Do not block the UI
- Limit concurrent processing
- Release image buffers
- Avoid unnecessary file copies
- Watch memory use

---

# Error Handling

Handle:
- Corrupted image
- Unsupported format
- Read permission denied
- Write permission denied
- Disk full
- Invalid output path
- Impossible target size
- Missing codec
- Very large image
- Existing output file

One file failing must never cancel the full batch.

---

# Privacy

Core promise:

**User images never leave the computer.**

Do not require:
- Login
- Cloud upload
- External image-processing API
- Internet connection

Do not send:
- Images
- Filenames
- File paths
- EXIF/GPS data

If analytics are ever added, they must be optional and privacy-safe.

---

# Settings

## General
- Theme
- Default output folder
- Open folder after processing
- Confirm before overwrite

## Compression
- Default quality
- Default format
- Default preset

## Naming
- Keep original name
- Suffix
- Rename pattern
- Overwrite behavior

## Metadata
- Preserve
- Remove all
- Remove GPS

## Performance
- Concurrent jobs
- Hardware acceleration if supported

---

# Testing Requirements

Every release should run:

```bash
cargo test --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
npm run build:web
npm run build
```

Add regression tests for every confirmed bug.

---

# Conversion Test Matrix

Where supported, test:

- JPG → PNG
- JPG → WebP
- JPG → AVIF
- PNG → JPG
- PNG → WebP
- PNG → AVIF
- WebP → JPG
- WebP → PNG
- WebP → AVIF
- AVIF → JPG
- AVIF → PNG
- HEIC → JPG
- HEIC → PNG
- HEIC → WebP

---

# Resize Tests

Test:
- No resize
- Width only
- Height only
- Exact width + height
- Aspect ratio lock
- Very wide image
- Very tall image
- Square image
- Upscale disabled
- Upscale enabled if supported

---

# Target Size Tests

Test:
- 50 KB
- 100 KB
- 200 KB
- 500 KB
- 1 MB

Use:
- JPG photo
- PNG screenshot
- Transparent PNG
- WebP
- Large photo

Ensure:
- No infinite loops
- Bounded attempts
- Graceful failure when impossible

---

# Batch Tests

Test:
- 10 images
- 50 images
- 100 images
- Mixed formats
- Corrupted file inside batch
- Duplicate names
- Unsupported files in folder
- Nested folders
- Existing output files

---

# macOS Requirements

Build:
- Universal macOS package when practical
- Intel x86_64
- Apple Silicon arm64

For public release later:
- Developer ID signing
- Apple notarization
- Stapling

Signing must not block local development builds.

---

# Windows Requirements

Build:
- x64 NSIS EXE
- x64 MSI

For public release later:
- Code signing
- SmartScreen reputation improvement

---

# GitHub Actions

CI should:
1. Run tests
2. Run frontend build
3. Build Windows installers
4. Build universal macOS package
5. Fail if expected artifacts are missing
6. Upload artifacts
7. Attach builds to tagged releases

Final build artifacts must use fail-on-missing behavior, not warning-only behavior.

---

# Release Strategy

Use semantic versioning:

- Patch: bug fixes, e.g. `v1.1.1`
- Minor: backward-compatible features, e.g. `v1.2.0`
- Major: major architecture/UX change, e.g. `v2.0.0`

---

# v1.1 Definition of Done

Do not release v1.1 until:

- HEIC/HEIF works
- AVIF works
- Folder import works
- Include Subfolders works
- Smart Optimize works
- Compression presets work
- Target size presets work
- Before/after comparison works
- Saved presets work
- Metadata cleaner works
- Batch progress works
- Result summary works
- Existing v1.0 features still work
- Automated tests pass
- macOS build passes
- Windows build passes
- Universal DMG generated
- Windows EXE generated
- Windows MSI generated

---

# v1.1 Recommended Development Order

## Phase 1 — Foundation
- Audit v1.0
- Create v1.1 branch
- Add regression tests

## Phase 2 — Formats
- HEIC/HEIF
- AVIF
- Conversion test matrix

## Phase 3 — Batch Input
- Add Folder
- Include Subfolders
- Queue improvements

## Phase 4 — Smart Compression
- Smart Optimize
- Compression presets
- Target size presets

## Phase 5 — UX
- Before/after comparison
- Better batch progress
- Better result summary

## Phase 6 — Utility
- Metadata cleaner
- Saved presets

## Phase 7 — Performance
- Large batches
- Memory usage
- CPU/concurrency tests

## Phase 8 — Release
- macOS build
- Windows build
- Universal DMG
- EXE/MSI
- Release candidate testing

---

# Do Not Add to v1.1

Keep these for later:
- User accounts
- Cloud sync
- AI image generation
- Online processing
- Subscription system
- Team accounts
- OCR
- RAW workflow
- PDF editor
- Full photo editor
- Duplicate finder
- CLI

---

# Immediate Next Task

Start **v1.1** in this exact order:

1. HEIC / HEIF
2. AVIF
3. Add Folder
4. Include Subfolders
5. Smart Optimize
6. Compression Presets
7. Target Size Presets
8. Before / After Comparison
9. Better Batch Progress
10. Saved Presets
11. Metadata Cleaner
12. Better Result Summary

Do not start v1.2 until v1.1 is stable, tested, and packaged successfully for macOS and Windows.

---

# v1.5 — Extra Utility Tools

## Goal
Add practical standalone tools that users often need together with compression and conversion.

## 1. PNG Optimizer
Add a dedicated PNG optimization mode.

Requirements:
- Lossless optimization where possible
- Preserve transparency
- Avoid color shifts
- Show original vs optimized size
- Work in batch mode
- Do not convert to another format unless the user explicitly chooses conversion

## 2. Transparent Image Optimizer
Add a workflow specifically for transparent assets.

Support:
- PNG
- WebP
- AVIF where supported

Requirements:
- Preserve alpha channel
- Avoid black/white accidental backgrounds
- Optimize without visible edge artifacts

## 3. Color Profile / sRGB Converter
Add optional color profile handling.

Options:
- Preserve source profile
- Convert to sRGB
- Remove embedded profile where safe

Useful for:
- Website images
- E-commerce
- Cross-device consistency

## 4. Passport / Document Image Compressor
Add quick presets for documents and online portals.

Presets:
- 20 KB
- 50 KB
- 100 KB
- 200 KB
- 500 KB
- Custom

Use cases:
- Passport photo uploads
- Visa forms
- University forms
- Job applications
- Government portals

Requirements:
- Keep aspect ratio
- Avoid unreadable output
- Show final dimensions and file size
- Warn when requested target is unrealistic

## 5. Custom Document Presets
Allow users to create presets such as:

```text
Visa Upload
Max Size: 200 KB
Format: JPG
Width: 600 px
Background: White
```

Save locally.

## 6. Image to Base64
Add:
- Image → Base64
- Copy Base64
- Save Base64 as text

This should be an advanced utility and should not clutter the main compressor UI.

## 7. Base64 to Image
Allow:
- Paste Base64
- Detect image type
- Preview
- Save as image

## 8. Image Dimensions Inspector
Quick tool showing:
- Width
- Height
- Aspect ratio
- Megapixels
- File size
- Format

Useful without entering the full processing workflow.

## 9. Quick Rotate / Flip
Add simple:
- Rotate 90° left
- Rotate 90° right
- Rotate 180°
- Flip horizontal
- Flip vertical

Must support batch mode where practical.

## 10. Crop Tool
Add a lightweight crop tool.

Modes:
- Free crop
- Original ratio
- 1:1
- 4:5
- 3:4
- 16:9
- 9:16

Do not turn the app into a full photo editor.

## 11. Canvas / Padding Tool
Allow fitting an image inside a new canvas.

Options:
- Custom width/height
- Background color
- Transparent background where format supports it
- Center image
- Fit
- Fill

Useful for product images and social media.

## 12. Border Tool
Simple optional border around images.

Controls:
- Thickness
- Color
- Inner/outer behavior where practical

Keep this tool lightweight.

---

# v1.6 — E-commerce and Content Production Tools

## Goal
Make the app useful for product teams, agencies, Shopify stores and social media teams.

## 1. Product Image Standardizer
User selects a folder and the app makes images consistent.

Options:
- Same canvas size
- Same background
- Same padding
- Same format
- Same quality
- Same naming pattern

Example:

```text
All product images:
2000×2000
White background
WebP
Quality 82
```

## 2. White Background Mode
For transparent product images:

- Add white background
- Keep centered
- Apply configurable padding
- Export JPG/WebP

This is not AI background removal.

## 3. Product Image Padding
Add consistent breathing space around product images.

Options:
- Percentage padding
- Pixel padding
- Auto center
- Background color

## 4. Multi-Format Export
One source can export multiple formats in one run.

Example:

```text
product.jpg
→ product.webp
→ product.avif
→ product.png
```

Useful for web developers.

## 5. Responsive Website Export
Generate multiple widths.

Example:

```text
480w
768w
1024w
1440w
1920w
```

Optional generated HTML:

```html
<picture>...</picture>
```

This should be optional.

## 6. Favicon / Icon Generator
Generate common icon sizes.

Potential outputs:
- 16×16
- 32×32
- 48×48
- 180×180
- 192×192
- 512×512

Formats:
- PNG
- ICO where supported

## 7. Thumbnail Generator
Generate thumbnails in bulk.

Controls:
- Width
- Height
- Fit
- Fill
- Crop
- Quality
- Naming suffix

Example:

```text
photo.jpg
→ photo-thumb.webp
```

## 8. Contact Sheet Generator
Generate a single sheet containing thumbnails for review.

Controls:
- Columns
- Rows
- Thumbnail size
- Filename labels
- Page background

This is for file review, not image-generation collage output.

## 9. Brand Presets
Allow reusable presets such as:

```text
Shopify Product
Instagram Post
Website Hero
Client A
Client B
```

Each preset can store:
- Dimensions
- Format
- Quality
- Metadata
- Naming
- Watermark
- Background

---

# v1.7 — File Management and Cleanup

## Goal
Help users manage large image libraries safely.

## 1. Exact Duplicate Finder
Use:
- File hash
- Pixel hash where useful

Show:
- Duplicate groups
- File sizes
- Potential space saving

Never delete automatically.

## 2. Similar Image Finder
Use perceptual similarity.

Show:
- Similar groups
- Similarity score
- Dimensions
- Size

User decides what to keep.

## 3. Large Image Finder
Scan selected folders and list:
- Largest files
- Highest-resolution images
- Images above selected threshold

Example thresholds:
- > 5 MB
- > 10 MB
- > 20 MB
- Custom

## 4. Oversized Dimension Finder
Find images above:
- 2000 px
- 4000 px
- 8000 px
- Custom

Useful before website uploads.

## 5. Corrupt Image Scanner
Scan folders and identify:
- Unreadable images
- Truncated files
- Unsupported formats

Do not modify files automatically.

## 6. Duplicate Filename Finder
Find different files that share the same filename across folders.

Useful for asset cleanup.

---

# v1.8 — Advanced Automation

## 1. Batch Queue
Allow multiple jobs to be queued.

Example:

```text
Job 1 — Website Images
Job 2 — Social Media
Job 3 — Product Photos
```

Users can:
- Pause
- Resume
- Cancel
- Reorder jobs

## 2. Scheduled Folder Processing
Optional local scheduler.

Example:

```text
Every day at 7 PM
Process /Incoming Images
using Website preset
```

No cloud dependency.

## 3. Preset Import / Export
Allow presets to be exported as a small config file and imported on another computer.

Do not include user file paths unless explicitly requested.

## 4. Processing Reports
Generate a local report containing:
- Files processed
- Failures
- Original size
- Final size
- Space saved
- Duration
- Preset used

Formats:
- TXT
- CSV
- JSON

## 5. Undo Safety
Where possible, do not modify originals.

Default behavior:
- Write new output
- Keep source untouched

If overwrite mode is enabled:
- Show warning
- Offer optional local backup

---

# v2.1 — Optional Advanced Formats

Only add when codec support is stable and licensing/distribution is acceptable.

Potential:
- TIFF
- BMP
- ICO
- JPEG XL
- GIF image conversion
- Animated WebP handling
- Animated AVIF handling
- RAW formats such as CR2, NEF, ARW, DNG

Each format must be tested on both macOS and Windows before release.

---

# v2.2 — PDF and Document Utilities

Keep these as a separate module so the core image app stays clean.

Potential tools:
- Images → PDF
- Multiple images → one PDF
- PDF pages → images
- PDF page image extraction
- PDF image downsampling
- PDF image compression presets

Do not turn the app into a full PDF editor.

---

# v2.3 — Developer Utilities

Optional advanced tab:

- Image → Base64
- Base64 → Image
- Generate `<picture>` markup
- Generate `srcset`
- Responsive width export
- File hash viewer
- MIME type viewer
- Color profile information

These tools should remain hidden from normal users unless they open the Advanced/Developer section.

---

# Feature Prioritization Rule

Before adding any remaining feature, score it against:

1. User demand
2. Daily usefulness
3. Offline compatibility
4. Batch compatibility
5. Performance cost
6. Package-size impact
7. Maintenance complexity
8. Cross-platform reliability

A feature with high complexity and low daily usefulness should remain optional or postponed.

---

# Revised Long-Term Roadmap

## v1.1
Core formats + Smart Optimize + folders + presets + metadata + comparison

## v1.2
Professional batch tools + rename + watermark + DPI + pipeline

## v1.3
Website + Shopify + WordPress + social workflows

## v1.4
Folder Watch + automation + optional CLI

## v1.5
Document compressor + PNG/transparent tools + Base64 + crop/rotate/canvas

## v1.6
E-commerce production + responsive export + thumbnails + brand presets

## v1.7
Duplicate/similar/large/corrupt image cleanup tools

## v1.8
Advanced queue + scheduled processing + reports + preset sharing

## v2.0+
Advanced formats, PDF utilities and developer tools

---

# Important Scope Rule

Do not implement every version at once.

Each release must:
- Keep existing features working
- Pass automated tests
- Build on Windows
- Build on macOS
- Avoid unnecessary package bloat
- Remain usable offline
- Preserve user privacy
- Keep the default UI simple

Advanced tools should be grouped under clear sections instead of crowding the main compression screen.

