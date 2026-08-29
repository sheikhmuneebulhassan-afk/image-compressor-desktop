use image::{codecs::{avif::AvifEncoder, jpeg::JpegEncoder, png::{CompressionType, FilterType, PngEncoder}}, imageops::FilterType as ResizeFilter, DynamicImage, GenericImageView, ImageEncoder};
use serde::{Deserialize, Serialize};
use std::{fs, path::{Path, PathBuf}, process::Command};
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, Serialize)]
struct ImageInfo {
    path: String,
    name: String,
    size: u64,
    width: u32,
    height: u32,
    format: String,
}

#[derive(Debug, Deserialize, Clone)]
struct ProcessOptions {
    format: String,
    quality: u8,
    target_kb: Option<u64>,
    resize_enabled: bool,
    width: Option<u32>,
    height: Option<u32>,
    keep_aspect: bool,
    suffix: String,
    overwrite: bool,
}

#[derive(Debug, Serialize)]
struct ProcessResult {
    input_path: String,
    output_path: Option<String>,
    original_bytes: u64,
    output_bytes: u64,
    width: u32,
    height: u32,
    format: String,
    quality_used: u8,
    error: Option<String>,
}

#[tauri::command]
async fn pick_images(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let files = app
        .dialog()
        .file()
        .add_filter("Images", &["jpg", "jpeg", "png", "webp", "avif"])
        .blocking_pick_files();

    Ok(files
        .unwrap_or_default()
        .into_iter()
        .filter_map(|f| f.as_path().map(|p| p.to_string_lossy().to_string()))
        .collect())
}

#[tauri::command]
async fn pick_output_folder(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let default = dirs::picture_dir().or_else(dirs::download_dir);
    let mut picker = app.dialog().file();
    if let Some(dir) = default { picker = picker.set_directory(dir); }
    let folder = picker.blocking_pick_folder();
    Ok(folder.and_then(|f| f.as_path().map(|p| p.to_string_lossy().to_string())))
}

#[tauri::command]
fn get_images_info(paths: Vec<String>) -> Result<Vec<ImageInfo>, String> {
    paths.iter().map(|p| image_info(Path::new(p))).collect()
}

fn image_info(path: &Path) -> Result<ImageInfo, String> {
    let meta = fs::metadata(path).map_err(|e| format!("Could not read {}: {e}", path.display()))?;
    let img = image::ImageReader::open(path)
        .map_err(|e| format!("Could not open {}: {e}", path.display()))?
        .with_guessed_format()
        .map_err(|e| format!("Could not detect image format: {e}"))?
        .decode()
        .map_err(|e| format!("Could not decode {}: {e}", path.display()))?;
    let (width, height) = img.dimensions();
    let format = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
    Ok(ImageInfo {
        path: path.to_string_lossy().to_string(),
        name: path.file_name().and_then(|s| s.to_str()).unwrap_or("image").to_string(),
        size: meta.len(), width, height, format,
    })
}

#[tauri::command]
fn process_images(paths: Vec<String>, output_dir: String, options: ProcessOptions) -> Result<Vec<ProcessResult>, String> {
    let output_dir = PathBuf::from(output_dir);
    fs::create_dir_all(&output_dir).map_err(|e| format!("Could not create output folder: {e}"))?;
    Ok(paths
        .iter()
        .map(|path| match process_one(Path::new(path), &output_dir, &options) {
            Ok(result) => result,
            Err(e) => ProcessResult {
                input_path: path.clone(),
                output_path: None,
                original_bytes: 0,
                output_bytes: 0,
                width: 0,
                height: 0,
                format: String::new(),
                quality_used: 0,
                error: Some(e),
            },
        })
        .collect())
}

fn process_one(input: &Path, output_dir: &Path, options: &ProcessOptions) -> Result<ProcessResult, String> {
    let original_bytes = fs::metadata(input).map_err(|e| format!("Could not read input file: {e}"))?.len();
    let mut img = image::ImageReader::open(input)
        .map_err(|e| format!("Could not open {}: {e}", input.display()))?
        .with_guessed_format().map_err(|e| e.to_string())?
        .decode().map_err(|e| format!("Could not decode {}: {e}", input.display()))?;

    if options.resize_enabled {
        img = resize_image(img, options.width, options.height, options.keep_aspect);
    }

    let input_ext = input.extension().and_then(|s| s.to_str()).unwrap_or("jpg").to_lowercase();
    let mut output_format = if options.format == "same" { normalize_format(&input_ext) } else { normalize_format(&options.format) };
    if !matches!(output_format.as_str(), "jpg" | "png" | "webp" | "avif") { output_format = "jpg".to_string(); }

    let target_bytes = options.target_kb.map(|kb| kb.saturating_mul(1024));
    let (encoded, final_img, quality_used) = encode_with_target(img, &output_format, options.quality.clamp(1, 100), target_bytes)?;
    let (width, height) = final_img.dimensions();

    let stem = input.file_stem().and_then(|s| s.to_str()).unwrap_or("image");
    let suffix = sanitize_suffix(&options.suffix);
    let desired = output_dir.join(format!("{stem}{suffix}.{output_format}"));
    let output_path = unique_path(desired, options.overwrite);
    fs::write(&output_path, &encoded).map_err(|e| format!("Could not write {}: {e}", output_path.display()))?;

    Ok(ProcessResult {
        input_path: input.to_string_lossy().to_string(),
        output_path: Some(output_path.to_string_lossy().to_string()),
        original_bytes,
        output_bytes: encoded.len() as u64,
        width, height,
        format: output_format,
        quality_used,
        error: None,
    })
}

fn normalize_format(value: &str) -> String {
    match value.to_lowercase().as_str() {
        "jpeg" | "jpg" => "jpg".into(),
        "png" => "png".into(),
        "webp" => "webp".into(),
        other => other.to_string(),
    }
}

fn resize_image(img: DynamicImage, width: Option<u32>, height: Option<u32>, keep_aspect: bool) -> DynamicImage {
    match (width.filter(|v| *v > 0), height.filter(|v| *v > 0)) {
        (None, None) => img,
        (Some(w), None) => img.resize(w, u32::MAX, ResizeFilter::Lanczos3),
        (None, Some(h)) => img.resize(u32::MAX, h, ResizeFilter::Lanczos3),
        (Some(w), Some(h)) if keep_aspect => img.resize(w, h, ResizeFilter::Lanczos3),
        (Some(w), Some(h)) => img.resize_exact(w, h, ResizeFilter::Lanczos3),
    }
}

fn encode_with_target(mut img: DynamicImage, format: &str, quality: u8, target: Option<u64>) -> Result<(Vec<u8>, DynamicImage, u8), String> {
    let Some(target_bytes) = target else {
        let data = encode_image(&img, format, quality)?;
        return Ok((data, img, quality));
    };

    if format == "png" {
        let mut data = encode_image(&img, format, quality)?;
        while data.len() as u64 > target_bytes && img.width() > 96 && img.height() > 96 {
            let nw = ((img.width() as f32) * 0.9).round().max(96.0) as u32;
            let nh = ((img.height() as f32) * 0.9).round().max(96.0) as u32;
            img = img.resize_exact(nw, nh, ResizeFilter::Lanczos3);
            data = encode_image(&img, format, quality)?;
        }
        return Ok((data, img, quality));
    }

    loop {
        let mut low = 5u8;
        let mut high = quality.max(5);
        let mut best: Option<(Vec<u8>, u8)> = None;
        while low <= high {
            let mid = low + (high - low) / 2;
            let data = encode_image(&img, format, mid)?;
            if data.len() as u64 <= target_bytes {
                best = Some((data, mid));
                low = mid.saturating_add(1);
            } else {
                if mid == 0 { break; }
                high = mid.saturating_sub(1);
            }
        }
        if let Some((data, q)) = best { return Ok((data, img, q)); }

        if img.width() <= 96 || img.height() <= 96 {
            let data = encode_image(&img, format, 5)?;
            return Ok((data, img, 5));
        }
        let nw = ((img.width() as f32) * 0.88).round().max(96.0) as u32;
        let nh = ((img.height() as f32) * 0.88).round().max(96.0) as u32;
        img = img.resize_exact(nw, nh, ResizeFilter::Lanczos3);
    }
}

fn encode_image(img: &DynamicImage, format: &str, quality: u8) -> Result<Vec<u8>, String> {
    match format {
        "jpg" => encode_jpeg(img, quality),
        "png" => encode_png(img),
        "webp" => encode_webp(img, quality),
        "avif" => encode_avif(img, quality),
        _ => Err(format!("Unsupported output format: {format}")),
    }
}

fn encode_jpeg(img: &DynamicImage, quality: u8) -> Result<Vec<u8>, String> {
    let rgba = img.to_rgba8();
    let mut rgb = Vec::with_capacity((rgba.width() * rgba.height() * 3) as usize);
    for px in rgba.pixels() {
        let a = px[3] as u16;
        for channel in 0..3 {
            let c = px[channel] as u16;
            let blended = (c * a + 255 * (255 - a) + 127) / 255;
            rgb.push(blended as u8);
        }
    }
    let mut out = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut out, quality);
    encoder.encode(&rgb, rgba.width(), rgba.height(), image::ExtendedColorType::Rgb8)
        .map_err(|e| format!("JPEG encode failed: {e}"))?;
    Ok(out)
}

fn encode_png(img: &DynamicImage) -> Result<Vec<u8>, String> {
    let rgba = img.to_rgba8();
    let mut out = Vec::new();
    let encoder = PngEncoder::new_with_quality(&mut out, CompressionType::Best, FilterType::Adaptive);
    encoder.write_image(rgba.as_raw(), rgba.width(), rgba.height(), image::ExtendedColorType::Rgba8)
        .map_err(|e| format!("PNG encode failed: {e}"))?;
    Ok(out)
}

fn encode_webp(img: &DynamicImage, quality: u8) -> Result<Vec<u8>, String> {
    let rgba = img.to_rgba8();
    let encoder = webp::Encoder::from_rgba(rgba.as_raw(), rgba.width(), rgba.height());
    let memory = encoder.encode(quality as f32);
    Ok(memory.to_vec())
}

fn encode_avif(img: &DynamicImage, quality: u8) -> Result<Vec<u8>, String> {
    let rgba = img.to_rgba8();
    let mut out = Vec::new();
    let encoder = AvifEncoder::new_with_speed_quality(&mut out, 9, quality.clamp(1, 100));
    encoder.write_image(rgba.as_raw(), rgba.width(), rgba.height(), image::ExtendedColorType::Rgba8)
        .map_err(|e| format!("AVIF encode failed: {e}"))?;
    Ok(out)
}

fn sanitize_suffix(value: &str) -> String {
    let s: String = value.chars().filter(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | ' ')).collect();
    if s.trim().is_empty() { "-compressed".into() } else { s }
}

fn unique_path(path: PathBuf, overwrite: bool) -> PathBuf {
    if overwrite || !path.exists() { return path; }
    let parent = path.parent().map(Path::to_path_buf).unwrap_or_default();
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("image");
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    for i in 2..10000 {
        let candidate = if ext.is_empty() { parent.join(format!("{stem}-{i}")) } else { parent.join(format!("{stem}-{i}.{ext}")) };
        if !candidate.exists() { return candidate; }
    }
    path
}

#[tauri::command]
fn open_folder(path: String) -> Result<(), String> {
    let folder = PathBuf::from(path);
    if !folder.exists() { return Err("Output folder does not exist.".into()); }

    #[cfg(target_os = "windows")]
    let status = Command::new("explorer").arg(&folder).status();
    #[cfg(target_os = "macos")]
    let status = Command::new("open").arg(&folder).status();
    #[cfg(target_os = "linux")]
    let status = Command::new("xdg-open").arg(&folder).status();

    status.map_err(|e| format!("Could not open folder: {e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> PathBuf {
        let mut dir = std::env::temp_dir();
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        dir.push(format!("ic-test-{nanos}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn solid_image(w: u32, h: u32, rgba: [u8; 4]) -> DynamicImage {
        DynamicImage::ImageRgba8(RgbaImage::from_fn(w, h, |_, _| Rgba(rgba)))
    }

    #[test]
    fn jpeg_encode_blends_transparency_to_white_not_black() {
        let img = solid_image(4, 4, [10, 20, 30, 0]);
        let data = encode_jpeg(&img, 90).unwrap();
        let decoded = image::load_from_memory(&data).unwrap().to_rgba8();
        let px = decoded.get_pixel(0, 0);
        assert!(px[0] > 200 && px[1] > 200 && px[2] > 200, "expected near-white background, got {:?}", px);
    }

    #[test]
    fn avif_encode_decode_roundtrips_dimensions_and_alpha() {
        let img = solid_image(20, 16, [30, 120, 90, 128]);
        let data = encode_avif(&img, 70).unwrap();
        let decoded = image::load_from_memory(&data).unwrap().to_rgba8();
        assert_eq!(decoded.dimensions(), (20, 16));
        let px = decoded.get_pixel(0, 0);
        assert!(px[3] > 100 && px[3] < 156, "expected alpha near 128, got {}", px[3]);
    }

    #[test]
    fn png_encode_preserves_transparency() {
        let img = solid_image(4, 4, [10, 20, 30, 0]);
        let data = encode_png(&img).unwrap();
        let decoded = image::load_from_memory(&data).unwrap().to_rgba8();
        assert_eq!(decoded.get_pixel(0, 0)[3], 0);
    }

    #[test]
    fn resize_width_only_preserves_aspect_ratio() {
        let img = solid_image(200, 100, [255, 0, 0, 255]);
        let resized = resize_image(img, Some(100), None, true);
        assert_eq!(resized.dimensions(), (100, 50));
    }

    #[test]
    fn resize_exact_without_aspect_lock_stretches() {
        let img = solid_image(200, 100, [255, 0, 0, 255]);
        let resized = resize_image(img, Some(50), Some(50), false);
        assert_eq!(resized.dimensions(), (50, 50));
    }

    #[test]
    fn resize_no_dimensions_returns_unchanged() {
        let img = solid_image(200, 100, [255, 0, 0, 255]);
        let resized = resize_image(img, None, None, true);
        assert_eq!(resized.dimensions(), (200, 100));
    }

    #[test]
    fn target_kb_compression_stays_near_target_for_jpeg() {
        let img = RgbaImage::from_fn(300, 300, |x, y| {
            Rgba([((x * 7 + y * 3) % 256) as u8, ((x * 13) % 256) as u8, ((y * 11) % 256) as u8, 255])
        });
        let target = 15_000u64;
        let (data, _final_img, quality) = encode_with_target(DynamicImage::ImageRgba8(img), "jpg", 90, Some(target)).unwrap();
        assert!(data.len() as u64 <= target || quality <= 5, "expected near target or minimum quality, got {} bytes at q{}", data.len(), quality);
    }

    #[test]
    fn target_kb_does_not_loop_forever_on_impossible_target() {
        let img = solid_image(64, 64, [128, 64, 200, 255]);
        let (data, _img, _q) = encode_with_target(img, "jpg", 90, Some(1)).unwrap();
        assert!(!data.is_empty());
    }

    #[test]
    fn unique_path_adds_numeric_suffix_when_file_exists() {
        let dir = temp_dir();
        let base = dir.join("photo.jpg");
        fs::write(&base, b"x").unwrap();
        let next = unique_path(base.clone(), false);
        assert_eq!(next, dir.join("photo-2.jpg"));
        fs::write(&next, b"x").unwrap();
        let next2 = unique_path(base.clone(), false);
        assert_eq!(next2, dir.join("photo-3.jpg"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn unique_path_overwrite_returns_same_path() {
        let dir = temp_dir();
        let base = dir.join("photo.jpg");
        fs::write(&base, b"x").unwrap();
        let same = unique_path(base.clone(), true);
        assert_eq!(same, base);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn one_failed_image_does_not_cancel_the_batch() {
        let dir = temp_dir();
        let good_path = dir.join("good.png");
        solid_image(20, 20, [1, 2, 3, 255]).save_with_format(&good_path, image::ImageFormat::Png).unwrap();
        let missing_path = dir.join("does-not-exist.png");

        let options = ProcessOptions {
            format: "jpg".into(), quality: 80, target_kb: None,
            resize_enabled: false, width: None, height: None, keep_aspect: true,
            suffix: "-out".into(), overwrite: true,
        };
        let out_dir = dir.join("out");
        let results = process_images(
            vec![good_path.to_string_lossy().to_string(), missing_path.to_string_lossy().to_string()],
            out_dir.to_string_lossy().to_string(),
            options,
        ).unwrap();

        assert_eq!(results.len(), 2);
        assert!(results[0].error.is_none(), "expected good image to succeed: {:?}", results[0].error);
        assert!(results[0].output_path.is_some());
        assert!(results[1].error.is_some(), "expected missing image to report an error, not abort the batch");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn conversion_png_to_avif_and_avif_to_jpg_via_process_images() {
        let dir = temp_dir();
        let png_path = dir.join("source.png");
        solid_image(24, 24, [200, 40, 40, 255]).save_with_format(&png_path, image::ImageFormat::Png).unwrap();
        let out_dir = dir.join("out");

        let to_avif = ProcessOptions {
            format: "avif".into(), quality: 70, target_kb: None,
            resize_enabled: false, width: None, height: None, keep_aspect: true,
            suffix: "-avif".into(), overwrite: true,
        };
        let results = process_images(vec![png_path.to_string_lossy().to_string()], out_dir.to_string_lossy().to_string(), to_avif).unwrap();
        assert!(results[0].error.is_none(), "PNG->AVIF failed: {:?}", results[0].error);
        assert_eq!(results[0].format, "avif");
        let avif_path = results[0].output_path.clone().unwrap();

        let infos = get_images_info(vec![avif_path.clone()]).unwrap();
        assert_eq!(infos[0].width, 24);
        assert_eq!(infos[0].height, 24);

        let to_jpg = ProcessOptions {
            format: "jpg".into(), quality: 80, target_kb: None,
            resize_enabled: false, width: None, height: None, keep_aspect: true,
            suffix: "-jpg".into(), overwrite: true,
        };
        let jpg_results = process_images(vec![avif_path], out_dir.to_string_lossy().to_string(), to_jpg).unwrap();
        assert!(jpg_results[0].error.is_none(), "AVIF->JPG failed: {:?}", jpg_results[0].error);
        assert_eq!(jpg_results[0].format, "jpg");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn conversion_png_to_webp_roundtrips_and_reads_back() {
        let img = solid_image(30, 30, [5, 100, 200, 255]);
        let data = encode_webp(&img, 85).unwrap();
        let decoded = image::load_from_memory(&data).unwrap();
        assert_eq!(decoded.dimensions(), (30, 30));
    }

    #[test]
    fn get_images_info_reports_dimensions_and_format() {
        let dir = temp_dir();
        let path = dir.join("info.png");
        solid_image(12, 34, [0, 0, 0, 255]).save_with_format(&path, image::ImageFormat::Png).unwrap();
        let infos = get_images_info(vec![path.to_string_lossy().to_string()]).unwrap();
        assert_eq!(infos.len(), 1);
        assert_eq!(infos[0].width, 12);
        assert_eq!(infos[0].height, 34);
        assert_eq!(infos[0].format, "png");
        let _ = fs::remove_dir_all(&dir);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            pick_images,
            pick_output_folder,
            get_images_info,
            process_images,
            open_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running Image Compressor");
}
