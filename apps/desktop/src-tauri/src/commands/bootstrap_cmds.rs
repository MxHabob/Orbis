//! Bootstrap Tauri IPC commands — first-time organization initialization,
//! regional environment auto-detection, and smart slug transliteration.

use tauri::State;
use serde::{Deserialize, Serialize};
use chrono::{Datelike, Local};
use crate::state::AppState;

// ═══════════════════════════════════════════════════════════════
// DTOs
// ═══════════════════════════════════════════════════════════════

/// Enriched DTO for bootstrap request via Tauri IPC.
#[derive(Debug, Clone, Deserialize)]
pub struct BootstrapRequest {
    pub institution_type: String,
    pub organization_name: String,
    pub organization_name_ar: Option<String>,
    pub organization_slug: Option<String>,
    pub organization_code: Option<String>,

    // Enriched Organization Profile
    pub educational_track: Option<String>,
    pub organization_size: Option<String>,
    pub country_code: Option<String>,
    pub city: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub tax_id: Option<String>,
    pub license_number: Option<String>,

    // Mulverse Ecosystem Link
    pub mulverse_node_id: Option<String>,

    // Localization & Regional Settings
    pub timezone: String,
    pub locale: String,
    pub date_format: Option<String>,
    pub is_hijri_enabled: Option<bool>,
    pub base_currency_code: Option<String>,

    // Branding
    pub primary_color: Option<String>,

    // Administrator
    pub admin_first_name: String,
    pub admin_last_name: String,
    pub admin_first_name_ar: Option<String>,
    pub admin_last_name_ar: Option<String>,
    pub admin_email: String,
    pub admin_password: String,
    pub admin_phone: Option<String>,
}

/// DTO for bootstrap result via Tauri IPC.
#[derive(Debug, Clone, Serialize)]
pub struct BootstrapResult {
    pub success: bool,
    pub organization_id: String,
    pub organization_name: String,
    pub organization_slug: String,
    pub institution_type: String,
    pub admin_user_id: String,
    pub admin_email: String,
    pub setup_required: bool,
    pub warnings: Vec<String>,
}

/// DTO for bootstrap status via Tauri IPC.
#[derive(Debug, Clone, Serialize)]
pub struct BootstrapStatusResult {
    pub status: String,
    pub organization_id: Option<String>,
    pub organization_name: Option<String>,
    pub institution_type: Option<String>,
    pub has_admin: bool,
}

/// Regional and environment detection result for smart onboarding prefill.
#[derive(Debug, Clone, Serialize)]
pub struct EnvironmentDetectionResult {
    pub detected_locale: String,
    pub detected_language: String,
    pub country_code: String,
    pub country_name_en: String,
    pub country_name_ar: String,
    pub flag_emoji: String,
    pub timezone: String,
    pub timezone_offset: String,
    pub timezone_offset_hours: i32,
    pub currency_code: String,
    pub currency_symbol_en: String,
    pub currency_symbol_ar: String,
    pub currency_name_en: String,
    pub currency_name_ar: String,
    pub is_hijri_recommended: bool,
    pub date_format: String,
    pub suggested_academic_year: String,
    pub suggested_academic_year_ar: String,
}

/// Smart slug generation response.
#[derive(Debug, Clone, Serialize)]
pub struct SlugGenerationResult {
    pub slug: String,
    pub is_available: bool,
    pub suggestions: Vec<String>,
}

// ═══════════════════════════════════════════════════════════════
// Smart Transliteration & Slugification Engine
// ═══════════════════════════════════════════════════════════════

/// Transliterates Arabic and English text to a clean URL-safe slug.
pub fn generate_smart_slug(input_en: &str, input_ar: Option<&str>) -> String {
    let base_text = if !input_en.trim().is_empty() {
        input_en.trim().to_string()
    } else if let Some(ar) = input_ar {
        transliterate_arabic(ar)
    } else {
        String::new()
    };

    if base_text.is_empty() {
        return "organization".to_string();
    }

    slugify_string(&base_text)
}

/// Slugify a string into lowercase kebab-case.
fn slugify_string(text: &str) -> String {
    let mut result = String::new();
    let mut last_was_dash = false;

    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            result.push(c.to_ascii_lowercase());
            last_was_dash = false;
        } else if c == ' ' || c == '-' || c == '_' || c == '.' || c == '/' {
            if !last_was_dash && !result.is_empty() {
                result.push('-');
                last_was_dash = true;
            }
        }
    }

    while result.ends_with('-') {
        result.pop();
    }

    if result.is_empty() {
        "institution".to_string()
    } else {
        result
    }
}

/// Convert Arabic text into clean English phonetic words.
pub fn transliterate_arabic(text: &str) -> String {
    let mut cleaned = text.to_string();

    let replacements = [
        ("مدارس", "madaris"),
        ("مدرسة", "school"),
        ("أكاديمية", "academy"),
        ("اكاديمية", "academy"),
        ("جامعة", "university"),
        ("كلية", "college"),
        ("معهد", "institute"),
        ("مركز", "center"),
        ("الأهلية", "al-ahliya"),
        ("الاهلية", "al-ahliya"),
        ("العالمية", "international"),
        ("الدولية", "international"),
        ("النموذجية", "model"),
        ("الخاصة", "private"),
        ("المستقبل", "future"),
        ("النور", "al-noor"),
        ("الرواد", "al-ruwwad"),
        ("البيان", "al-bayan"),
        ("المجد", "al-majd"),
        ("الإبداع", "al-ibda"),
        ("التميز", "al-tamayuz"),
        ("الأندلس", "al-andalus"),
        ("النهضة", "al-nahda"),
        ("الأمل", "al-amal"),
        ("الريادة", "al-riyadah"),
        ("الفضيلة", "al-fadeela"),
        ("الصفوة", "al-safwa"),
        ("الحكمة", "al-hikma"),
        ("الفرقان", "al-furqan"),
        ("الرياض", "riyadh"),
        ("جدة", "jeddah"),
        ("عدن", "aden"),
        ("صنعاء", "sanaa"),
        ("دبي", "dubai"),
        ("القاهرة", "cairo"),
    ];

    for (ar_word, en_word) in replacements {
        cleaned = cleaned.replace(ar_word, &format!(" {} ", en_word));
    }

    let mut out = String::new();
    for c in cleaned.chars() {
        match c {
            'ا' | 'أ' | 'إ' | 'آ' | 'ء' | 'ئ' | 'ؤ' => out.push('a'),
            'ب' => out.push('b'),
            'ت' | 'ة' => out.push('t'),
            'ث' => out.push_str("th"),
            'ج' => out.push('j'),
            'ح' => out.push('h'),
            'خ' => out.push_str("kh"),
            'د' => out.push('d'),
            'ذ' => out.push_str("dh"),
            'ر' => out.push('r'),
            'ز' => out.push('z'),
            'س' => out.push('s'),
            'ش' => out.push_str("sh"),
            'ص' => out.push('s'),
            'ض' => out.push('d'),
            'ط' => out.push('t'),
            'ظ' => out.push('z'),
            'ع' => out.push('a'),
            'غ' => out.push_str("gh"),
            'ف' => out.push('f'),
            'ق' => out.push('q'),
            'ك' => out.push('k'),
            'ل' => out.push('l'),
            'م' => out.push('m'),
            'ن' => out.push('n'),
            'ه' => out.push('h'),
            'و' => out.push('w'),
            'ي' | 'ى' => out.push('y'),
            '٠' => out.push('0'),
            '١' => out.push('1'),
            '٢' => out.push('2'),
            '٣' => out.push('3'),
            '٤' => out.push('4'),
            '٥' => out.push('5'),
            '٦' => out.push('6'),
            '٧' => out.push('7'),
            '٨' => out.push('8'),
            '٩' => out.push('9'),
            'َ' | 'ً' | 'ُ' | 'ٌ' | 'ِ' | 'ٍ' | 'ْ' | 'ّ' => {},
            other => out.push(other),
        }
    }

    out
}

// ═══════════════════════════════════════════════════════════════
// IPC Commands
// ═══════════════════════════════════════════════════════════════

/// Detect system locale, timezone, currency, country, and regional preferences for onboarding.
#[tauri::command]
pub async fn bootstrap_detect_environment() -> Result<EnvironmentDetectionResult, String> {
    let local_now = Local::now();
    let offset_seconds = local_now.offset().local_minus_utc();
    let offset_hours = offset_seconds / 3600;

    let (
        country_code,
        country_name_en,
        country_name_ar,
        flag_emoji,
        timezone,
        currency_code,
        currency_symbol_en,
        currency_symbol_ar,
        currency_name_en,
        currency_name_ar,
        is_hijri,
    ) = match offset_hours {
        3 => (
            "SA",
            "Saudi Arabia & GCC",
            "المملكة العربية السعودية والخليج",
            "🇸🇦",
            "Asia/Riyadh",
            "SAR",
            "SAR",
            "ر.س",
            "Saudi Riyal",
            "ريال سعودي",
            true,
        ),
        4 => (
            "AE",
            "United Arab Emirates & Oman",
            "الإمارات العربية المتحدة وعمان",
            "🇦🇪",
            "Asia/Dubai",
            "AED",
            "AED",
            "د.إ",
            "UAE Dirham",
            "درهم إماراتي",
            true,
        ),
        2 => (
            "EG",
            "Egypt & Jordan",
            "مصر والأردن",
            "🇪🇬",
            "Africa/Cairo",
            "EGP",
            "EGP",
            "ج.م",
            "Egyptian Pound",
            "جنيه مصري",
            true,
        ),
        1 => (
            "DZ",
            "Algeria & North Africa",
            "الجزائر وتونس والمغرب",
            "🇩🇿",
            "Africa/Algiers",
            "DZD",
            "DZD",
            "د.ج",
            "Algerian Dinar",
            "دينار جزائري",
            true,
        ),
        0 => (
            "GB",
            "United Kingdom / UTC",
            "المملكة المتحدة والتوقيت العالمي",
            "🇬🇧",
            "Europe/London",
            "GBP",
            "GBP (£)",
            "جنيه (£)",
            "British Pound",
            "جنيه إسترليني",
            false,
        ),
        -5 => (
            "US",
            "United States (Eastern)",
            "الولايات المتحدة (التوقيت الشرقي)",
            "🇺🇸",
            "America/New_York",
            "USD",
            "USD ($)",
            "دولار ($)",
            "US Dollar",
            "دولار أمريكي",
            false,
        ),
        _ => (
            "SA",
            "Saudi Arabia",
            "المملكة العربية السعودية",
            "🇸🇦",
            "Asia/Riyadh",
            "SAR",
            "SAR",
            "ر.س",
            "Saudi Riyal",
            "ريال سعودي",
            true,
        ),
    };

    let tz_offset_formatted = if offset_hours >= 0 {
        format!("UTC+{}", offset_hours)
    } else {
        format!("UTC{}", offset_hours)
    };

    let current_year = local_now.year();
    let next_year = current_year + 1;
    let suggested_academic_year = format!("{}/{}", current_year, next_year);
    let suggested_academic_year_ar = format!("{}/{} م", current_year, next_year);

    Ok(EnvironmentDetectionResult {
        detected_locale: "ar-SA".to_string(),
        detected_language: "العربية (Arabic)".to_string(),
        country_code: country_code.to_string(),
        country_name_en: country_name_en.to_string(),
        country_name_ar: country_name_ar.to_string(),
        flag_emoji: flag_emoji.to_string(),
        timezone: timezone.to_string(),
        timezone_offset: tz_offset_formatted,
        timezone_offset_hours: offset_hours,
        currency_code: currency_code.to_string(),
        currency_symbol_en: currency_symbol_en.to_string(),
        currency_symbol_ar: currency_symbol_ar.to_string(),
        currency_name_en: currency_name_en.to_string(),
        currency_name_ar: currency_name_ar.to_string(),
        is_hijri_recommended: is_hijri,
        date_format: "YYYY-MM-DD".to_string(),
        suggested_academic_year,
        suggested_academic_year_ar,
    })
}

/// Generate a smart, validated Latin slug with uniqueness verification.
#[tauri::command]
pub async fn bootstrap_generate_slug(
    state: State<'_, AppState>,
    name: String,
    name_ar: Option<String>,
) -> Result<SlugGenerationResult, String> {
    let mut candidate_slug = generate_smart_slug(&name, name_ar.as_deref());

    if candidate_slug.is_empty() {
        candidate_slug = "organization".to_string();
    }

    let existing = state.org_repo.find_by_slug(&candidate_slug).await
        .map_err(|e| e.to_string())?;

    let is_available = existing.is_none();

    let mut suggestions = Vec::new();
    if !is_available {
        suggestions.push(format!("{}-edu", candidate_slug));
        suggestions.push(format!("{}-academy", candidate_slug));
        suggestions.push(format!("{}-core", candidate_slug));
    }

    Ok(SlugGenerationResult {
        slug: candidate_slug,
        is_available,
        suggestions,
    })
}

/// Get bootstrap status — check if the system needs initialization.
#[tauri::command]
pub async fn bootstrap_get_status(
    state: State<'_, AppState>,
) -> Result<BootstrapStatusResult, String> {
    let status = state.bootstrap_service.get_status().await
        .map_err(|e| e.to_string())?;

    Ok(BootstrapStatusResult {
        status: match status.status {
            orbis_app::services::BootstrapStatus::Required => "required".to_string(),
            orbis_app::services::BootstrapStatus::InProgress => "in_progress".to_string(),
            orbis_app::services::BootstrapStatus::Completed => "completed".to_string(),
        },
        organization_id: status.organization_id.map(|id| id.to_string()),
        organization_name: status.organization_name,
        institution_type: status.institution_type,
        has_admin: status.has_admin,
    })
}

/// Execute bootstrap — create the first organization and administrator.
#[tauri::command]
pub async fn bootstrap_execute(
    state: State<'_, AppState>,
    request: BootstrapRequest,
) -> Result<BootstrapResult, String> {
    let service_req = orbis_app::services::BootstrapRequest {
        institution_type: request.institution_type,
        organization_name: request.organization_name,
        organization_name_ar: request.organization_name_ar,
        organization_slug: request.organization_slug,
        organization_code: request.organization_code,
        educational_track: request.educational_track,
        organization_size: request.organization_size,
        country_code: request.country_code,
        city: request.city,
        phone: request.phone,
        website: request.website,
        tax_id: request.tax_id,
        license_number: request.license_number,
        mulverse_node_id: request.mulverse_node_id,
        timezone: request.timezone,
        locale: request.locale,
        date_format: request.date_format,
        is_hijri_enabled: request.is_hijri_enabled,
        base_currency_code: request.base_currency_code,
        primary_color: request.primary_color,
        admin_first_name: request.admin_first_name,
        admin_last_name: request.admin_last_name,
        admin_first_name_ar: request.admin_first_name_ar,
        admin_last_name_ar: request.admin_last_name_ar,
        admin_email: request.admin_email,
        admin_password: request.admin_password,
        admin_phone: request.admin_phone,
    };

    let result = state.bootstrap_service.execute(service_req).await
        .map_err(|e| e.to_string())?;

    Ok(BootstrapResult {
        success: result.success,
        organization_id: result.organization_id.to_string(),
        organization_name: result.organization_name,
        organization_slug: result.organization_slug,
        institution_type: result.institution_type,
        admin_user_id: result.admin_user_id.to_string(),
        admin_email: result.admin_email,
        setup_required: result.setup_required,
        warnings: result.warnings,
    })
}
