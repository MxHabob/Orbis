// ═══════════════════════════════════════════════════════════════
// docforge-engine — Barcode & QR Code Generator
// Generates SVG strings from barcode/QR data using Rust-native crates.
// Binary impact: barcoders ~50KB, qrcode ~50KB
// ═══════════════════════════════════════════════════════════════

use crate::error::DocforgeError;

/// Supported barcode formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarcodeFormat {
    Code128,
    Code39,
    Ean13,
    Ean8,
    Codabar,
}

impl BarcodeFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "code128" | "code-128" => Some(BarcodeFormat::Code128),
            "code39" | "code-39" => Some(BarcodeFormat::Code39),
            "ean13" | "ean-13" => Some(BarcodeFormat::Ean13),
            "ean8" | "ean-8" => Some(BarcodeFormat::Ean8),
            "codabar" => Some(BarcodeFormat::Codabar),
            _ => None,
        }
    }
}

/// Generate a barcode as SVG string.
///
/// # Arguments
fn prepare_code128_data(data: &str) -> String {
    if data.starts_with('À')
        || data.starts_with('Ɓ')
        || data.starts_with('Ć')
        || data.starts_with('\u{00C0}')
        || data.starts_with('\u{0181}')
        || data.starts_with('\u{0106}')
    {
        data.to_string()
    } else {
        let is_pure_digits = data.chars().all(|c| c.is_ascii_digit());
        if is_pure_digits && data.len() % 2 == 0 {
            format!("\u{0106}{}", data)
        } else {
            format!("\u{0181}{}", data)
        }
    }
}

/// Generate a barcode as SVG string.
///
/// # Arguments
/// * `format` - Barcode format
/// * `data` - Data to encode
/// * `width` - Width in PDF points
/// * `height` - Height in PDF points
pub fn generate_barcode_svg(
    format: BarcodeFormat,
    data: &str,
    width: f32,
    height: f32,
) -> Result<String, DocforgeError> {
    use barcoders::sym;

    // Generate the barcode encoding (Vec of 0s and 1s)
    let encoded = match format {
        BarcodeFormat::Code128 => {
            let prepared = prepare_code128_data(data);
            sym::code128::Code128::new(&prepared)
                .map_err(|e| DocforgeError::Render(format!("Code128 error: {}", e)))?
                .encode()
        }
        BarcodeFormat::Code39 => sym::code39::Code39::new(data)
            .map_err(|e| DocforgeError::Render(format!("Code39 error: {}", e)))?
            .encode(),
        BarcodeFormat::Ean13 => sym::ean13::EAN13::new(data)
            .map_err(|e| DocforgeError::Render(format!("EAN13 error: {}", e)))?
            .encode(),
        BarcodeFormat::Ean8 => sym::ean8::EAN8::new(data)
            .map_err(|e| DocforgeError::Render(format!("EAN8 error: {}", e)))?
            .encode(),
        BarcodeFormat::Codabar => sym::codabar::Codabar::new(data)
            .map_err(|e| DocforgeError::Render(format!("Codabar error: {}", e)))?
            .encode(),
    };

    // Convert encoding to SVG manually (lighter than barcoders' built-in SVG)
    let bar_count = encoded.len();
    if bar_count == 0 {
        return Err(DocforgeError::Render("Empty barcode encoding".into()));
    }

    let bar_width = width / bar_count as f32;
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">"#,
        width, height, width, height
    );
    svg.push('\n');

    for (i, &bar) in encoded.iter().enumerate() {
        if bar == 1 {
            let x = i as f32 * bar_width;
            svg.push_str(&format!(
                r#"<rect x="{:.2}" y="0" width="{:.2}" height="{}" fill="black"/>"#,
                x, bar_width, height
            ));
            svg.push('\n');
        }
    }

    svg.push_str("</svg>");
    Ok(svg)
}

/// QR code error correction level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QrErrorCorrection {
    Low,
    Medium,
    Quartile,
    High,
}

impl QrErrorCorrection {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "low" | "l" => QrErrorCorrection::Low,
            "quartile" | "q" => QrErrorCorrection::Quartile,
            "high" | "h" => QrErrorCorrection::High,
            _ => QrErrorCorrection::Medium,
        }
    }

    fn to_ecl(&self) -> qrcode::EcLevel {
        match self {
            QrErrorCorrection::Low => qrcode::EcLevel::L,
            QrErrorCorrection::Medium => qrcode::EcLevel::M,
            QrErrorCorrection::Quartile => qrcode::EcLevel::Q,
            QrErrorCorrection::High => qrcode::EcLevel::H,
        }
    }
}

/// Generate a QR code as SVG string.
///
/// # Arguments
/// * `data` - Data to encode
/// * `size` - Size in PDF points (square)
/// * `ec_level` - Error correction level
/// * `fg_color` - Foreground color (hex, e.g., "#000000")
/// * `bg_color` - Background color (hex, e.g., "#ffffff")
pub fn generate_qr_svg(
    data: &str,
    size: f32,
    ec_level: QrErrorCorrection,
    fg_color: &str,
    bg_color: &str,
) -> Result<String, DocforgeError> {
    use qrcode::QrCode;

    let code = QrCode::with_error_correction_level(data.as_bytes(), ec_level.to_ecl())
        .map_err(|e| DocforgeError::Render(format!("QR code error: {}", e)))?;

    let modules = code.to_colors();
    let module_count = code.width() as f32;
    let module_size = size / module_count;

    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">"#,
        size, size, size, size
    );
    svg.push('\n');

    // Background
    svg.push_str(&format!(
        r#"<rect x="0" y="0" width="{}" height="{}" fill="{}"/>"#,
        size, size, bg_color
    ));
    svg.push('\n');

    // Modules
    let w = code.width();
    for (i, color) in modules.iter().enumerate() {
        if *color == qrcode::Color::Dark {
            let row = i / w;
            let col = i % w;
            let x = col as f32 * module_size;
            let y = row as f32 * module_size;
            svg.push_str(&format!(
                r#"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" fill="{}"/>"#,
                x, y, module_size, module_size, fg_color
            ));
            svg.push('\n');
        }
    }

    svg.push_str("</svg>");
    Ok(svg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barcode_code128() {
        let svg = generate_barcode_svg(BarcodeFormat::Code128, "12345", 200.0, 50.0)
            .expect("Should generate barcode");
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("rect"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn test_qr_code() {
        let svg = generate_qr_svg(
            "https://orbis.edu",
            100.0,
            QrErrorCorrection::Medium,
            "#000000",
            "#ffffff",
        )
        .expect("Should generate QR code");
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("rect"));
    }

    #[test]
    fn test_barcode_format_parsing() {
        assert_eq!(
            BarcodeFormat::from_str("code128"),
            Some(BarcodeFormat::Code128)
        );
        assert_eq!(BarcodeFormat::from_str("EAN13"), Some(BarcodeFormat::Ean13));
        assert_eq!(BarcodeFormat::from_str("unknown"), None);
    }
}
