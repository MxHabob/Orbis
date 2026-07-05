//! Academic Context Service — centralized academic state resolution.
//!
//! This service resolves the currently active academic cycle based on
//! the current date, and provides a unified `AcademicContext` object
//! that all modules can use instead of requiring manual cycle selection.
//!
//! # Design (inspired by Ellucian Academic Structure + PowerSchool context)
//!
//! - The active cycle is determined by date range (`start_date` ≤ today ≤ `end_date`).
//! - When multiple cycles overlap, the narrowest scope wins (semester > year).
//! - The `CyclePhase` enum computes the current lifecycle stage from dates.
//! - `validate_write_access` prevents mutations on archived years.

use serde::{Deserialize, Serialize};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QueryOrder};

use crate::entities::academic_cycle;

// ── Phase enum ─────────────────────────────────────────────────────

/// Lifecycle phase of an academic cycle, computed from its date fields.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CyclePhase {
    /// Before the cycle's start_date
    PreStart,
    /// Between start_date and exam_start_date (or end_date if no exam dates)
    Active,
    /// Between exam_start_date and exam_end_date
    Exams,
    /// Between exam_end_date and end_date (grade entry window)
    GradeEntry,
    /// After end_date
    Closed,
}

// ── Response DTOs ──────────────────────────────────────────────────

/// Summary of a single cycle (lightweight, for lists).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicCycleSummary {
    pub id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub cycle_type: String,
    pub start_date: String,
    pub end_date: String,
    pub is_current: bool,
    pub is_locked: bool,
    pub phase: CyclePhase,
}

/// Full context for the active cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicCycleContext {
    pub id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub cycle_type: String,
    pub start_date: String,
    pub end_date: String,
    pub is_enrollment_open: bool,
    pub is_grade_entry_open: bool,
    pub is_locked: bool,
    pub phase: CyclePhase,
}

/// The complete academic context returned to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicContext {
    /// Currently active year info (from registry).
    pub year_id: String,
    pub year_name: String,
    pub year_name_ar: Option<String>,
    pub year_status: String,
    pub is_read_only: bool,

    /// Auto-resolved active cycle (may be None if no cycle covers today).
    pub active_cycle: Option<AcademicCycleContext>,

    /// All cycles in the current year (lightweight summaries).
    pub all_cycles: Vec<AcademicCycleSummary>,
}

// ── Service ────────────────────────────────────────────────────────

pub struct AcademicContextService;

impl AcademicContextService {
    /// Resolve the active cycle for an organization based on the current date.
    ///
    /// Logic:
    /// 1. Find all cycles where `start_date <= today <= end_date`.
    /// 2. Among matches, prefer the narrowest scope (semester > year).
    /// 3. If no date match, fall back to `is_current = 1`.
    pub async fn resolve_active_cycle(
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<Option<academic_cycle::Model>, String> {
        let today = chrono::Local::now().date_naive().to_string();

        // Get all cycles for this org
        let all_cycles = academic_cycle::Entity::find()
            .filter(academic_cycle::Column::OrgId.eq(org_id))
            .order_by_asc(academic_cycle::Column::SortOrder)
            .all(db)
            .await
            .map_err(|e| e.to_string())?;

        // 1. Find cycles that contain today in their date range
        let mut date_matches: Vec<&academic_cycle::Model> = all_cycles
            .iter()
            .filter(|c| c.start_date.as_str() <= today.as_str() && today.as_str() <= c.end_date.as_str())
            .collect();

        if !date_matches.is_empty() {
            // 2. Sort by type priority: narrower scope wins
            date_matches.sort_by_key(|c| Self::type_priority(&c.cycle_type));
            return Ok(Some(date_matches[0].clone()));
        }

        // 3. Fallback: use is_current flag
        let current = all_cycles
            .iter()
            .find(|c| c.is_current == 1)
            .cloned();

        Ok(current)
    }

    /// Build the full academic context.
    ///
    /// Requires both the registry DB (for year info) and the year DB (for cycles).
    pub async fn get_context(
        registry_db: &DatabaseConnection,
        year_db: &DatabaseConnection,
        org_id: &str,
        year_id: &str,
        is_archived: bool,
    ) -> Result<AcademicContext, String> {
        // Get year info from registry
        let year = crate::entities::academic_year::Entity::find_by_id(year_id)
            .one(registry_db)
            .await
            .map_err(|e| e.to_string())?;

        let (year_name, year_name_ar, year_status) = match &year {
            Some(y) => (
                y.name.clone(),
                y.name_ar.clone(),
                y.status.clone(),
            ),
            None => (String::new(), None, "active".to_string()),
        };

        // Get all cycles from year DB
        let all_cycles_raw = academic_cycle::Entity::find()
            .filter(academic_cycle::Column::OrgId.eq(org_id))
            .order_by_asc(academic_cycle::Column::SortOrder)
            .all(year_db)
            .await
            .map_err(|e| e.to_string())?;

        // Resolve active cycle
        let resolved = Self::resolve_active_cycle(year_db, org_id).await?;

        // Build summaries
        let all_cycles: Vec<AcademicCycleSummary> = all_cycles_raw
            .iter()
            .map(|c| AcademicCycleSummary {
                id: c.id.clone(),
                name: c.name.clone(),
                name_ar: c.name_ar.clone(),
                cycle_type: c.cycle_type.clone(),
                start_date: c.start_date.clone(),
                end_date: c.end_date.clone(),
                is_current: c.is_current == 1,
                is_locked: c.is_locked == 1,
                phase: Self::compute_phase(c),
            })
            .collect();

        // Build active cycle context
        let active_cycle = resolved.map(|c| AcademicCycleContext {
            id: c.id.clone(),
            name: c.name.clone(),
            name_ar: c.name_ar.clone(),
            cycle_type: c.cycle_type.clone(),
            start_date: c.start_date.clone(),
            end_date: c.end_date.clone(),
            is_enrollment_open: c.is_enrollment_open == 1,
            is_grade_entry_open: c.is_grade_entry_open == 1,
            is_locked: c.is_locked == 1,
            phase: Self::compute_phase(&c),
        });

        Ok(AcademicContext {
            year_id: year_id.to_string(),
            year_name,
            year_name_ar,
            year_status: year_status.clone(),
            is_read_only: is_archived || year_status == "archived",
            active_cycle,
            all_cycles,
        })
    }

    /// Compute the lifecycle phase of a cycle based on the current date.
    pub fn compute_phase(cycle: &academic_cycle::Model) -> CyclePhase {
        let today = chrono::Local::now().date_naive().to_string();
        let today = today.as_str();

        // Before start
        if today < cycle.start_date.as_str() {
            return CyclePhase::PreStart;
        }

        // After end
        if today > cycle.end_date.as_str() {
            return CyclePhase::Closed;
        }

        // During exam period
        if let (Some(ref exam_start), Some(ref exam_end)) =
            (&cycle.exam_start_date, &cycle.exam_end_date)
        {
            if !exam_start.is_empty() && !exam_end.is_empty() {
                if today >= exam_start.as_str() && today <= exam_end.as_str() {
                    return CyclePhase::Exams;
                }
                // After exams but before cycle end → grade entry window
                if today > exam_end.as_str() {
                    return CyclePhase::GradeEntry;
                }
            }
        }

        // Grade entry explicitly open
        if cycle.is_grade_entry_open == 1 {
            return CyclePhase::GradeEntry;
        }

        CyclePhase::Active
    }

    /// Check if the current context allows write operations.
    pub fn validate_write_access(ctx: &AcademicContext) -> Result<(), String> {
        if ctx.is_read_only {
            return Err(
                "Cannot modify data in an archived year. Switch to an active year first."
                    .to_string(),
            );
        }
        Ok(())
    }

    // ── Internal helpers ──

    /// Priority for cycle type resolution: lower = more specific = higher priority.
    fn type_priority(cycle_type: &str) -> u8 {
        match cycle_type {
            "monthly" => 0,
            "quarter" => 1,
            "trimester" => 2,
            "semester" => 3,
            "custom" => 4,
            "year" => 5,
            _ => 6,
        }
    }
}
