//! Student Ordering Engine — orchestrates sorting ordinals for classroom listings and seating.

use std::sync::Arc;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_kernel::errors::AppError;
use crate::entities::student_ordering;
use crate::repository::{ExamCommandRepo, ExamQueryRepo};

/// Ordered student view returned by the ordering engine
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct OrderedStudent {
    pub enrollment_id: String,
    pub student_id: String,
    pub class_id: String,
    pub student_name_ar: Option<String>,
    pub level: String,
    pub division: Option<String>,
    pub order_in_class: i32,
    pub order_in_level: Option<i32>,
    pub order_in_org: Option<i32>,
    pub sort_key: String,
    pub is_frozen: bool,
}

impl From<student_ordering::Model> for OrderedStudent {
    fn from(m: student_ordering::Model) -> Self {
        Self {
            enrollment_id: m.enrollment_id,
            student_id: m.student_id,
            class_id: m.class_id,
            student_name_ar: m.student_name_ar,
            level: m.level,
            division: m.division,
            order_in_class: m.order_in_class,
            order_in_level: m.order_in_level,
            order_in_org: m.order_in_org,
            sort_key: m.sort_key,
            is_frozen: m.is_frozen == 1,
        }
    }
}

/// Result of a rebuild operation
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct RebuildOrderingResult {
    pub total_students: usize,
    pub ordered_count: usize,
    pub frozen_skipped: usize,
    pub strategy: String,
    pub version: i32,
}

pub struct StudentOrderingService {
    pub cmd: Arc<ExamCommandRepo>,
    pub query: Arc<ExamQueryRepo>,
}

impl StudentOrderingService {
    pub fn new(cmd: Arc<ExamCommandRepo>, query: Arc<ExamQueryRepo>) -> Self {
        Self { cmd, query }
    }

    /// Rebuild student ordering for an entire cycle using the specified strategy.
    pub async fn rebuild_ordering(
        &self,
        org_id: &str,
        cycle_id: &str,
        strategy: &str,
    ) -> Result<RebuildOrderingResult, AppError> {
        let valid_strategies = [
            "name_ar_asc",
            "academic_number",
            "division_name",
            "sex_name",
            "hybrid",
        ];
        if !valid_strategies.contains(&strategy) {
            return Err(AppError::validation(format!(
                "invalid ordering strategy '{}'. Valid: {:?}",
                strategy, valid_strategies
            )));
        }

        // 1. Fetch active enrollments
        let enrollments = self.query.get_enrollments_for_ordering(org_id, cycle_id).await?;

        if enrollments.is_empty() {
            return Ok(RebuildOrderingResult {
                total_students: 0,
                ordered_count: 0,
                frozen_skipped: 0,
                strategy: strategy.to_string(),
                version: 1,
            });
        }

        // 2. Get existing frozen records
        let existing = self.query.get_student_ordering(org_id, cycle_id, None).await?;

        let frozen_ids: std::collections::HashSet<String> = existing
            .iter()
            .filter(|o| o.is_frozen == 1)
            .map(|o| o.enrollment_id.clone())
            .collect();

        let frozen_skipped = frozen_ids.len();

        // 3. Delete non-frozen ordering records
        self.cmd.delete_student_ordering_for_cycle(org_id, cycle_id).await?;

        // 4. Build sort keys and sort
        let mut sortable: Vec<(String, String, String, String, Option<String>, Option<String>, Option<String>, Option<String>)> =
            enrollments
                .iter()
                .filter(|e| !frozen_ids.contains(&e.enrollment_id))
                .map(|e| {
                    let sort_key = build_sort_key(
                        strategy,
                        &e.level,
                        e.division.as_deref(),
                        e.student_name_ar.as_deref(),
                        e.academic_number.as_deref(),
                        e.student_sex.as_deref(),
                    );
                    (
                        sort_key,
                        e.enrollment_id.clone(),
                        e.student_id.clone(),
                        e.class_id.clone(),
                        Some(e.level.clone()),
                        e.division.clone(),
                        e.student_name_ar.clone(),
                        e.student_sex.clone(),
                    )
                })
                .collect();

        sortable.sort_by(|a, b| a.0.cmp(&b.0));

        // 5. Assign ordinals
        let now = Utc::now().to_rfc3339();
        let mut class_counters: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        let mut level_counters: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        let mut org_counter = 0i32;
        let mut ordered_count = 0usize;

        let max_version = existing.iter().map(|o| o.version).max().unwrap_or(0);
        let new_version = max_version + 1;

        for (sort_key, enrollment_id, student_id, class_id, level, division, name_ar, sex) in &sortable {
            let level_str = level.as_deref().unwrap_or("");

            let class_order = class_counters.entry(class_id.clone()).or_insert(0);
            *class_order += 1;
            let order_in_class = *class_order;

            let level_order = level_counters.entry(level_str.to_string()).or_insert(0);
            *level_order += 1;
            let order_in_level = *level_order;

            org_counter += 1;

            let model = student_ordering::Model {
                id: Uuid::new_v4().to_string(),
                org_id: org_id.to_string(),
                cycle_id: cycle_id.to_string(),
                enrollment_id: enrollment_id.clone(),
                student_id: student_id.clone(),
                class_id: class_id.clone(),
                sort_key: sort_key.clone(),
                order_in_class,
                order_in_division: None,
                order_in_level: Some(order_in_level),
                order_in_org: Some(org_counter),
                level: level_str.to_string(),
                division: division.clone(),
                student_name_ar: name_ar.clone(),
                student_sex: sex.clone(),
                academic_number: None,
                is_frozen: 0,
                frozen_at: None,
                frozen_by: None,
                strategy: strategy.to_string(),
                version: new_version,
                created_at: now.clone(),
                updated_at: now.clone(),
            };

            self.cmd.upsert_student_ordering(model).await?;
            ordered_count += 1;
        }

        Ok(RebuildOrderingResult {
            total_students: enrollments.len(),
            ordered_count,
            frozen_skipped,
            strategy: strategy.to_string(),
            version: new_version,
        })
    }

    /// Retrieve ordered students for a class.
    pub async fn get_ordered_students(
        &self,
        org_id: &str,
        cycle_id: &str,
        class_id: Option<&str>,
    ) -> Result<Vec<OrderedStudent>, AppError> {
        let records = self.query.get_student_ordering(org_id, cycle_id, class_id).await?;
        Ok(records.into_iter().map(OrderedStudent::from).collect())
    }

    /// Freeze ordering so it cannot be modified.
    pub async fn freeze_ordering(
        &self,
        org_id: &str,
        cycle_id: &str,
        frozen_by: &str,
    ) -> Result<u64, AppError> {
        Ok(self.cmd.freeze_student_ordering(org_id, cycle_id, frozen_by).await?)
    }
}

fn build_sort_key(
    strategy: &str,
    level: &str,
    division: Option<&str>,
    name_ar: Option<&str>,
    academic_number: Option<&str>,
    sex: Option<&str>,
) -> String {
    let div = division.unwrap_or("");
    let name = name_ar.unwrap_or("");
    let acad = academic_number.unwrap_or("");
    let s = sex.unwrap_or("");

    match strategy {
        "name_ar_asc" => format!("{}|{}|{}", level, div, name),
        "academic_number" => format!("{}|{}|{}", level, div, acad),
        "division_name" => format!("{}|{}|{}", level, div, name),
        "sex_name" => format!("{}|{}|{}", level, s, name),
        "hybrid" => format!("{}|{}|{}|{}", level, div, s, name),
        _ => format!("{}|{}|{}", level, div, name),
    }
}
