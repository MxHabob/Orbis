// ═══════════════════════════════════════════════════════════════
// docforge-engine — Image Loader
// Decodes PNG/JPEG image bytes for embedding in PDFs.
// Uses the `image` crate (already in the project for Tauri tray icons).
// ═══════════════════════════════════════════════════════════════

use crate::error::DocforgeError;

/// Decoded image data ready for PDF embedding
#[derive(Debug, Clone)]
pub struct DecodedImage {
    pub width: u32,
    pub height: u32,
    pub color_type: ImageColorType,
    /// Raw pixel data (RGBA or RGB bytes)
    pub pixels: Vec<u8>,
    /// Original bytes (for direct PDF embedding when possible)
    pub original_bytes: Vec<u8>,
    pub mime_type: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageColorType {
    Rgb,
    Rgba,
    Grayscale,
    GrayscaleAlpha,
}

/// Decode image bytes (PNG or JPEG) into pixel data.
///
/// JPEG images can often be embedded directly into PDF without re-encoding,
/// so we keep the original bytes.
pub fn decode_image(data: &[u8]) -> Result<DecodedImage, DocforgeError> {

    let img = image::load_from_memory(data)
        .map_err(|e| DocforgeError::Render(format!("Image decode error: {}", e)))?;

    let width = img.width();
    let height = img.height();

    // Determine color type and get raw pixels
    let (color_type, pixels) = match img.color() {
        image::ColorType::Rgb8 | image::ColorType::Rgb16 | image::ColorType::Rgb32F => {
            (ImageColorType::Rgb, img.to_rgb8().into_raw())
        }
        image::ColorType::Rgba8 | image::ColorType::Rgba16 | image::ColorType::Rgba32F => {
            (ImageColorType::Rgba, img.to_rgba8().into_raw())
        }
        image::ColorType::L8 | image::ColorType::L16 => {
            (ImageColorType::Grayscale, img.to_luma8().into_raw())
        }
        image::ColorType::La8 | image::ColorType::La16 => {
            (ImageColorType::GrayscaleAlpha, img.to_luma_alpha8().into_raw())
        }
        _ => {
            // Fallback: convert to RGBA
            (ImageColorType::Rgba, img.to_rgba8().into_raw())
        }
    };

    // Detect MIME type from magic bytes
    let mime_type = detect_mime(data);

    Ok(DecodedImage {
        width,
        height,
        color_type,
        pixels,
        original_bytes: data.to_vec(),
        mime_type,
    })
}

/// Detect image MIME type from magic bytes
fn detect_mime(data: &[u8]) -> String {
    if data.len() >= 8 && &data[0..8] == b"\x89PNG\r\n\x1a\n" {
        "image/png".to_string()
    } else if data.len() >= 2 && &data[0..2] == b"\xff\xd8" {
        "image/jpeg".to_string()
    } else {
        "application/octet-stream".to_string()
    }
}

/// Calculate the aspect-ratio-preserved dimensions for an image
/// within a bounding box.
pub fn fit_dimensions(
    img_width: u32,
    img_height: u32,
    box_width: f32,
    box_height: f32,
    fit: &str,
) -> (f32, f32) {
    let _aspect = img_width as f32 / img_height as f32;

    match fit {
        "contain" => {
            // Scale to fit within box, preserving aspect ratio
            let scale = (box_width / img_width as f32).min(box_height / img_height as f32);
            (img_width as f32 * scale, img_height as f32 * scale)
        }
        "cover" => {
            // Scale to cover box, preserving aspect ratio
            let scale = (box_width / img_width as f32).max(box_height / img_height as f32);
            (img_width as f32 * scale, img_height as f32 * scale)
        }
        "fill" | _ => {
            // Stretch to fill box
            (box_width, box_height)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_png() {
        let data = b"\x89PNG\r\n\x1a\nsome data";
        assert_eq!(detect_mime(data), "image/png");
    }

    #[test]
    fn test_detect_jpeg() {
        let data = b"\xff\xd8\xff\xe0some data";
        assert_eq!(detect_mime(data), "image/jpeg");
    }

    #[test]
    fn test_fit_contain() {
        let (w, h) = fit_dimensions(200, 100, 100.0, 100.0, "contain");
        assert!((w - 100.0).abs() < 0.01);
        assert!((h - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_fit_cover() {
        let (w, h) = fit_dimensions(200, 100, 100.0, 100.0, "cover");
        assert!((w - 200.0).abs() < 0.01);
        assert!((h - 100.0).abs() < 0.01);
    }
}
