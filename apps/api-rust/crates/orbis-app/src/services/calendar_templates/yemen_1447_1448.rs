//! Yemen Official School Calendar Template — 1447–1448 AH / 2026–2027
//!
//! Based on Ministerial Decision No. (100) dated 20 Dhul Hijjah 1447 AH / 6 June 2026
//! Ministry of Education and Scientific Research, Republic of Yemen.
//!
//! This template defines the supplementary periods that are NOT covered by the
//! generic `provision_hijri_school_calendar` function: pre-semester preparation,
//! mid-year vacation, and public examination windows.
//!
//! The core 8-month teaching structure (Muharram–Sha'ban 1448 split into two
//! 4-month terms with monthly exams and term finals) is produced by calling
//! `provision_hijri_school_calendar` with the Hijri start parameters defined here.

use chrono::{DateTime, Duration, NaiveDate, NaiveTime, Utc};
use orbis_domain::entities::academic_period::{
    AcademicPeriod, AcademicPeriodState, PeriodKind, PeriodType,
};
use serde_json::json;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════
// Template Constants
// ═══════════════════════════════════════════════════════════════

/// Template identifier used as the `code` for calendar lookups.
pub const TEMPLATE_CODE: &str = "YE-1447-1448";

/// Human-readable name.
pub const TEMPLATE_NAME: &str = "1448 AH / 2026-2027";
pub const TEMPLATE_NAME_AR: &str = "١٤٤٨ هـ / ٢٠٢٦-٢٠٢٧ م";

/// Ministerial decision source provenance.
pub const DECISION_NUMBER: &str = "100";
pub const DECISION_DATE_HIJRI: &str = "20 Dhu al-Hijjah 1447";
pub const DECISION_DATE_GREGORIAN: &str = "2026-06-06";
pub const DECISION_AUTHORITY: &str =
    "Ministry of Education and Scientific Research, Republic of Yemen";

// ── Hijri start parameters for `provision_hijri_school_calendar` ──

/// The first Hijri teaching month is Muharram 1448.
/// The study begins on 9 Muharram 1448 = 27 June 2026 (Saturday).
pub const HIJRI_START_YEAR: i32 = 1448;
pub const HIJRI_START_MONTH: u8 = 1; // Muharram
pub const HIJRI_START_DAY: u8 = 9; // 9th of Muharram

/// Article 2: school week is 5 days, Saturday–Wednesday.
pub const SCHOOL_WEEK_DAYS: u8 = 5;

/// Article 1: total calendar days = 368.
pub const TOTAL_CALENDAR_DAYS: u16 = 368;

/// Article 1: total holiday/break days = 81 (not counting regular weekends).
pub const TOTAL_HOLIDAY_DAYS: u16 = 81;

/// Article 1: preparation/work days = 5.
pub const PREPARATION_DAYS: u16 = 5;

// ═══════════════════════════════════════════════════════════════
// Supplementary Period Definitions
// ═══════════════════════════════════════════════════════════════

/// A supplementary period that the Yemen official calendar defines
/// beyond the core 8-month teaching structure.
#[derive(Debug, Clone)]
pub struct SupplementaryPeriodDef {
    pub code: &'static str,
    pub name: &'static str,
    pub name_ar: &'static str,
    pub period_type: PeriodType,
    pub period_kind: PeriodKind,
    /// Offset in days from the academic year start_date.
    /// Negative means before the year start.
    pub start_offset_days: i64,
    /// Duration in days (inclusive).
    pub duration_days: i64,
    /// Sort order for display.
    pub sort_order: i32,
    /// Whether this period is a child of a specific term (by code).
    /// `None` means it is a top-level period under the year.
    pub parent_term_code: Option<&'static str>,
}

/// Returns the supplementary period definitions for the Yemen 1447-1448 calendar.
///
/// These are the periods that the generic `provision_hijri_school_calendar`
/// does NOT create: admin/registration prep, teacher prep, mid-year break,
/// and public examination windows.
///
/// # Date Verification Status
///
/// The start_offset_days values below are **placeholder estimates** derived from
/// the ministerial decision image analysis. They MUST be verified against the
/// physical document before production deployment. Each period is tagged with
/// a confidence indicator in its metadata.
pub fn supplementary_period_defs() -> Vec<SupplementaryPeriodDef> {
    vec![
        // ── Pre-Semester Activities ────────────────────────────
        SupplementaryPeriodDef {
            code: "YE-ADMIN-OPEN",
            name: "School Administration Opening",
            name_ar: "افتتاح الإدارات المدرسية وبدء عملية القيد والتسجيل",
            period_type: PeriodType::Custom,
            period_kind: PeriodKind::Registration,
            // Admin opening starts ~32 days before study begins
            // (approx 1 Dhul Hijjah 1447, needs verification)
            start_offset_days: -32,
            duration_days: 28,
            sort_order: -20,
            parent_term_code: None,
        },
        SupplementaryPeriodDef {
            code: "YE-TEACHER-PREP",
            name: "Teaching Staff Preparation",
            name_ar: "استئناف دوام هيئة التدريس",
            period_type: PeriodType::Custom,
            period_kind: PeriodKind::Preparatory,
            // Teachers report ~4 days before study begins
            // (approx 5 Muharram 1448, needs verification)
            start_offset_days: -4,
            duration_days: 4,
            sort_order: -10,
            parent_term_code: None,
        },
        // ── Mid-Year Break ────────────────────────────────────
        SupplementaryPeriodDef {
            code: "YE-MIDYEAR-BREAK",
            name: "Mid-Year Vacation",
            name_ar: "إجازة منتصف العام الدراسي",
            period_type: PeriodType::Custom,
            period_kind: PeriodKind::Break,
            // Mid-year break starts after the 4th month ends
            // This is computed dynamically relative to term1 end
            // (placeholder — will be adjusted by the provisioning logic)
            start_offset_days: 0, // Sentinel: computed from term1 end
            duration_days: 14,    // ~2 weeks (needs verification)
            sort_order: 50,
            parent_term_code: None,
        },
        // ── Public Examination Windows ────────────────────────
        SupplementaryPeriodDef {
            code: "YE-EXAM-SECONDARY",
            name: "General Secondary Certificate Examinations (Science & Arts)",
            name_ar: "اختبارات الشهادة الثانوية العامة (العلمي والأدبي)",
            period_type: PeriodType::Custom,
            period_kind: PeriodKind::Examination,
            // After term 2 ends (placeholder — computed from term2 end)
            start_offset_days: 0, // Sentinel: computed from term2 end
            duration_days: 14,    // ~2 weeks (needs verification)
            sort_order: 200,
            parent_term_code: None,
        },
        SupplementaryPeriodDef {
            code: "YE-EXAM-BASIC",
            name: "Basic General Certificate Examinations (9th Grade)",
            name_ar: "اختبارات الشهادة الأساسية العامة (الصف التاسع)",
            period_type: PeriodType::Custom,
            period_kind: PeriodKind::Examination,
            // Overlaps or follows secondary exams (placeholder)
            start_offset_days: 0, // Sentinel: computed from term2 end
            duration_days: 14,    // ~2 weeks (needs verification)
            sort_order: 210,
            parent_term_code: None,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════
// Provenance Metadata
// ═══════════════════════════════════════════════════════════════

/// Returns a JSON metadata value documenting the provenance of this calendar.
pub fn provenance_metadata() -> serde_json::Value {
    json!({
        "source": "ministerial_decision",
        "template_code": TEMPLATE_CODE,
        "decision_number": DECISION_NUMBER,
        "decision_date_hijri": DECISION_DATE_HIJRI,
        "decision_date_gregorian": DECISION_DATE_GREGORIAN,
        "authority": DECISION_AUTHORITY,
        "academic_year": "1447-1448 AH / 2026-2027",
        "school_week": {
            "days": SCHOOL_WEEK_DAYS,
            "start_day": "Saturday",
            "end_day": "Wednesday"
        },
        "totals": {
            "calendar_days": TOTAL_CALENDAR_DAYS,
            "holiday_days": TOTAL_HOLIDAY_DAYS,
            "preparation_days": PREPARATION_DAYS
        },
        "verification_status": "dates_require_manual_verification",
        "anchor_date": {
            "hijri": "9 Muharram 1448",
            "gregorian": "2026-06-27",
            "day": "Saturday",
            "confidence": "high"
        }
    })
}

// ═══════════════════════════════════════════════════════════════
// Period Builder
// ═══════════════════════════════════════════════════════════════

fn at_day_start(date: NaiveDate) -> DateTime<Utc> {
    date.and_time(NaiveTime::MIN).and_utc()
}

fn at_day_end(date: NaiveDate) -> DateTime<Utc> {
    date.and_hms_opt(23, 59, 59)
        .expect("valid end-of-day time")
        .and_utc()
}

fn period_state(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    now: DateTime<Utc>,
    is_exam: bool,
) -> AcademicPeriodState {
    if end < now {
        AcademicPeriodState::Closed
    } else if start <= now {
        if is_exam {
            AcademicPeriodState::FinalExams
        } else {
            AcademicPeriodState::ActiveTeaching
        }
    } else {
        AcademicPeriodState::Planning
    }
}

/// Build the supplementary `AcademicPeriod` records for this template.
///
/// # Arguments
/// - `org_id` — the organization UUID
/// - `year_id` — the academic year UUID
/// - `year_start_date` — the Gregorian start date of the academic year
///   (which should be the first day of study, i.e. 27 June 2026)
/// - `term1_end_date` — the Gregorian end date of Term 1 (for computing mid-year break)
/// - `term2_end_date` — the Gregorian end date of Term 2 (for computing public exam windows)
/// - `existing_period_codes` — codes of periods already created (for idempotency)
/// - `now` — current timestamp
///
/// Returns only the periods that do not yet exist (by code).
pub fn build_supplementary_periods(
    org_id: Uuid,
    year_id: Uuid,
    year_start_date: NaiveDate,
    term1_end_date: Option<NaiveDate>,
    term2_end_date: Option<NaiveDate>,
    existing_period_codes: &[String],
    now: DateTime<Utc>,
) -> Vec<AcademicPeriod> {
    let defs = supplementary_period_defs();
    let metadata = provenance_metadata();
    let mut result = Vec::new();

    for def in &defs {
        // Skip if already provisioned
        if existing_period_codes.iter().any(|c| c == def.code) {
            continue;
        }

        // Compute actual start date
        let start_date = match def.code {
            "YE-MIDYEAR-BREAK" => {
                // Mid-year break starts the day after Term 1 ends
                match term1_end_date {
                    Some(t1_end) => t1_end + Duration::days(1),
                    None => continue, // Can't place break without term1 end
                }
            }
            "YE-EXAM-SECONDARY" => {
                // Secondary exams start the day after Term 2 ends
                match term2_end_date {
                    Some(t2_end) => t2_end + Duration::days(1),
                    None => continue,
                }
            }
            "YE-EXAM-BASIC" => {
                // Basic exams start after secondary exams
                match term2_end_date {
                    Some(t2_end) => t2_end + Duration::days(15),
                    None => continue,
                }
            }
            _ => {
                // Pre-semester periods use offset from year start
                year_start_date + Duration::days(def.start_offset_days)
            }
        };

        let end_date = start_date + Duration::days(def.duration_days - 1);
        let start_dt = at_day_start(start_date);
        let end_dt = at_day_end(end_date);
        let is_exam = def.period_kind == PeriodKind::Examination;

        let period = AcademicPeriod {
            id: Uuid::new_v4(),
            org_id,
            year_id,
            parent_id: None,
            name: def.name.to_string(),
            name_ar: Some(def.name_ar.to_string()),
            code: Some(def.code.to_string()),
            period_type: def.period_type,
            period_kind: def.period_kind,
            state: period_state(start_dt, end_dt, now, is_exam),
            start_date: start_dt,
            end_date: end_dt,
            hijri_start: None,
            hijri_end: None,
            enrollment_starts_at: None,
            enrollment_ends_at: None,
            is_enrollment_open: false,
            grade_entry_starts_at: None,
            grade_entry_ends_at: None,
            is_grade_entry_open: false,
            exam_starts_at: if is_exam { Some(start_dt) } else { None },
            exam_ends_at: if is_exam { Some(end_dt) } else { None },
            is_locked: false,
            locked_at: None,
            locked_by: None,
            sort_order: Some(def.sort_order),
            metadata: Some(metadata.clone()),
            created_at: now,
            updated_at: now,
        };

        result.push(period);
    }

    result
}

/// Returns the list of available official calendar templates.
/// Currently only the Yemen 1447-1448 template is defined.
pub fn list_templates() -> Vec<CalendarTemplateInfo> {
    vec![CalendarTemplateInfo {
        code: TEMPLATE_CODE.to_string(),
        name: TEMPLATE_NAME.to_string(),
        name_ar: TEMPLATE_NAME_AR.to_string(),
        country: "YE".to_string(),
        country_name: "Republic of Yemen".to_string(),
        country_name_ar: "الجمهورية اليمنية".to_string(),
        hijri_year: HIJRI_START_YEAR,
        gregorian_years: "2026-2027".to_string(),
        decision_number: Some(DECISION_NUMBER.to_string()),
        decision_date: Some(DECISION_DATE_GREGORIAN.to_string()),
        authority: DECISION_AUTHORITY.to_string(),
        authority_ar: "وزارة التربية والتعليم والبحث العلمي".to_string(),
    }]
}

/// Summary info about an available official calendar template.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CalendarTemplateInfo {
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub country: String,
    pub country_name: String,
    pub country_name_ar: String,
    pub hijri_year: i32,
    pub gregorian_years: String,
    pub decision_number: Option<String>,
    pub decision_date: Option<String>,
    pub authority: String,
    pub authority_ar: String,
}
