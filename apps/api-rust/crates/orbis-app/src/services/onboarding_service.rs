//! Onboarding Service — Orchestrates post-bootstrap institution configuration.
//!
//! This is an application-level orchestration service that evaluates the real
//! backend database state against the active institution blueprint to determine:
//! - Complete DAG of setup tasks
//! - Completed, in-progress, blocked, and optional tasks
//! - Task dependencies and blocking reasons
//! - Overall and category progress percentages
//! - Recommended next administrative action
//! - Optional idempotent default scaffolding provisioning
//!
//! Invariants:
//! - Idempotent, resumable, crash-safe
//! - 100% derived from real database state (never fakes completion)
//! - Never directly bypasses domain rules
//! - Blueprint-aware (School vs University vs Institute)

use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use orbis_domain::entities::academic_period::{
    AcademicPeriod, AcademicPeriodState, PeriodKind, PeriodType,
};
use orbis_domain::entities::academic_calendar::{AcademicCalendar, AcademicCalendarStatus};
use orbis_domain::entities::assessment_scheme::AssessmentScheme;
use orbis_domain::entities::section::Section;
use orbis_domain::entities::discount_template::{DiscountTemplate, DiscountType};
use orbis_domain::entities::fee_template::{FeeFrequency, FeeTemplate};
use orbis_domain::entities::org_unit::OrgUnit;
use orbis_domain::entities::organization::InstitutionType;
use orbis_domain::entities::subject::Subject;
use orbis_domain::entities::time_slot::TimeSlot;
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;
use orbis_domain::queries::filters::{
    SectionFilter, SectionSubjectFilter, PaginationParams, SubjectFilter,
};

use crate::context::RequestContext;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::reporting::{ReportDefinitionRepository, ReportTemplateRepository};
use crate::ports::repositories::{
    AcademicPeriodRepository, AcademicCalendarRepository, AccountRepository,
    AssessmentSchemeRepository, SectionRepository, SectionSubjectRepository,
    DiscountTemplateRepository, DocumentRequirementRepository, FeeTemplateRepository,
    IntegrationRepository, OrgUnitRepository, OrganizationRepository, RoomRepository,
    SettingsRepository, SubjectRepository, TimeSlotRepository,
};

// ═══════════════════════════════════════════════════════════════
// Models & Data Transfer Objects (DTOs)
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingTaskCategory {
    Organization,
    Calendar,
    Academics,
    Assessment,
    Finance,
    Students,
    Templates,
    Integrations,
}

impl OnboardingTaskCategory {
    pub fn label_en(&self) -> &'static str {
        match self {
            Self::Organization => "Organization & Identity",
            Self::Calendar => "Academic Calendar",
            Self::Academics => "Academic Structure",
            Self::Assessment => "Grading & Policy",
            Self::Finance => "Finance & Fees",
            Self::Students => "Student Readiness",
            Self::Templates => "Reports & Documents",
            Self::Integrations => "Integrations & Alerts",
        }
    }

    pub fn label_ar(&self) -> &'static str {
        match self {
            Self::Organization => "الهوية والمنشأة",
            Self::Calendar => "التقويم الدراسي",
            Self::Academics => "الهيكل الأكاديمي",
            Self::Assessment => "التقييم والسياسات",
            Self::Finance => "المالية والرسوم",
            Self::Students => "جاهزية الطلاب",
            Self::Templates => "النماذج والشهادات",
            Self::Integrations => "الربط والإشعارات",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingTaskStatus {
    NotStarted,
    InProgress,
    Completed,
    Blocked,
    Optional,
    NotApplicable,
}

fn resolve_task_status(
    is_completed: bool,
    dependencies_met: bool,
    progress: u8,
    is_required: bool,
) -> OnboardingTaskStatus {
    if is_completed && dependencies_met {
        OnboardingTaskStatus::Completed
    } else if !dependencies_met {
        OnboardingTaskStatus::Blocked
    } else if progress > 0 {
        OnboardingTaskStatus::InProgress
    } else if is_required {
        OnboardingTaskStatus::NotStarted
    } else {
        OnboardingTaskStatus::Optional
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupTask {
    pub id: String,
    pub category: OnboardingTaskCategory,
    pub title: String,
    pub title_ar: String,
    pub description: String,
    pub description_ar: String,
    pub is_required: bool,
    pub status: OnboardingTaskStatus,
    pub dependencies: Vec<String>,
    pub progress: u8,
    pub route: String,
    pub action_type: String,
    pub institution_types: Vec<String>,
    pub required_permissions: Vec<String>,
    pub completion_condition: String,
    pub blocking_reason: Option<String>,
    pub blocking_reason_ar: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryProgress {
    pub category: OnboardingTaskCategory,
    pub label: String,
    pub label_ar: String,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub progress_percentage: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingStateResponse {
    pub org_id: Uuid,
    pub org_name: String,
    pub institution_type: String,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub required_tasks: usize,
    pub required_completed: usize,
    pub optional_tasks: usize,
    pub optional_completed: usize,
    pub blocked_tasks: usize,
    pub progress_percentage: u8,
    pub required_progress_percentage: u8,
    pub is_institution_ready: bool,
    pub tasks: Vec<SetupTask>,
    pub categories: Vec<CategoryProgress>,
    pub recommended_next_task: Option<SetupTask>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionDefaultsRequest {
    pub calendar_id: Option<Uuid>,
    pub academic_calendar_name: Option<String>,
    pub academic_calendar_name_ar: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    /// Whether to create the optional legacy academic-year record.
    pub provision_academic_calendar: Option<bool>,
    /// Whether to add an optional starter period structure to a newly created year.
    pub provision_default_academic_periods: Option<bool>,
    /// Whether to create sample classes. Defaults to **false** (opt-in demo data).
    pub provision_stages_and_grades: Option<bool>,
    /// Whether to create sample subjects. Defaults to **false** (opt-in demo data).
    pub provision_subjects: Option<bool>,
    pub provision_assessment_scheme: Option<bool>,
    /// Whether to create a tuition fee template. Defaults to false.
    pub provision_tuition_fee_template: Option<bool>,
    /// Whether to create discount templates. Defaults to **false** (opt-in demo data).
    pub provision_discount_templates: Option<bool>,
    pub provision_hijri_school_calendar: Option<bool>,
    pub hijri_start_year: Option<i32>,
    pub hijri_start_month: Option<u8>,
    pub hijri_start_day: Option<u8>,
    pub hijri_month_lengths: Option<Vec<u8>>,
    pub provision_time_slots: Option<bool>,
    pub enable_period_autopilot: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionDefaultsResult {
    pub success: bool,
    pub created_items: Vec<String>,
    pub state: OnboardingStateResponse,
}

fn at_day_start(date: NaiveDate) -> DateTime<Utc> {
    date.and_time(NaiveTime::MIN).and_utc()
}

fn at_day_end(date: NaiveDate) -> DateTime<Utc> {
    date.and_hms_opt(23, 59, 59)
        .expect("valid end-of-day time")
        .and_utc()
}

fn hijri_month_position(start_year: i32, start_month: u8, offset: usize) -> (i32, usize) {
    let absolute_month = usize::from(start_month - 1) + offset;
    (
        start_year + (absolute_month / 12) as i32,
        absolute_month % 12,
    )
}

fn calendar_period_state(
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

// ═══════════════════════════════════════════════════════════════
// Onboarding Service
// ═══════════════════════════════════════════════════════════════

pub struct OnboardingService {
    pub org_repo: Arc<dyn OrganizationRepository>,
    pub org_unit_repo: Arc<dyn OrgUnitRepository>,
    pub year_repo: Arc<dyn AcademicCalendarRepository>,
    pub period_repo: Arc<dyn AcademicPeriodRepository>,
    pub subject_repo: Arc<dyn SubjectRepository>,
    pub section_repo: Arc<dyn SectionRepository>,
    pub section_subject_repo: Arc<dyn SectionSubjectRepository>,
    pub assessment_scheme_repo: Arc<dyn AssessmentSchemeRepository>,
    pub settings_repo: Arc<dyn SettingsRepository>,
    pub account_repo: Arc<dyn AccountRepository>,
    pub fee_template_repo: Arc<dyn FeeTemplateRepository>,
    pub discount_template_repo: Arc<dyn DiscountTemplateRepository>,
    pub room_repo: Arc<dyn RoomRepository>,
    pub time_slot_repo: Arc<dyn TimeSlotRepository>,
    pub doc_req_repo: Arc<dyn DocumentRequirementRepository>,
    pub definition_repo: Arc<dyn ReportDefinitionRepository>,
    pub template_repo: Arc<dyn ReportTemplateRepository>,
    pub integration_repo: Arc<dyn IntegrationRepository>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl OnboardingService {
    pub fn new(
        org_repo: Arc<dyn OrganizationRepository>,
        org_unit_repo: Arc<dyn OrgUnitRepository>,
        year_repo: Arc<dyn AcademicCalendarRepository>,
        period_repo: Arc<dyn AcademicPeriodRepository>,
        subject_repo: Arc<dyn SubjectRepository>,
        section_repo: Arc<dyn SectionRepository>,
        section_subject_repo: Arc<dyn SectionSubjectRepository>,
        assessment_scheme_repo: Arc<dyn AssessmentSchemeRepository>,
        settings_repo: Arc<dyn SettingsRepository>,
        account_repo: Arc<dyn AccountRepository>,
        fee_template_repo: Arc<dyn FeeTemplateRepository>,
        discount_template_repo: Arc<dyn DiscountTemplateRepository>,
        room_repo: Arc<dyn RoomRepository>,
        time_slot_repo: Arc<dyn TimeSlotRepository>,
        doc_req_repo: Arc<dyn DocumentRequirementRepository>,
        definition_repo: Arc<dyn ReportDefinitionRepository>,
        template_repo: Arc<dyn ReportTemplateRepository>,
        integration_repo: Arc<dyn IntegrationRepository>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            org_repo,
            org_unit_repo,
            year_repo,
            period_repo,
            subject_repo,
            section_repo,
            section_subject_repo,
            assessment_scheme_repo,
            settings_repo,
            account_repo,
            fee_template_repo,
            discount_template_repo,
            room_repo,
            time_slot_repo,
            doc_req_repo,
            definition_repo,
            template_repo,
            integration_repo,
            permission_checker,
        }
    }

    async fn provision_hijri_school_calendar(
        &self,
        org_id: Uuid,
        year_id: Uuid,
        hijri_start_year: i32,
        hijri_start_month: u8,
        hijri_start_day: u8,
        month_lengths: Option<Vec<u8>>,
        now: DateTime<Utc>,
    ) -> Result<Vec<AcademicPeriod>, DomainError> {
        let existing_periods = self.period_repo.find_by_year(year_id).await?;
        if !(1300..=1600).contains(&hijri_start_year) {
            return Err(DomainError::Validation(
                "The Hijri start year is outside the supported range.".into(),
            ));
        }
        if !(1..=12).contains(&hijri_start_month) || !(1..=30).contains(&hijri_start_day) {
            return Err(DomainError::Validation(
                "The Hijri start month or day is invalid.".into(),
            ));
        }
        let lengths = month_lengths.unwrap_or_else(|| {
            vec![30, 29, 30, 29, 30, 29, 30, 29]
        });
        if lengths.len() != 8
            || lengths
                .iter()
                .enumerate()
                .any(|(index, days)| {
                    !(if index == 0 { 1..=30 } else { 29..=30 }).contains(days)
                })
        {
            return Err(DomainError::Validation(
                "The Hijri calendar requires eight valid month lengths.".into(),
            ));
        }
        let mut expected_codes = vec!["T1".to_string(), "T2".to_string()];
        for index in 0..8 {
            let (month_year, month_index) =
                hijri_month_position(hijri_start_year, hijri_start_month, index);
            let month_number = month_index + 1;
            expected_codes.push(format!("H{}-M{:02}", month_year, month_number));
            expected_codes.push(if index % 4 == 3 {
                format!("T{}-FINAL", index / 4 + 1)
            } else {
                format!("H{}-M{:02}-EXAM", month_year, month_number)
            });
        }
        if existing_periods.iter().any(|period| {
            period.hijri_start.is_none()
                || !period
                    .code
                    .as_ref()
                    .is_some_and(|code| expected_codes.contains(code))
        }) {
            return Err(DomainError::Validation(
                "The selected academic year already contains non-Hijri or unrelated periods. Complete or archive them before provisioning the Hijri calendar.".into(),
            ));
        }

        let mut year =
            self.year_repo.find_by_id(year_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Academic year {} not found", year_id))
            })?;

        let month_names_en = [
            "Muharram",
            "Safar",
            "Rabi' al-Awwal",
            "Rabi' al-Thani",
            "Jumada al-Ula",
            "Jumada al-Thani",
            "Rajab",
            "Sha'ban",
            "Ramadan",
            "Shawwal",
            "Dhu al-Qi'dah",
            "Dhu al-Hijjah",
        ];
        let month_names_ar = [
            "محرم",
            "صفر",
            "ربيع الأول",
            "ربيع الآخر",
            "جمادى الأولى",
            "جمادى الآخرة",
            "رجب",
            "شعبان",
            "رمضان",
            "شوال",
            "ذو القعدة",
            "ذو الحجة",
        ];
        let start_date = year.start_date.date_naive();
        let calendar_end_date = start_date
            + Duration::days(lengths.iter().map(|days| i64::from(*days)).sum::<i64>() - 1);
        if year.end_date.date_naive() < calendar_end_date {
            year.end_date = at_day_end(calendar_end_date);
            self.year_repo.save(&year).await?;
        }
        let mut periods = existing_periods;
        let mut next_month_start = start_date;
        let mut month_ranges = Vec::with_capacity(8);

        for (index, days) in lengths.iter().enumerate() {
            let month_start = next_month_start;
            let month_end = month_start + Duration::days(i64::from(*days) - 1);
            let exam_start_offset = i64::from((*days).min(3) - 1);
            let exam_start_date = month_end - Duration::days(exam_start_offset);
            let (month_year, month_index) =
                hijri_month_position(hijri_start_year, hijri_start_month, index);
            let month_number = month_index + 1;
            let hijri_day_start = if index == 0 { hijri_start_day } else { 1 };
            let hijri_day_end = hijri_day_start + days - 1;
            month_ranges.push((month_start, month_end));

            let term_index = index / 4;
            let term_name = if term_index == 0 {
                ("First Term", "الفصل الدراسي الأول", "T1")
            } else {
                ("Second Term", "الفصل الدراسي الثاني", "T2")
            };
            let term_id = if let Some(term) = periods
                .iter()
                .find(|period| {
                    period.parent_id.is_none() && period.code.as_deref() == Some(term_name.2)
                })
                .cloned()
            {
                term.id
            } else if index % 4 == 0 {
                let term_start = month_start;
                let term_end = (index..index + 4)
                    .map(|month_index| i64::from(lengths[month_index]))
                    .sum::<i64>();
                let term_last_date = term_start + Duration::days(term_end - 1);
                let term_end_index = index + 3;
                let (term_end_year, term_end_month_index) =
                    hijri_month_position(hijri_start_year, hijri_start_month, term_end_index);
                let id = Uuid::new_v4();
                let term = AcademicPeriod {
                    id,
                    org_id,
                    year_id,
                    parent_id: None,
                    name: term_name.0.to_string(),
                    name_ar: Some(term_name.1.to_string()),
                    code: Some(term_name.2.to_string()),
                    period_type: PeriodType::Semester,
                    period_kind: PeriodKind::Teaching,
                    state: calendar_period_state(
                        at_day_start(term_start),
                        at_day_end(term_last_date),
                        now,
                        false,
                    ),
                    start_date: at_day_start(term_start),
                    end_date: at_day_end(term_last_date),
                    hijri_start: Some(format!(
                        "{} {} {}",
                        hijri_day_start, month_names_en[month_index], month_year
                    )),
                    hijri_end: Some(format!(
                        "{} {} {}",
                        lengths[term_end_index],
                        month_names_en[term_end_month_index],
                        term_end_year
                    )),
                    enrollment_starts_at: None,
                    enrollment_ends_at: None,
                    is_enrollment_open: false,
                    grade_entry_starts_at: None,
                    grade_entry_ends_at: None,
                    is_grade_entry_open: false,
                    exam_starts_at: None,
                    exam_ends_at: None,
                    is_locked: false,
                    locked_at: None,
                    locked_by: None,
                    sort_order: Some((term_index + 1) as i32),
                    metadata: None,
                    created_at: now,
                    updated_at: now,
                };
                self.period_repo.save(&term).await?;
                periods.push(term);
                id
            } else {
                periods
                    .iter()
                    .find(|period| {
                        period.parent_id.is_none() && period.code.as_deref() == Some(term_name.2)
                    })
                    .map(|period| period.id)
                    .ok_or_else(|| {
                        DomainError::Internal("Academic term was not provisioned".into())
                    })?
            };

            let exam_start = at_day_start(exam_start_date);
            if index > 0 || *days > 3 {
                let teaching_start = at_day_start(month_start);
                let teaching_end = exam_start - Duration::seconds(1);
                let teaching = AcademicPeriod {
                    id: Uuid::new_v4(),
                    org_id,
                    year_id,
                    parent_id: Some(term_id),
                    name: format!("{} {}", month_names_en[month_index], month_year),
                    name_ar: Some(format!("{} {}", month_names_ar[month_index], month_year)),
                    code: Some(format!("H{}-M{:02}", month_year, month_number)),
                    period_type: PeriodType::Month,
                    period_kind: PeriodKind::Teaching,
                    state: calendar_period_state(teaching_start, teaching_end, now, false),
                    start_date: teaching_start,
                    end_date: teaching_end,
                    hijri_start: Some(format!(
                        "{} {} {}",
                        hijri_day_start, month_names_en[month_index], month_year
                    )),
                    hijri_end: Some(format!(
                        "{} {} {}",
                        hijri_day_end - 3,
                        month_names_en[month_index],
                        month_year
                    )),
                    enrollment_starts_at: None,
                    enrollment_ends_at: None,
                    is_enrollment_open: false,
                    grade_entry_starts_at: None,
                    grade_entry_ends_at: None,
                    is_grade_entry_open: calendar_period_state(
                        teaching_start,
                        teaching_end,
                        now,
                        false,
                    ) == AcademicPeriodState::ActiveTeaching,
                    exam_starts_at: None,
                    exam_ends_at: None,
                    is_locked: false,
                    locked_at: None,
                    locked_by: None,
                    sort_order: Some(month_number as i32),
                    metadata: None,
                    created_at: now,
                    updated_at: now,
                };
                if !periods.iter().any(|period| period.code == teaching.code) {
                    self.period_repo.save(&teaching).await?;
                    periods.push(teaching);
                }
            }

            let exam_is_final = index % 4 == 3;
            let exam_end = at_day_end(month_end);
            let exam = AcademicPeriod {
                id: Uuid::new_v4(),
                org_id,
                year_id,
                parent_id: Some(term_id),
                name: if exam_is_final {
                    format!("{} Final Examination", term_name.0)
                } else {
                    format!("{} Monthly Examination", month_names_en[index])
                },
                name_ar: Some(if exam_is_final {
                    format!("اختبار نهاية {}", term_name.1)
                } else {
                    format!("اختبار {}", month_names_ar[index])
                }),
                code: Some(if exam_is_final {
                    format!("{}-FINAL", term_name.2)
                } else {
                    format!("H{}-M{:02}-EXAM", month_year, month_number)
                }),
                period_type: PeriodType::Month,
                period_kind: PeriodKind::Examination,
                state: calendar_period_state(exam_start, exam_end, now, true),
                start_date: exam_start,
                end_date: exam_end,
                hijri_start: Some(format!(
                    "{} {} {}",
                    hijri_day_end - 2,
                    month_names_en[month_index],
                    month_year
                )),
                hijri_end: Some(format!(
                    "{} {} {}",
                    hijri_day_end, month_names_en[month_index], month_year
                )),
                enrollment_starts_at: None,
                enrollment_ends_at: None,
                is_enrollment_open: false,
                grade_entry_starts_at: None,
                grade_entry_ends_at: None,
                is_grade_entry_open: false,
                exam_starts_at: Some(exam_start),
                exam_ends_at: Some(exam_end),
                is_locked: false,
                locked_at: None,
                locked_by: None,
                sort_order: Some(100 + month_number as i32),
                metadata: None,
                created_at: now,
                updated_at: now,
            };
            if !periods.iter().any(|period| period.code == exam.code) {
                self.period_repo.save(&exam).await?;
                periods.push(exam);
            }

            next_month_start = month_end + Duration::days(1);
        }

        let calendar_end_date = month_ranges[7].1;
        if year.end_date.date_naive() == calendar_end_date {
            year.end_date = at_day_end(calendar_end_date);
        }
        let (end_year, end_month_index) =
            hijri_month_position(hijri_start_year, hijri_start_month, 7);
        year.hijri_start = Some(format!(
            "{} {} {}",
            hijri_start_day, month_names_en[usize::from(hijri_start_month - 1)], hijri_start_year
        ));
        year.hijri_end = Some(format!(
            "{} {} {}",
            lengths[7],
            month_names_en[end_month_index],
            end_year
        ));
        year.updated_at = now;
        self.year_repo.save(&year).await?;

        Ok(periods)
    }

    async fn provision_default_school_time_slots(
        &self,
        periods: &[AcademicPeriod],
        is_yemen: bool,
        now: DateTime<Utc>,
    ) -> Result<usize, DomainError> {
        let slot_definitions = [
            ("Lesson 1", "الحصة الأولى", (7, 30), (8, 15), false, None),
            ("Lesson 2", "الحصة الثانية", (8, 15), (9, 0), false, None),
            (
                "Morning Break",
                "الفسحة",
                (9, 0),
                (9, 15),
                true,
                Some("short"),
            ),
            ("Lesson 3", "الحصة الثالثة", (9, 15), (10, 0), false, None),
            ("Lesson 4", "الحصة الرابعة", (10, 0), (10, 45), false, None),
            (
                "Lunch Break",
                "استراحة الغداء",
                (10, 45),
                (11, 15),
                true,
                Some("lunch"),
            ),
            ("Lesson 5", "الحصة الخامسة", (11, 15), (12, 0), false, None),
            ("Lesson 6", "الحصة السادسة", (12, 0), (12, 45), false, None),
            ("Lesson 7", "الحصة السابعة", (12, 45), (13, 30), false, None),
        ];
        let mut created = 0;
        let study_days: &[i16] = if is_yemen {
            &[7, 1, 2, 3, 4]
        } else {
            &[1, 2, 3, 4, 5]
        };

        for period in periods.iter().filter(|period| {
            period.parent_id.is_some()
                && period.period_type == PeriodType::Month
                && period.period_kind == PeriodKind::Teaching
        }) {
            let existing_slots = self.time_slot_repo.find_by_period(period.id).await?;
            let mut slots = Vec::with_capacity(45);
            for &day_of_week in study_days {
                for (index, (name, name_ar, start, end, is_break, break_type)) in
                    slot_definitions.iter().enumerate()
                {
                    let slot_index = index as i32 + 1;
                    if existing_slots.iter().any(|slot| {
                        slot.day_of_week == day_of_week && slot.slot_index == slot_index
                    }) {
                        continue;
                    }
                    let start_time =
                        NaiveTime::from_hms_opt(start.0, start.1, 0).ok_or_else(|| {
                            DomainError::Internal("Invalid default timetable start time".into())
                        })?;
                    let end_time = NaiveTime::from_hms_opt(end.0, end.1, 0).ok_or_else(|| {
                        DomainError::Internal("Invalid default timetable end time".into())
                    })?;
                    slots.push(TimeSlot {
                        id: Uuid::new_v4(),
                        period_id: period.id,
                        day_of_week,
                        slot_index,
                        name: Some((*name).to_string()),
                        name_ar: Some((*name_ar).to_string()),
                        start_time,
                        end_time,
                        is_break: *is_break,
                        break_type: break_type.map(str::to_string),
                        created_at: now,
                        updated_at: now,
                    });
                }
            }
            if !slots.is_empty() {
                created += slots.len();
                self.time_slot_repo.save_bulk(&slots).await?;
            }
        }

        Ok(created)
    }

    // ─────────────────────────────────────────────────────────
    // Get Complete Onboarding State
    // ─────────────────────────────────────────────────────────

    /// Dynamically evaluates real database state to produce the full DAG onboarding state.
    pub async fn get_state(&self, org_id: Uuid) -> Result<OnboardingStateResponse, DomainError> {
        let org =
            self.org_repo.find_by_id(org_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Organization {} not found", org_id))
            })?;

        let institution_type_str = org
            .metadata
            .as_ref()
            .and_then(|m| m.get("institution_type"))
            .and_then(|v| v.as_str())
            .unwrap_or("school");
        let _inst_type = InstitutionType::from_str_loose(institution_type_str);

        // ── 1. Gather Real Backend Metrics ─────────────────────
        let units = self.org_unit_repo.find_by_org(org_id).await?;
        let rooms = self.room_repo.find_by_org(org_id).await?;
        let years = self.year_repo.find_by_org(org_id).await?;
        let active_year = years.iter().find(|y| y.status == AcademicCalendarStatus::Active || !y.is_locked);
        let periods = if let Some(ay) = active_year {
            self.period_repo.find_by_year(ay.id).await?
        } else {
            self.period_repo.find_by_org(org_id).await?
        };

        let subjects_res = self
            .subject_repo
            .list(org_id, SubjectFilter::default())
            .await?;
        let classes_res = self.section_repo.list(org_id, SectionFilter::default()).await?;
        let section_subjects_res = self
            .section_subject_repo
            .list(org_id, SectionSubjectFilter::default())
            .await?;
        let settings = self
            .settings_repo
            .get_settings(org_id)
            .await?;
        let accounts = self.account_repo.find_by_org(org_id).await?;
        let fee_templates = self.fee_template_repo.find_by_org(org_id).await?;
        let discount_templates = self.discount_template_repo.find_by_org(org_id).await?;
        let doc_reqs = self.doc_req_repo.find_by_org(org_id).await?;
        let templates = self
            .template_repo
            .list_by_org(org_id, None, None, None)
            .await?;
        let integrations = self.integration_repo.find_configs_by_org(org_id).await?;

        // ── 2. Raw Task Completion & Progress Checks ───────────
        let mut raw_tasks = Vec::new();

        // Task 1: Organization Profile
        let has_org_profile = !org.name.trim().is_empty()
            && !org.timezone.trim().is_empty()
            && !org.locale.trim().is_empty()
            && (org.name_ar.is_some() || org.primary_color.is_some() || org.code.is_some());
        raw_tasks.push((
            "org_profile",
            OnboardingTaskCategory::Organization,
            "Organization Profile & Branding",
            "الملف التعريفي والهوية المؤسسية",
            "Configure legal names, logo, timezone, locale, and primary contact information.",
            "إعداد بيانات المنشأة الرسمية، الاسم بالعربية والإنجليزية، الشعار، والمنطقة الزمنية.",
            true,
            vec![],
            if has_org_profile { 100 } else { 50 },
            has_org_profile,
            "/settings",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["organization:write".into()],
            "Organization name, timezone, locale, and branding verified.",
            None,
        ));

        // Task 2: Campuses & Branches (Org Units)
        let has_org_units = !units.is_empty();
        raw_tasks.push((
            "org_units",
            OnboardingTaskCategory::Organization,
            "Campuses & Branch Structure",
            "الفروع والمقرات الإدارية",
            "Create main campus and institutional administrative units or branches.",
            "تعريف المقر الرئيسي والفروع التعليمية والإدارية التابعة للمؤسسة.",
            true,
            vec!["org_profile".into()],
            if has_org_units { 100 } else { 0 },
            has_org_units,
            "/settings/branches",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["org_unit:write".into()],
            "At least one active branch or organizational unit created.",
            Some((
                "Requires organization profile completion.",
                "يتطلب إكمال الملف التعريفي أولاً.",
            )),
        ));

        // Task 3: Facilities & Rooms (Optional)
        let has_rooms = !rooms.is_empty();
        raw_tasks.push((
            "facilities_rooms",
            OnboardingTaskCategory::Organization,
            "Rooms & Educational Facilities",
            "القاعات والمرافق التعليمية",
            "Configure classrooms, laboratories, auditoriums, and capacity bounds.",
            "إضافة الفصول، المعامل، القاعات الدراسية، والطاقة الاستيعابية لكل مرفق.",
            false,
            vec!["org_units".into()],
            if has_rooms { 100 } else { 0 },
            has_rooms,
            "/settings/rooms",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["org_unit:write".into()],
            "Educational rooms or facilities defined.",
            Some((
                "Requires at least one branch or campus.",
                "يتطلب وجود فرع أو مقر واحد على الأقل.",
            )),
        ));

        // Task 4: Academic Year
        let has_academic_calendar = active_year.is_some() || !years.is_empty();
        raw_tasks.push((
            "academic_calendar",
            OnboardingTaskCategory::Calendar,
            "Academic Year & Dates",
            "العام الدراسي والتواريخ",
            "Define the active academic year, opening and closing dates, and official calendar status.",
            "تحديد العام الدراسي الفعال، تواريخ البداية والنهاية، وحالة التقويم الرسمي.",
            false,
            vec!["org_units".into()],
            if has_academic_calendar { 100 } else { 0 },
            has_academic_calendar,
            "/settings/periods",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["period:write".into()],
            "An active academic year exists with valid date range.",
            Some(("Requires branch setup before opening academic year.", "يتطلب إعداد الفروع أولاً.")),
        ));

        // Academic calendars and their periods are optional; organizations may run
        // rolling, cohort-based, or otherwise non-calendar academic models.
        let has_periods = !periods.is_empty()
            && periods
                .iter()
                .any(|p| p.period_kind == PeriodKind::Teaching || p.period_type == PeriodType::Semester);
        let period_progress = if has_periods {
            100
        } else if !periods.is_empty() {
            50
        } else {
            0
        };
        let period_desc_en =
            "Configure one or more academic cycle periods and evaluation windows aligned with your institution calendar.";
        let period_desc_ar =
            "تهيئة فترات دورة أكاديمية واحدة أو أكثر وفترات تقييم مناسبة للتقويم المؤسسي الخاص بك.";
        raw_tasks.push((
            "academic_periods",
            OnboardingTaskCategory::Calendar,
            "Academic Cycles & Evaluation Windows",
            "الدورات الأكاديمية ونوافذ التقييم",
            period_desc_en,
            period_desc_ar,
            false,
            vec![],
            period_progress,
            has_periods,
            "/settings/periods",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["period:write".into()],
            "Configured terms and periods under the active academic year.",
            Some((
                "Requires an active Academic Year.",
                "يتطلب إنشاء وتفعيل العام الدراسي أولاً.",
            )),
        ));

        // Task 6: Academic Subjects / Courses
        let has_subjects = subjects_res.total_count > 0;
        raw_tasks.push((
            "academic_subjects",
            OnboardingTaskCategory::Academics,
            "Curriculum Subjects & Courses",
            "المواد والمساقات الدراسية",
            "Add curriculum subjects with code, credit/weekly hours, and classification.",
            "إضافة المواد الدراسية مع الرمز، عدد الحصص/الساعات الأسبوعية، ونوع المادة.",
            true,
            vec!["org_units".into()],
            if has_subjects { 100 } else { 0 },
            has_subjects,
            "/settings/subjects",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["subject:write".into()],
            "At least one subject or course defined.",
            Some((
                "Requires branch configuration.",
                "يتطلب وجود فرع تعليمي واحد على الأقل.",
            )),
        ));

        // Task 7: Classes / Sections
        let has_classes = classes_res.total_count > 0;
        raw_tasks.push((
            "academic_classes",
            OnboardingTaskCategory::Academics,
            "Classes, Sections & Cohorts",
            "الفصول والشعب الدراسية",
            "Configure classroom sections, homeroom divisions, grade level associations, and capacity limits.",
            "إنشاء الشعب الدراسية والفصول، وتحديد المرحلة والصف والطاقة الاستيعابية.",
            true,
            vec!["academic_subjects".into()],
            if has_classes { 100 } else { 0 },
            has_classes,
            "/academics/classes",
            "navigate",
            vec!["school".into()],
            vec!["Section:write".into()],
            "At least one Section or section created for the active period.",
            Some(("Requires academic periods and subjects to be created first.", "يتطلب إعداد الفصول الدراسية والمواد أولاً.")),
        ));

        // Task 8: Teacher Subject Assignments (Optional prerequisite for timetable)
        let has_teaching = section_subjects_res.total_count > 0;
        raw_tasks.push((
            "teaching_assignments",
            OnboardingTaskCategory::Academics,
            "Teaching Assignments",
            "إسناد المواد للمعلمين",
            "Link teachers to classes and subjects for gradebook and scheduling.",
            "تعيين المعلمين للمواد والشعب الدراسية لتمكين رصد الدرجات والجدول المدرسي.",
            false,
            vec!["academic_classes".into(), "academic_subjects".into()],
            if has_teaching { 100 } else { 0 },
            has_teaching,
            "/academics/classes",
            "navigate",
            vec!["school".into()],
            vec!["Section:write".into()],
            "Section-subject teacher assignments configured.",
            Some((
                "Requires classes and subjects to be configured.",
                "يتطلب إنشاء الفصول والمواد أولاً.",
            )),
        ));

        // Task 9: Weekly Schedule
        let teaching_periods: Vec<_> = periods
            .iter()
            .filter(|period| period.period_kind == PeriodKind::Teaching)
            .collect();
        let mut configured_periods = 0;
        for period in &teaching_periods {
            let slots = self.time_slot_repo.find_by_period(period.id).await?;
            if !slots.is_empty() {
                configured_periods += 1;
            }
        }
        let has_timetable = configured_periods > 0;
        let timetable_progress = if teaching_periods.is_empty() {
            0
        } else {
            ((configured_periods * 100) / teaching_periods.len()) as u8
        };
        raw_tasks.push((
            "scheduling_setup",
            OnboardingTaskCategory::Academics,
            "Weekly Schedule & Time Slots",
            "الجدول الأسبوعي والحصص",
            "Set up a weekly timetable with lesson times, breaks, and applicable school days.",
            "تهيئة جدول أسبوعي يتضمن أوقات الحصص والاستراحات وأيام الدراسة.",
            false,
            vec!["academic_periods".into()],
            timetable_progress,
            has_timetable,
            "/onboarding/scheduling",
            "navigate",
            vec!["school".into()],
            vec!["schedule:write".into()],
            "At least one teaching period has a configured timetable.",
            Some((
                "Requires an academic calendar with teaching periods.",
                "يتطلب إنشاء تقويم دراسي وفترات تعليمية أولاً.",
            )),
        ));

        // The calculation plan is the single calculation authority. Legacy assessment
        // schemes may still be readable, but they do not configure the academic engine.
        let has_academic_policy = settings.academic.calculation_plan.is_some()
            && settings.academic.outcome_policy.is_some();
        raw_tasks.push((
            "calculation_policy",
            OnboardingTaskCategory::Assessment,
            "Calculation Engine & Outcomes",
            "محرك الحساب وسياسات النتائج",
            "Review the typed calculation plan, outcome thresholds, rounding, and ranking policy for this institution.",
            "مراجعة خطة الحساب المحددة الأنواع، وحدود النتائج، والتقريب، وسياسة الترتيب الخاصة بالمؤسسة.",
            true,
            vec![],
            if has_academic_policy { 100 } else { 0 },
            has_academic_policy,
            "/settings/academic-policy",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["organization:write".into()],
            "A calculation plan and outcome policy are configured.",
            None,
        ));

        // Task 11: Chart of Accounts & Ledger
        let has_coa = accounts.iter().any(|a| {
            a.code == "110101"
                || a.code == "410101"
                || a.code == "1"
                || a.code == "1101"
                || a.code == "110301"
                || a.code == "1000"
        });
        raw_tasks.push((
            "chart_of_accounts",
            OnboardingTaskCategory::Finance,
            "Chart of Accounts & Ledger",
            "دليل الحسابات ودفتر الأستاذ",
            "Review asset, liability, revenue, discount, and cashbox accounts for double-entry postings.",
            "مراجعة حسابات الأصول، الإيرادات، الخصومات، والصناديق لضمان صحة القيود المحاسبية.",
            true,
            vec!["org_units".into()],
            if has_coa { 100 } else { 0 },
            has_coa,
            "/finance/accounts",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["finance:write".into()],
            "Standard double-entry chart of accounts verified.",
            Some(("Requires branch setup.", "يتطلب إعداد الفروع أولاً.")),
        ));

        // Task 12: Fee Templates & Tuition Rates
        let has_fees = !fee_templates.is_empty();
        raw_tasks.push((
            "fee_templates",
            OnboardingTaskCategory::Finance,
            "Fee Templates & Tuition Rates",
            "قوالب الرسوم الدراسية والخدمات",
            "Configure tuition, transportation, and service fee templates with grade-level targeting and installment rules.",
            "إنشاء قوالب الرسوم (تعليم، نقل، خدمات) مع تحديد الصفوف المستهدفة وسياسات الأقساط.",
            false,
            vec!["chart_of_accounts".into()],
            if has_fees { 100 } else { 0 },
            has_fees,
            "/settings/fee-templates",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["finance:write".into()],
            "At least one active fee template configured with revenue account link.",
            Some(("Requires chart of accounts and academic year.", "يتطلب دليل الحسابات والعام الدراسي.")),
        ));

        // Task 13: Discount Templates & Scholarships (Optional)
        let has_discounts = !discount_templates.is_empty();
        raw_tasks.push((
            "discount_templates",
            OnboardingTaskCategory::Finance,
            "Discounts & Scholarships",
            "الخصومات والمنح الدراسية",
            "Configure sibling discounts, staff concessions, and academic scholarship rules.",
            "تهيئة خصومات الأخوة، أبناء العاملين، ومنح التفوق الأكاديمي.",
            false,
            vec!["fee_templates".into()],
            if has_discounts { 100 } else { 0 },
            has_discounts,
            "/settings/discount-templates",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["finance:write".into()],
            "Discount templates configured.",
            Some((
                "Requires fee templates to be created first.",
                "يتطلب إنشاء قوالب الرسوم أولاً.",
            )),
        ));

        // Task 14: Student Registration & Admission Policy
        let has_registration_policy =
            settings.enrollment.is_configured && settings.enrollment.max_class_size_default > 0;
        raw_tasks.push((
            "student_registration_policy",
            OnboardingTaskCategory::Students,
            "Admission & Enrollment Rules",
            "شروط القبول وسياسات التسجيل",
            "Set default Section capacity, waitlist rules, late enrollment permissions, and numbering sequence.",
            "تحديد الحد الأقصى للطلاب، سياسة قائمة الانتظار، وشروط القبول المتأخر وتوليد الأرقام الأكاديمية.",
            true,
            vec![],
            if has_registration_policy { 100 } else { 0 },
            has_registration_policy,
            "/settings/registration",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["organization:write".into()],
            "Student registration and enrollment rules configured.",
            None,
        ));

        // Task 15: Document Requirements (Optional)
        let has_doc_reqs = !doc_reqs.is_empty();
        raw_tasks.push((
            "document_requirements",
            OnboardingTaskCategory::Students,
            "Required Student Documents",
            "المستندات والوثائق المطلوبة",
            "Define mandatory student admission documents (birth certificate, national ID, vaccine card, transfer files).",
            "تحديد الوثائق الإلزامية لملف الطالب (شهادة الميلاد، الهوية، بطاقة التطعيم، ملف النقل).",
            false,
            vec![],
            if has_doc_reqs { 100 } else { 0 },
            has_doc_reqs,
            "/settings/archive",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["document:write".into()],
            "Document requirements or archive folders configured.",
            None,
        ));

        // Task 16: Certificate & Report Templates (Optional)
        let has_templates = !templates.is_empty();
        raw_tasks.push((
            "system_templates",
            OnboardingTaskCategory::Templates,
            "Certificates & Report Templates",
            "نماذج الشهادات والبطاقات الذكية",
            "Review and customize official student report card, ID card, and graduation certificate visual templates.",
            "مراجعة وتخصيص نماذج الشهادات الرسمية، بطاقات الهوية الذكية، وكشوف الدرجات المعتمدة.",
            false,
            vec!["org_profile".into()],
            if has_templates { 100 } else { 0 },
            has_templates,
            "/templates",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["template:write".into()],
            "Visual templates available and verified.",
            Some(("Requires organization profile.", "يتطلب الملف التعريفي للمنشأة.")),
        ));

        // Task 17: Integrations & Notifications (Optional)
        let has_integrations = !integrations.is_empty();
        raw_tasks.push((
            "integrations_setup",
            OnboardingTaskCategory::Integrations,
            "Integrations & Gateways",
            "بوابات الربط والإشعارات",
            "Configure Email SMTP, SMS, WhatsApp, Electronic Payment Gateway, and DocForge connections.",
            "ربط مزودي البريد الإلكتروني، الرسائل النصية، الواتساب، وبوابات الدفع الإلكتروني ومحرك المستندات.",
            false,
            vec!["org_profile".into()],
            if has_integrations { 100 } else { 0 },
            has_integrations,
            "/settings/notifications",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["organization:write".into()],
            "External communication channels or integrations configured.",
            Some(("Requires organization profile.", "يتطلب الملف التعريفي للمنشأة.")),
        ));

        // ── 3. Resolve Dependencies & Directed Statuses ────────
        let mut completed_task_ids = std::collections::HashSet::new();

        let mut tasks = Vec::new();
        let mut blocked_count = 0;
        let mut required_count = 0;
        let mut required_completed = 0;
        let mut optional_count = 0;
        let mut optional_completed = 0;
        let mut total_progress_sum: u32 = 0;
        let mut required_progress_sum: u32 = 0;

        for (
            id,
            category,
            title,
            title_ar,
            desc,
            desc_ar,
            is_required,
            deps,
            raw_progress,
            raw_is_completed,
            route,
            action_type,
            inst_types,
            perms,
            condition,
            blocking_hint,
        ) in raw_tasks
        {
            let id_str = id.to_string();
            let is_applicable = inst_types.iter().any(|kind| kind == institution_type_str);

            // A raw completion predicate is only effective after its prerequisites pass.
            let dependencies_met = deps
                .iter()
                .all(|dependency| completed_task_ids.contains(dependency));
            let dependencies_met = dependencies_met || !is_applicable;
            let is_completed = raw_is_completed && dependencies_met;
            let status = if is_applicable {
                resolve_task_status(
                    raw_is_completed,
                    dependencies_met,
                    raw_progress,
                    is_required,
                )
            } else {
                OnboardingTaskStatus::NotApplicable
            };
            if is_completed {
                completed_task_ids.insert(id_str.clone());
            }
            if !is_applicable {
                completed_task_ids.insert(id_str.clone());
            }

            let mut blocking_reason = None;
            let mut blocking_reason_ar = None;

            if is_applicable && !dependencies_met {
                for dep in &deps {
                    if !completed_task_ids.contains(dep) {
                        if let Some((hint_en, hint_ar)) = blocking_hint {
                            blocking_reason = Some(hint_en.to_string());
                            blocking_reason_ar = Some(hint_ar.to_string());
                        } else {
                            blocking_reason = Some(format!("Requires prerequisite task '{}'", dep));
                            blocking_reason_ar =
                                Some(format!("يتطلب إنجاز المهمة السابقة '{}'", dep));
                        }
                        break;
                    }
                }
            }
            if status == OnboardingTaskStatus::Blocked {
                blocked_count += 1;
            }

            let effective_progress = if !is_applicable || is_completed {
                100
            } else if dependencies_met {
                raw_progress
            } else {
                0
            };

            if is_applicable && is_required {
                required_count += 1;
                if is_completed {
                    required_completed += 1;
                }

                required_progress_sum += effective_progress as u32;
            } else if is_applicable {
                optional_count += 1;
                if is_completed {
                    optional_completed += 1;
                }
            }

            if is_applicable {
                total_progress_sum += effective_progress as u32;
            }

            tasks.push(SetupTask {
                id: id_str,
                category,
                title: title.to_string(),
                title_ar: title_ar.to_string(),
                description: desc.to_string(),
                description_ar: desc_ar.to_string(),
                is_required: is_applicable && is_required,
                status,
                dependencies: deps,
                progress: effective_progress,
                route: route.to_string(),
                action_type: action_type.to_string(),
                institution_types: inst_types,
                required_permissions: perms,
                completion_condition: condition.to_string(),
                blocking_reason,
                blocking_reason_ar,
                metadata: None,
            });
        }

        // ── 4. Calculate Category Progress Summaries ────────────
        let all_categories = [
            OnboardingTaskCategory::Organization,
            OnboardingTaskCategory::Calendar,
            OnboardingTaskCategory::Academics,
            OnboardingTaskCategory::Assessment,
            OnboardingTaskCategory::Finance,
            OnboardingTaskCategory::Students,
            OnboardingTaskCategory::Templates,
            OnboardingTaskCategory::Integrations,
        ];

        let mut categories = Vec::new();
        for cat in all_categories {
            let cat_tasks: Vec<_> = tasks
                .iter()
                .filter(|t| {
                    t.category == cat && t.status != OnboardingTaskStatus::NotApplicable
                })
                .collect();
            let total = cat_tasks.len();
            let completed = cat_tasks
                .iter()
                .filter(|t| t.status == OnboardingTaskStatus::Completed)
                .count();
            let prog = if total == 0 {
                100
            } else {
                let sum: u32 = cat_tasks.iter().map(|t| t.progress as u32).sum();
                (sum / total as u32).min(100) as u8
            };

            categories.push(CategoryProgress {
                category: cat,
                label: cat.label_en().to_string(),
                label_ar: cat.label_ar().to_string(),
                total_tasks: total,
                completed_tasks: completed,
                progress_percentage: prog,
            });
        }

        // ── 5. Global Aggregations & Next Recommended Action ───
        let total_tasks = tasks
            .iter()
            .filter(|task| task.status != OnboardingTaskStatus::NotApplicable)
            .count();
        let completed_tasks = required_completed + optional_completed;
        let progress_percentage = if total_tasks == 0 {
            100
        } else {
            (total_progress_sum / total_tasks as u32).min(100) as u8
        };
        let required_progress_percentage = if required_count == 0 {
            100
        } else {
            (required_progress_sum / required_count as u32).min(100) as u8
        };
        let is_institution_ready = required_completed >= required_count;

        // Find the first unblocked, uncompleted required task (or first optional)
        let recommended_next_task = tasks
            .iter()
            .find(|t| {
                t.is_required
                    && (t.status == OnboardingTaskStatus::NotStarted
                        || t.status == OnboardingTaskStatus::InProgress)
            })
            .cloned()
            .or_else(|| {
                tasks
                    .iter()
                    .find(|t| {
                        t.status == OnboardingTaskStatus::Optional
                            || t.status == OnboardingTaskStatus::InProgress
                    })
                    .cloned()
            });

        let mut warnings = Vec::new();
        if !is_institution_ready {
            warnings.push(format!(
                "Institution is in setup mode: {} of {} required tasks remaining.",
                required_count - required_completed,
                required_count
            ));
        }

        Ok(OnboardingStateResponse {
            org_id,
            org_name: org.name,
            institution_type: institution_type_str.to_string(),
            total_tasks,
            completed_tasks,
            required_tasks: required_count,
            required_completed,
            optional_tasks: optional_count,
            optional_completed,
            blocked_tasks: blocked_count,
            progress_percentage,
            required_progress_percentage,
            is_institution_ready,
            tasks,
            categories,
            recommended_next_task,
            warnings,
        })
    }

    // ─────────────────────────────────────────────────────────
    // Optional Helper: Provision Blueprint Defaults Scaffolding
    // ─────────────────────────────────────────────────────────

    pub fn generic_subject_templates() -> Vec<(&'static str, &'static str, Option<&'static str>, bool, i32)> {
        vec![
            ("SUBJ-01", "Core Learning Subject", Some("موضوع تعليمي أساسي"), true, 3),
            (
                "SUBJ-02",
                "Applied Learning Subject",
                Some("موضوع تعليمي تطبيقي"),
                false,
                2,
            ),
            (
                "SUBJ-03",
                "Specialized Learning Subject",
                Some("موضوع تعليمي متخصص"),
                true,
                4,
            ),
        ]
    }

    pub fn generic_class_templates() -> Vec<(&'static str, Option<&'static str>, &'static str, &'static str)> {
        vec![
            ("Learning Group 01", Some("مجموعة تعليمية 01"), "Study Track", "A"),
            ("Learning Group 02", Some("مجموعة تعليمية 02"), "Study Track", "B"),
            ("Learning Group 03", Some("مجموعة تعليمية 03"), "Study Track", "C"),
        ]
    }

    /// Resolve educational subjects adapted to country profile and institution type
    pub fn subjects_for_profile_and_type(
        _country_profile: &orbis_domain::entities::country_profile::CountryProfile,
        inst_type: &InstitutionType,
    ) -> Vec<(String, String, Option<String>, bool, i32)> {
        match inst_type {
            InstitutionType::School => vec![
                ("ISL-01".into(), "Islamic Studies".into(), Some("القرآن الكريم والتربية الإسلامية".into()), true, 4),
                ("ARB-01".into(), "Arabic Language".into(), Some("اللغة العربية".into()), true, 5),
                ("MATH-01".into(), "Mathematics".into(), Some("الرياضيات".into()), true, 5),
                ("SCI-01".into(), "Science".into(), Some("العلوم العامة".into()), true, 4),
                ("ENG-01".into(), "English Language".into(), Some("اللغة الإنجليزية".into()), true, 4),
                ("SOC-01".into(), "Social Studies".into(), Some("الدراسات الاجتماعية".into()), true, 3),
                ("COMP-01".into(), "Computer & IT".into(), Some("الحاسوب وتكنولوجيا المعلومات".into()), false, 2),
            ],
            InstitutionType::University => vec![
                ("CS101".into(), "Introduction to Computer Science".into(), Some("مقدمة في علوم الحاسوب".into()), true, 3),
                ("MATH101".into(), "Calculus I".into(), Some("تفاضل وتكامل 1".into()), true, 3),
                ("ENG101".into(), "Academic English".into(), Some("لغة إنجليزية أكاديمية".into()), true, 2),
                ("ISL101".into(), "Islamic Culture".into(), Some("الثقافة الإسلامية".into()), true, 2),
                ("PHYS101".into(), "General Physics".into(), Some("فيزياء عامة".into()), true, 3),
            ],
            InstitutionType::Institute => vec![
                ("VOC-SAF".into(), "Industrial Safety & Hazards".into(), Some("السلامة المهنية والصناعية".into()), true, 2),
                ("VOC-ELEC".into(), "Principles of Electrical Circuits".into(), Some("مبادئ الدوائر الكهربائية".into()), true, 3),
                ("VOC-PRAC".into(), "Applied Workshop Practice".into(), Some("التطبيقات والورش العملية".into()), true, 4),
                ("VOC-DRAW".into(), "Technical Drawing & CAD".into(), Some("الرسم الهندسي والتقني".into()), false, 2),
            ],
        }
    }

    /// Resolve educational sections/grades adapted to country profile and institution type
    pub fn sections_for_profile_and_type(
        country_profile: &orbis_domain::entities::country_profile::CountryProfile,
        inst_type: &InstitutionType,
    ) -> Vec<(String, Option<String>, String, String)> {
        match inst_type {
            InstitutionType::School => {
                let stages = country_profile.stages_for("school");
                let mut sections = Vec::new();
                for stage in stages {
                    for level in &stage.levels {
                        sections.push((
                            format!("{} - Section A", level.name),
                            Some(format!("{} - شعبة أ", level.name_ar)),
                            level.code.clone(),
                            "A".to_string(),
                        ));
                    }
                }
                if sections.is_empty() {
                    sections.push(("First Grade Basic - A".into(), Some("الصف الأول الأساسي - شعبة أ".into()), "G01".into(), "A".into()));
                }
                sections
            }
            InstitutionType::University => vec![
                ("CS101 - Section 01".into(), Some("مقدمة الحاسوب - شعبة 01".into()), "UG_Y1".into(), "01".into()),
                ("MATH101 - Section 01".into(), Some("تفاضل وتكامل 1 - شعبة 01".into()), "UG_Y1".into(), "01".into()),
                ("ENG101 - Section 01".into(), Some("لغة إنجليزية - شعبة 01".into()), "UG_Y1".into(), "01".into()),
            ],
            InstitutionType::Institute => vec![
                ("Cohort 2026-A Group 1".into(), Some("الدفعة 2026-أ مجموعة 1".into()), "VOC_Y1".into(), "G1".into()),
                ("Cohort 2026-A Group 2".into(), Some("الدفعة 2026-أ مجموعة 2".into()), "VOC_Y1".into(), "G2".into()),
            ],
        }
    }

    /// Idempotently provisions standard blueprint defaults (Year, Terms, Stages, Classes, Schemes, Fees)
    /// for administrators who request accelerated setup.
    pub async fn provision_blueprint_defaults(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        req: ProvisionDefaultsRequest,
    ) -> Result<ProvisionDefaultsResult, DomainError> {
        self.permission_checker
            .check(ctx, Permission::OrganizationWrite)
            .await?;

        let org =
            self.org_repo.find_by_id(org_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Organization {} not found", org_id))
            })?;
        let institution_type_str = org
            .metadata
            .as_ref()
            .and_then(|m| m.get("institution_type"))
            .and_then(|v| v.as_str())
            .unwrap_or("school");
        let inst_type = InstitutionType::from_str_loose(institution_type_str);
        let country_code = org
            .metadata
            .as_ref()
            .and_then(|m| m.get("country_code"))
            .and_then(|v| v.as_str());
        let country_profile = orbis_domain::entities::country_profile::CountryProfile::resolve(country_code);

        let now = Utc::now();
        let mut created_items = Vec::new();

        // Only create the sample campus when Section scaffolding was explicitly requested.
        let units = self.org_unit_repo.find_by_org(org_id).await?;
        let main_unit_id = if let Some(unit) = units.first() {
            Some(unit.id)
        } else if req.provision_stages_and_grades.unwrap_or(false) {
            let unit_id = Uuid::new_v4();
            let unit = OrgUnit {
                id: unit_id,
                org_id,
                parent_id: None,
                path: "/MAIN/".to_string(),
                depth: 1,
                name: "Main Campus".to_string(),
                name_ar: Some("المقر الرئيسي".to_string()),
                code: Some("MAIN".to_string()),
                unit_type: "campus".to_string(),
                head_id: None,
                capacity: Some(500),
                room_number: None,
                floor: None,
                building: None,
                settings: None,
                local_policies: None,
                data_isolation: "shared".to_string(),
                children_count: 0,
                descendant_count: 0,
                sort_order: Some(1),
                is_active: true,
                created_by: None,
                updated_by: None,
                created_at: now,
                updated_at: now,
            };
            self.org_unit_repo.save(&unit).await?;
            created_items.push("Main Campus (OrgUnit)".to_string());
            Some(unit_id)
        } else {
            None
        };

        // Academic years and starter periods are opt-in; an organization may use another
        // calendar model or intentionally operate without a single year container.
        let existing_years = self.year_repo.find_by_org(org_id).await?;
        let should_create_year = req.provision_academic_calendar.unwrap_or(false)
            || req.provision_default_academic_periods.unwrap_or(false)
            || req.academic_calendar_name.is_some()
            || req.academic_calendar_name_ar.is_some()
            || req.start_date.is_some()
            || req.end_date.is_some()
            || req.provision_hijri_school_calendar.unwrap_or(false);
        if existing_years.is_empty() && req.calendar_id.is_some() && !should_create_year {
            return Err(DomainError::Validation(
                "The selected academic year does not exist for this organization.".into(),
            ));
        }
        let active_year_id = if existing_years.is_empty() && should_create_year {
            if req.calendar_id.is_some() {
                return Err(DomainError::Validation(
                    "The selected academic year does not exist for this organization.".into(),
                ));
            }
            let year_name = req
                .academic_calendar_name
                .filter(|name| !name.trim().is_empty())
                .ok_or_else(|| {
                    DomainError::Validation(
                        "An academic year name is required when creating an academic year."
                            .into(),
                    )
                })?;
            let start_date = req.start_date.ok_or_else(|| {
                DomainError::Validation(
                    "An academic year start date is required when creating an academic year."
                        .into(),
                )
            })?;
            let end_date = req.end_date.ok_or_else(|| {
                DomainError::Validation(
                    "An academic year end date is required when creating an academic year."
                        .into(),
                )
            })?;
            if end_date <= start_date {
                return Err(DomainError::Validation(
                    "Academic year end date must be after its start date.".into(),
                ));
            }
            let year_id = Uuid::new_v4();
            let current_year_num = chrono::Datelike::year(&now);

            let year = AcademicCalendar {
                id: year_id,
                org_id,
                code: None,
                name: year_name,
                name_ar: req
                    .academic_calendar_name_ar
                    .filter(|name| !name.trim().is_empty()),
                start_date,
                end_date,
                starts_at: Some(start_date),
                ends_at: Some(end_date),
                hijri_start: None,
                hijri_end: None,
                status: AcademicCalendarStatus::Active,
                is_locked: false,
                locked_at: None,
                locked_by: None,
                lock_reason: None,
                current_period_id: None,
                archived_at: None,
                archived_by: None,
                notes: None,
                created_at: now,
                updated_at: now,
            };
            self.year_repo.save(&year).await?;
            created_items.push("Active Academic Year".to_string());

            // Starter period templates remain available only when explicitly requested.
            if req.provision_default_academic_periods.unwrap_or(false) {
            match inst_type {
                InstitutionType::School => {
                    if !req.provision_hijri_school_calendar.unwrap_or(false) {
                        // ── Term 1 (Sep - Jan) ────────────────────
                        let t1_id = Uuid::new_v4();
                        let t1_start = start_date;
                        let t1_end = chrono::NaiveDate::from_ymd_opt(current_year_num + 1, 1, 31)
                            .unwrap()
                            .and_hms_opt(23, 59, 59)
                            .unwrap()
                            .and_utc();
                        let t1 = AcademicPeriod {
                            id: t1_id,
                            org_id,
                            year_id,
                            parent_id: None,
                            name: "First Term".to_string(),
                            name_ar: Some("الفصل الدراسي الأول".to_string()),
                            code: Some("T1".to_string()),
                            period_type: PeriodType::Semester,
                            period_kind: PeriodKind::Teaching,
                            state: AcademicPeriodState::ActiveTeaching,
                            start_date: t1_start,
                            end_date: t1_end,
                            hijri_start: None,
                            hijri_end: None,
                            enrollment_starts_at: None,
                            enrollment_ends_at: None,
                            is_enrollment_open: true,
                            grade_entry_starts_at: None,
                            grade_entry_ends_at: None,
                            is_grade_entry_open: true,
                            exam_starts_at: None,
                            exam_ends_at: None,
                            is_locked: false,
                            locked_at: None,
                            locked_by: None,
                            sort_order: Some(1),
                            metadata: None,
                            created_at: now,
                            updated_at: now,
                        };
                        self.period_repo.save(&t1).await?;

                        // ── Term 1 Monthly Periods (3 months) + Exam ──
                        let t1_months: Vec<(u32, u32, &str, &str, &str)> = vec![
                            (current_year_num as u32, 9, "M1", "Month 1", "الشهر الأول"),
                            (current_year_num as u32, 10, "M2", "Month 2", "الشهر الثاني"),
                            (current_year_num as u32, 11, "M3", "Month 3", "الشهر الثالث"),
                        ];
                        for (idx, (yr, mon, code, name_en, name_ar)) in t1_months.iter().enumerate()
                        {
                            let m_start = chrono::NaiveDate::from_ymd_opt(*yr as i32, *mon, 1)
                                .unwrap()
                                .and_hms_opt(0, 0, 0)
                                .unwrap()
                                .and_utc();
                            let last_day = if *mon == 12 {
                                31
                            } else {
                                chrono::NaiveDate::from_ymd_opt(*yr as i32, *mon + 1, 1)
                                    .unwrap()
                                    .pred_opt()
                                    .unwrap()
                                    .day()
                            };
                            let m_end = chrono::NaiveDate::from_ymd_opt(*yr as i32, *mon, last_day)
                                .unwrap()
                                .and_hms_opt(23, 59, 59)
                                .unwrap()
                                .and_utc();
                            let mp = AcademicPeriod {
                                id: Uuid::new_v4(),
                                org_id,
                                year_id,
                                parent_id: Some(t1_id),
                                name: name_en.to_string(),
                                name_ar: Some(name_ar.to_string()),
                                code: Some(code.to_string()),
                                period_type: PeriodType::Month,
                                period_kind: PeriodKind::Teaching,
                                state: if idx == 0 {
                                    AcademicPeriodState::ActiveTeaching
                                } else {
                                    AcademicPeriodState::Planning
                                },
                                start_date: m_start,
                                end_date: m_end,
                                hijri_start: None,
                                hijri_end: None,
                                enrollment_starts_at: None,
                                enrollment_ends_at: None,
                                is_enrollment_open: false,
                                grade_entry_starts_at: None,
                                grade_entry_ends_at: None,
                                is_grade_entry_open: idx == 0,
                                exam_starts_at: None,
                                exam_ends_at: None,
                                is_locked: false,
                                locked_at: None,
                                locked_by: None,
                                sort_order: Some((idx + 1) as i32),
                                metadata: None,
                                created_at: now,
                                updated_at: now,
                            };
                            self.period_repo.save(&mp).await?;
                        }
                        // Term 1 Exam Period
                        let t1_exam_start =
                            chrono::NaiveDate::from_ymd_opt(current_year_num + 1, 1, 1)
                                .unwrap()
                                .and_hms_opt(0, 0, 0)
                                .unwrap()
                                .and_utc();
                        let t1_exam_end =
                            chrono::NaiveDate::from_ymd_opt(current_year_num + 1, 1, 20)
                                .unwrap()
                                .and_hms_opt(23, 59, 59)
                                .unwrap()
                                .and_utc();
                        let t1_exam = AcademicPeriod {
                            id: Uuid::new_v4(),
                            org_id,
                            year_id,
                            parent_id: Some(t1_id),
                            name: "Term 1 Examination".to_string(),
                            name_ar: Some("اختبار الفصل الأول".to_string()),
                            code: Some("T1-EXAM".to_string()),
                            period_type: PeriodType::Month,
                            period_kind: PeriodKind::Examination,
                            state: AcademicPeriodState::Planning,
                            start_date: t1_exam_start,
                            end_date: t1_exam_end,
                            hijri_start: None,
                            hijri_end: None,
                            enrollment_starts_at: None,
                            enrollment_ends_at: None,
                            is_enrollment_open: false,
                            grade_entry_starts_at: None,
                            grade_entry_ends_at: None,
                            is_grade_entry_open: false,
                            exam_starts_at: Some(t1_exam_start),
                            exam_ends_at: Some(t1_exam_end),
                            is_locked: false,
                            locked_at: None,
                            locked_by: None,
                            sort_order: Some(4),
                            metadata: None,
                            created_at: now,
                            updated_at: now,
                        };
                        self.period_repo.save(&t1_exam).await?;

                        // ── Term 2 (Feb - Jun) ────────────────────
                        let t2_id = Uuid::new_v4();
                        let t2_start = chrono::NaiveDate::from_ymd_opt(current_year_num + 1, 2, 1)
                            .unwrap()
                            .and_hms_opt(0, 0, 0)
                            .unwrap()
                            .and_utc();
                        let t2_end = end_date;
                        let t2 = AcademicPeriod {
                            id: t2_id,
                            org_id,
                            year_id,
                            parent_id: None,
                            name: "Second Term".to_string(),
                            name_ar: Some("الفصل الدراسي الثاني".to_string()),
                            code: Some("T2".to_string()),
                            period_type: PeriodType::Semester,
                            period_kind: PeriodKind::Teaching,
                            state: AcademicPeriodState::Planning,
                            start_date: t2_start,
                            end_date: t2_end,
                            hijri_start: None,
                            hijri_end: None,
                            enrollment_starts_at: None,
                            enrollment_ends_at: None,
                            is_enrollment_open: false,
                            grade_entry_starts_at: None,
                            grade_entry_ends_at: None,
                            is_grade_entry_open: false,
                            exam_starts_at: None,
                            exam_ends_at: None,
                            is_locked: false,
                            locked_at: None,
                            locked_by: None,
                            sort_order: Some(2),
                            metadata: None,
                            created_at: now,
                            updated_at: now,
                        };
                        self.period_repo.save(&t2).await?;

                        // ── Term 2 Monthly Periods (3 months) + Exam ──
                        let t2_months: Vec<(u32, u32, &str, &str, &str)> = vec![
                            (
                                current_year_num as u32 + 1,
                                2,
                                "M4",
                                "Month 4",
                                "الشهر الرابع",
                            ),
                            (
                                current_year_num as u32 + 1,
                                3,
                                "M5",
                                "Month 5",
                                "الشهر الخامس",
                            ),
                            (
                                current_year_num as u32 + 1,
                                4,
                                "M6",
                                "Month 6",
                                "الشهر السادس",
                            ),
                        ];
                        for (idx, (yr, mon, code, name_en, name_ar)) in t2_months.iter().enumerate()
                        {
                            let m_start = chrono::NaiveDate::from_ymd_opt(*yr as i32, *mon, 1)
                                .unwrap()
                                .and_hms_opt(0, 0, 0)
                                .unwrap()
                                .and_utc();
                            let last_day = chrono::NaiveDate::from_ymd_opt(*yr as i32, *mon + 1, 1)
                                .unwrap()
                                .pred_opt()
                                .unwrap()
                                .day();
                            let m_end = chrono::NaiveDate::from_ymd_opt(*yr as i32, *mon, last_day)
                                .unwrap()
                                .and_hms_opt(23, 59, 59)
                                .unwrap()
                                .and_utc();
                            let mp = AcademicPeriod {
                                id: Uuid::new_v4(),
                                org_id,
                                year_id,
                                parent_id: Some(t2_id),
                                name: name_en.to_string(),
                                name_ar: Some(name_ar.to_string()),
                                code: Some(code.to_string()),
                                period_type: PeriodType::Month,
                                period_kind: PeriodKind::Teaching,
                                state: AcademicPeriodState::Planning,
                                start_date: m_start,
                                end_date: m_end,
                                hijri_start: None,
                                hijri_end: None,
                                enrollment_starts_at: None,
                                enrollment_ends_at: None,
                                is_enrollment_open: false,
                                grade_entry_starts_at: None,
                                grade_entry_ends_at: None,
                                is_grade_entry_open: false,
                                exam_starts_at: None,
                                exam_ends_at: None,
                                is_locked: false,
                                locked_at: None,
                                locked_by: None,
                                sort_order: Some((idx + 1) as i32),
                                metadata: None,
                                created_at: now,
                                updated_at: now,
                            };
                            self.period_repo.save(&mp).await?;
                        }
                        // Term 2 Exam Period
                        let t2_exam_start =
                            chrono::NaiveDate::from_ymd_opt(current_year_num + 1, 5, 15)
                                .unwrap()
                                .and_hms_opt(0, 0, 0)
                                .unwrap()
                                .and_utc();
                        let t2_exam_end =
                            chrono::NaiveDate::from_ymd_opt(current_year_num + 1, 6, 5)
                                .unwrap()
                                .and_hms_opt(23, 59, 59)
                                .unwrap()
                                .and_utc();
                        let t2_exam = AcademicPeriod {
                            id: Uuid::new_v4(),
                            org_id,
                            year_id,
                            parent_id: Some(t2_id),
                            name: "Term 2 Examination".to_string(),
                            name_ar: Some("اختبار الفصل الثاني".to_string()),
                            code: Some("T2-EXAM".to_string()),
                            period_type: PeriodType::Month,
                            period_kind: PeriodKind::Examination,
                            state: AcademicPeriodState::Planning,
                            start_date: t2_exam_start,
                            end_date: t2_exam_end,
                            hijri_start: None,
                            hijri_end: None,
                            enrollment_starts_at: None,
                            enrollment_ends_at: None,
                            is_enrollment_open: false,
                            grade_entry_starts_at: None,
                            grade_entry_ends_at: None,
                            is_grade_entry_open: false,
                            exam_starts_at: Some(t2_exam_start),
                            exam_ends_at: Some(t2_exam_end),
                            is_locked: false,
                            locked_at: None,
                            locked_by: None,
                            sort_order: Some(4),
                            metadata: None,
                            created_at: now,
                            updated_at: now,
                        };
                        self.period_repo.save(&t2_exam).await?;

                        created_items.push("Terms with Monthly Periods & Exam Sessions (T1: 3 months + exam, T2: 3 months + exam)".to_string());
                    }
                }
                InstitutionType::University => {
                    // Semester 1 (Fall)
                    let sem1_id = Uuid::new_v4();
                    let sem1 = AcademicPeriod {
                        id: sem1_id,
                        org_id,
                        year_id,
                        parent_id: None,
                        name: "Fall Semester".to_string(),
                        name_ar: Some("فصل الخريف".to_string()),
                        code: Some("FALL".to_string()),
                        period_type: PeriodType::Semester,
                        period_kind: PeriodKind::Teaching,
                        state: AcademicPeriodState::ActiveTeaching,
                        start_date,
                        end_date: chrono::NaiveDate::from_ymd_opt(current_year_num + 1, 1, 31)
                            .unwrap()
                            .and_hms_opt(23, 59, 59)
                            .unwrap()
                            .and_utc(),
                        hijri_start: None,
                        hijri_end: None,
                        enrollment_starts_at: None,
                        enrollment_ends_at: None,
                        is_enrollment_open: true,
                        grade_entry_starts_at: None,
                        grade_entry_ends_at: None,
                        is_grade_entry_open: true,
                        exam_starts_at: None,
                        exam_ends_at: None,
                        is_locked: false,
                        locked_at: None,
                        locked_by: None,
                        sort_order: Some(1),
                        metadata: None,
                        created_at: now,
                        updated_at: now,
                    };
                    self.period_repo.save(&sem1).await?;

                    // Semester 2 (Spring)
                    let sem2_id = Uuid::new_v4();
                    let sem2 = AcademicPeriod {
                        id: sem2_id,
                        org_id,
                        year_id,
                        parent_id: None,
                        name: "Spring Semester".to_string(),
                        name_ar: Some("فصل الربيع".to_string()),
                        code: Some("SPRING".to_string()),
                        period_type: PeriodType::Semester,
                        period_kind: PeriodKind::Teaching,
                        state: AcademicPeriodState::Planning,
                        start_date: chrono::NaiveDate::from_ymd_opt(current_year_num + 1, 2, 1)
                            .unwrap()
                            .and_hms_opt(0, 0, 0)
                            .unwrap()
                            .and_utc(),
                        end_date,
                        hijri_start: None,
                        hijri_end: None,
                        enrollment_starts_at: None,
                        enrollment_ends_at: None,
                        is_enrollment_open: false,
                        grade_entry_starts_at: None,
                        grade_entry_ends_at: None,
                        is_grade_entry_open: false,
                        exam_starts_at: None,
                        exam_ends_at: None,
                        is_locked: false,
                        locked_at: None,
                        locked_by: None,
                        sort_order: Some(2),
                        metadata: None,
                        created_at: now,
                        updated_at: now,
                    };
                    self.period_repo.save(&sem2).await?;
                    created_items.push("Semesters (Fall & Spring)".to_string());
                }
                InstitutionType::Institute => {
                    let cohort_id = Uuid::new_v4();
                    let cohort = AcademicPeriod {
                        id: cohort_id,
                        org_id,
                        year_id,
                        parent_id: None,
                        name: "Learning Cohort 01".to_string(),
                        name_ar: Some("دفعة تعليمية 01".to_string()),
                        code: Some("LC01".to_string()),
                        period_type: PeriodType::Module,
                        period_kind: PeriodKind::Teaching,
                        state: AcademicPeriodState::ActiveTeaching,
                        start_date,
                        end_date,
                        hijri_start: None,
                        hijri_end: None,
                        enrollment_starts_at: None,
                        enrollment_ends_at: None,
                        is_enrollment_open: true,
                        grade_entry_starts_at: None,
                        grade_entry_ends_at: None,
                        is_grade_entry_open: true,
                        exam_starts_at: None,
                        exam_ends_at: None,
                        is_locked: false,
                        locked_at: None,
                        locked_by: None,
                        sort_order: Some(1),
                        metadata: None,
                        created_at: now,
                        updated_at: now,
                    };
                    self.period_repo.save(&cohort).await?;
                    created_items.push("Training Cohort Cycle".to_string());
                }
            }
            }
            Some(year_id)
        } else if !existing_years.is_empty() {
            Some(if let Some(requested_year_id) = req.calendar_id {
                existing_years
                    .iter()
                    .find(|year| year.id == requested_year_id)
                    .map(|year| year.id)
                    .ok_or_else(|| {
                        DomainError::Validation(
                            "The selected academic year does not belong to this organization."
                                .into(),
                        )
                    })?
            } else {
                existing_years
                    .iter()
                    .find(|year| year.status == "active")
                    .or_else(|| existing_years.first())
                    .map(|year| year.id)
                    .ok_or_else(|| {
                        DomainError::NotFound(
                            "No academic year is available for default provisioning.".into(),
                        )
                    })?
            })
        } else {
            None
        };

        let mut school_periods = Vec::new();
        if req.provision_hijri_school_calendar.unwrap_or(false) {
            if inst_type != InstitutionType::School {
                return Err(DomainError::Validation(
                    "The Hijri school calendar can only be provisioned for school organizations."
                        .into(),
                ));
            }
            let hijri_start_year = req.hijri_start_year.unwrap_or(1448);
            let active_year_id = active_year_id.ok_or_else(|| {
                DomainError::Validation(
                    "Create or select an academic year before provisioning calendar periods."
                        .into(),
                )
            })?;
            school_periods = self
                .provision_hijri_school_calendar(
                    org_id,
                    active_year_id,
                    hijri_start_year,
                    req.hijri_start_month.unwrap_or(1),
                    req.hijri_start_day.unwrap_or(9),
                    req.hijri_month_lengths,
                    now,
                )
                .await?;

            // Automatically provision official supplementary periods for Yemeni schools
            // (admin registration opening, teacher prep, mid-year break, basic exams, secondary exams)
            if country_profile.country_code == "YE" {
                let existing_codes: Vec<String> =
                    school_periods.iter().filter_map(|p| p.code.clone()).collect();
                if let Ok(Some(year_obj)) = self.year_repo.find_by_id(active_year_id).await {
                    let term1_end = school_periods
                        .iter()
                        .find(|p| p.code.as_deref() == Some("T1") && p.parent_id.is_none())
                        .map(|p| p.end_date.date_naive());
                    let term2_end = school_periods
                        .iter()
                        .find(|p| p.code.as_deref() == Some("T2") && p.parent_id.is_none())
                        .map(|p| p.end_date.date_naive());
                    let supp = super::calendar_templates::yemen_1447_1448::build_supplementary_periods(
                        org_id,
                        active_year_id,
                        year_obj.start_date.date_naive(),
                        term1_end,
                        term2_end,
                        &existing_codes,
                        now,
                    );
                    for p in supp {
                        let _ = self.period_repo.save(&p).await;
                        school_periods.push(p);
                    }
                }
            }

            created_items.push(
                "Hijri academic calendar: two terms, eight teaching months, monthly exams, term finals, and official supplementary breaks & exam periods"
                    .to_string(),
            );
        }

        if req.provision_time_slots.unwrap_or(false) {
            let active_year_id = active_year_id.ok_or_else(|| {
                DomainError::Validation(
                    "Create or select an academic year before provisioning timetable slots."
                        .into(),
                )
            })?;
            if school_periods.is_empty() {
                school_periods = self.period_repo.find_by_year(active_year_id).await?;
            }
            let is_yemen = country_profile.country_code == "YE"
                || req.provision_hijri_school_calendar.unwrap_or(false);
            let created_slots = self
                .provision_default_school_time_slots(&school_periods, is_yemen, now)
                .await?;
            if created_slots == 0
                && !school_periods.iter().any(|period| {
                    period.parent_id.is_some()
                        && period.period_type == PeriodType::Month
                        && period.period_kind == PeriodKind::Teaching
                })
            {
                return Err(DomainError::Validation(
                    "Create teaching periods before provisioning the default timetable.".into(),
                ));
            }
            if created_slots > 0 {
                created_items.push(format!(
                    "Default weekly timetable ({} time slots)",
                    created_slots
                ));
            }
        }

        if req.enable_period_autopilot.unwrap_or(false) {
            let mut settings = self.settings_repo.get_settings(org_id).await?;
            settings.autopilot.auto_advance_periods = true;
            settings.autopilot.auto_period_lock = true;
            self.settings_repo.save_settings(org_id, &settings).await?;
            created_items.push("Automatic academic period opening and closing".to_string());
        }

        // 3. Provision any missing core subjects when requested.
        let existing_subjects = self
            .subject_repo
            .list(
                org_id,
                SubjectFilter {
                    pagination: Some(PaginationParams {
                        page_size: Some(100),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await?;
        if req.provision_subjects.unwrap_or(false) {
            let subject_templates = Self::subjects_for_profile_and_type(&country_profile, &inst_type);

            let mut created_subjects = 0;
            for (code, name, name_ar, is_core, hours) in subject_templates {
                if existing_subjects
                    .items
                    .iter()
                    .any(|subject| subject.code == code)
                {
                    continue;
                }
                let sub = Subject {
                    id: Uuid::new_v4(),
                    org_id,
                    unit_id: main_unit_id,
                    code: code.to_string(),
                    name: name.to_string(),
                    name_ar: name_ar,
                    description: None,
                    subject_type: "subject".to_string(),
                    is_core,
                    is_elective: !is_core,
                    credit_units: Some(Decimal::from(hours)),
                    workload_units: None,
                    theory_hours: Some(Decimal::from(hours)),
                    practical_hours: None,
                    is_lab_required: false,
                    max_students_per_section: Some(30),
                    color: None,
                    icon: None,
                    sort_order: None,
                    is_active: true,
                    created_at: now,
                    updated_at: now,
                };
                self.subject_repo.save(&sub).await?;
                created_subjects += 1;
            }
            if created_subjects > 0 {
                created_items.push(format!(
                    "Core Curriculum Subjects ({} added)",
                    created_subjects
                ));
            }
        }

        // 4. Provision Starter Classes if requested & empty
        let existing_classes = self
            .section_repo
            .list(
                org_id,
                SectionFilter {
                    pagination: Some(PaginationParams {
                        page_size: Some(100),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await?;
        if existing_classes.total_count == 0 && req.provision_stages_and_grades.unwrap_or(false) {
            let active_year_id = active_year_id.ok_or_else(|| {
                DomainError::Validation(
                    "Create or select an academic year before provisioning learning groups."
                        .into(),
                )
            })?;
            let _active_period = self
                .period_repo
                .find_by_year(active_year_id)
                .await?
                .into_iter()
                .find(|period| period.period_kind == PeriodKind::Teaching)
                .ok_or_else(|| {
                    DomainError::Validation(
                        "Create a teaching period before provisioning learning groups.".into(),
                    )
                })?;
            let generic_class_templates = Self::sections_for_profile_and_type(&country_profile, &inst_type);

            for (name, name_ar, level, div) in generic_class_templates {
                if existing_classes
                    .items
                    .iter()
                    .any(|section| section.level.as_deref() == Some(level.as_str()))
                {
                    continue;
                }
                let Some(main_unit_id) = main_unit_id else {
                    return Err(DomainError::Validation(
                        "Create an organizational unit before provisioning classes.".into(),
                    ));
                };
                let cls = Section {
                    id: Uuid::new_v4(),
                    org_id,
                    org_unit_id: Some(main_unit_id),
                    section_type: "section".to_string(),
                    code: format!("{}-{}", level, div),
                    name: name.to_string(),
                    name_ar: name_ar.map(|s| s.to_string()),
                    level: Some(level.to_string()),
                    track: Some(div.to_string()),
                    capacity: Some(30),
                    color: None,
                    is_active: true,
                    is_deleted: false,
                    deleted_at: None,
                    created_at: now,
                    updated_at: now,
                };
                self.section_repo.save(&cls).await?;
            }
            created_items.push("Starter Learning Groups".to_string());
        }

        // 5. Provision Assessment Schemes if requested & empty
        let existing_schemes = self.assessment_scheme_repo.find_by_org(org_id).await?;
        if existing_schemes.is_empty() && req.provision_assessment_scheme.unwrap_or(false) {
            match inst_type {
                InstitutionType::School => {
                    // 1. Monthly 100-point periodic scheme
                    let monthly_scheme = AssessmentScheme::default_school_monthly(org_id);
                    self.assessment_scheme_repo.save(&monthly_scheme).await?;

                    // 2. Term Exam 30-point scheme
                    let term_exam_scheme = AssessmentScheme::default_periodic_term_exam(org_id);
                    self.assessment_scheme_repo.save(&term_exam_scheme).await?;

                    if country_profile.country_code == "YE" {
                        // 3. Yemeni Term Aggregation (50 pts = 20 coursework + 30 exam)
                        let term_agg_scheme = AssessmentScheme::default_yemen_school_term_aggregation(org_id);
                        self.assessment_scheme_repo.save(&term_agg_scheme).await?;

                        // 4. Yemeni Thanawiya Certificate (80% ministerial + 20% cumulative prior years)
                        let thanawiya_scheme = AssessmentScheme::yemen_secondary_certificate(org_id);
                        self.assessment_scheme_repo.save(&thanawiya_scheme).await?;

                        // 5. Yemeni Basic Certificate (Grade 9)
                        let basic_scheme = AssessmentScheme::yemen_basic_certificate(org_id);
                        self.assessment_scheme_repo.save(&basic_scheme).await?;

                        created_items.push("Official Yemeni Assessment Framework (Monthly 100-pt, Term Exam 30-pt, Term Aggregation 50-pt, Thanawiya Certificate 80/20, Basic Certificate)".to_string());
                    } else {
                        created_items.push("Standard School Assessment Schemes (Monthly 100-pt & Term Exam 30-pt)".to_string());
                    }
                }
                InstitutionType::University => {
                    let uni_scheme = AssessmentScheme::default_university_credit(org_id);
                    self.assessment_scheme_repo.save(&uni_scheme).await?;
                    created_items.push("University Credit-Hour Assessment Scheme (100-pt)".to_string());
                }
                InstitutionType::Institute => {
                    let module_scheme = AssessmentScheme::default_institute_module(org_id);
                    self.assessment_scheme_repo.save(&module_scheme).await?;
                    let competency_scheme = AssessmentScheme::default_competency_vocational(org_id);
                    self.assessment_scheme_repo.save(&competency_scheme).await?;
                    created_items.push("Technical & Vocational Competency Assessment Schemes".to_string());
                }
            }
        }

        // 6. Provision Tuition Fee Template if requested & empty
        let existing_fees = self.fee_template_repo.find_by_org(org_id).await?;
        if existing_fees.is_empty() && req.provision_tuition_fee_template.unwrap_or(false) {
            let accounts = self.account_repo.find_by_org(org_id).await?;
            let rev_acct = accounts
                .iter()
                .find(|a| {
                    a.code == "410101"
                        || a.code == "4101"
                        || a.code == "4000"
                        || a.account_type == orbis_domain::entities::account::AccountType::Revenue
                })
                .map(|a| a.id)
                .unwrap_or_else(Uuid::new_v4);
            let rec_acct = accounts
                .iter()
                .find(|a| {
                    a.code == "110301"
                        || a.code == "1103"
                        || a.code == "1120"
                        || (a.account_type == orbis_domain::entities::account::AccountType::Asset
                            && a.code.starts_with("1103"))
                })
                .map(|a| a.id)
                .unwrap_or_else(Uuid::new_v4);

            let fee_tmpl = FeeTemplate {
                id: Uuid::new_v4(),
                org_id,
                code: "TUITION-ANNUAL".to_string(),
                name: "Annual Academic Tuition".to_string(),
                name_ar: Some("الرسوم الدراسية السنوية".to_string()),
                category: "tuition".to_string(),
                default_amount: Decimal::ZERO, // Placeholder — institution must set actual tuition amount
                currency_code: org
                    .base_currency_code
                    .clone()
                    .unwrap_or_else(|| "SAR".to_string()),
                revenue_account_id: rev_acct,
                receivable_account_id: rec_acct,
                applicable_levels: None, // Applies to all unless filtered
                applicable_study_modes: None,
                frequency: FeeFrequency::Annual,
                is_mandatory: true,
                is_active: true,
                created_at: now,
                updated_at: now,
            };
            self.fee_template_repo.save(&fee_tmpl).await?;
            created_items.push("Tuition Fee Template".to_string());
        }

        // 7. Provision Standard Discount Templates if requested & empty (opt-in demo data)
        let existing_discounts = self.discount_template_repo.find_by_org(org_id).await?;
        if existing_discounts.is_empty() && req.provision_discount_templates.unwrap_or(false) {
            let sibling_disc = DiscountTemplate {
                id: Uuid::new_v4(),
                org_id,
                code: "DISC-SIBLING".to_string(),
                name: "Sibling Discount (10%)".to_string(),
                name_ar: Some("خصم الأخوة (10%)".to_string()),
                discount_type: DiscountType::Percentage,
                value: Decimal::from(10),
                eligibility_rules: Some(
                    serde_json::json!({ "type": "sibling", "account_code": "410301" }),
                ),
                is_approval_required: false,
                is_auto_apply: true,
                is_stackable: false,
                priority: 1,
                is_active: true,
                created_at: now,
                updated_at: now,
            };
            self.discount_template_repo.save(&sibling_disc).await?;

            let merit_disc = DiscountTemplate {
                id: Uuid::new_v4(),
                org_id,
                code: "DISC-MERIT".to_string(),
                name: "Academic Excellence Scholarship (20%)".to_string(),
                name_ar: Some("منحة التفوق الأكاديمي (20%)".to_string()),
                discount_type: DiscountType::Percentage,
                value: Decimal::from(20),
                eligibility_rules: Some(
                    serde_json::json!({ "type": "merit", "account_code": "410302" }),
                ),
                is_approval_required: true,
                is_auto_apply: false,
                is_stackable: false,
                priority: 2,
                is_active: true,
                created_at: now,
                updated_at: now,
            };
            self.discount_template_repo.save(&merit_disc).await?;
            created_items.push("Standard Discount Templates (Sibling, Merit)".to_string());
        }

        // 8. Provision Master Report Definitions if empty
        let existing_reports = self.definition_repo.list_by_org(org_id, None, None).await?;
        if existing_reports.is_empty() {
            let count = self.definition_repo.provision_catalog(org_id).await?;
            created_items.push(format!("Report Definitions ({} reports)", count));
        }

        // 9. Provision Master Report Templates if empty
        let existing_templates = self
            .template_repo
            .list_by_org(org_id, None, None, None)
            .await?;
        if existing_templates.is_empty() {
            let admin_id = ctx.user_id().ok_or_else(|| {
                DomainError::Validation(
                    "Authenticated user required for template provisioning".into(),
                )
            })?;
            let count = self
                .template_repo
                .provision_system_templates(org_id, admin_id)
                .await?;
            created_items.push(format!("Document Templates ({} templates)", count));
        }

        let updated_state = self.get_state(org_id).await?;

        Ok(ProvisionDefaultsResult {
            success: true,
            created_items,
            state: updated_state,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        OnboardingTaskStatus, hijri_month_position, resolve_task_status, OnboardingService,
    };

    #[test]
    fn hijri_month_position_rolls_over_to_the_next_year() {
        assert_eq!(hijri_month_position(1448, 10, 0), (1448, 9));
        assert_eq!(hijri_month_position(1448, 10, 2), (1448, 11));
        assert_eq!(hijri_month_position(1448, 10, 3), (1449, 0));
    }

    #[test]
    fn task_with_default_data_is_blocked_until_its_dependencies_are_complete() {
        assert_eq!(
            resolve_task_status(true, false, 100, true),
            OnboardingTaskStatus::Blocked
        );
    }

    #[test]
    fn task_is_completed_when_its_data_and_dependencies_are_complete() {
        assert_eq!(
            resolve_task_status(true, true, 100, true),
            OnboardingTaskStatus::Completed
        );
    }

    #[test]
    fn generic_blueprint_templates_are_institution_neutral() {
        let subjects = OnboardingService::generic_subject_templates();
        let classes = OnboardingService::generic_class_templates();

        assert!(subjects.iter().all(|(_, name, _, _, _)| !name.to_lowercase().contains("kindergarten")));
        assert!(subjects.iter().all(|(_, name, _, _, _)| !name.to_lowercase().contains("grade ")));
        assert!(classes.iter().all(|(name, _, _, _)| !name.to_lowercase().contains("kindergarten")));
        assert!(classes.iter().all(|(name, _, _, _)| !name.to_lowercase().contains("grade ")));
        assert_eq!(subjects.len(), 3);
        assert_eq!(classes.len(), 3);
    }
}
