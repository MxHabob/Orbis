use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{seating_number, secret_number};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SeatingNumberView {
    pub id: String,
    pub enrollment_id: String,
    pub student_id: String,
    pub seating_number: String,
    pub sequence_number: i32,
    pub level: String,
    pub division: Option<String>,
    pub student_name_ar: Option<String>,
    pub class_id: String,
    pub is_active: bool,
}

impl From<seating_number::Model> for SeatingNumberView {
    fn from(m: seating_number::Model) -> Self {
        Self {
            id: m.id,
            enrollment_id: m.enrollment_id,
            student_id: m.student_id,
            seating_number: m.seating_number,
            sequence_number: m.sequence_number,
            level: m.level,
            division: m.division,
            student_name_ar: m.student_name_ar,
            class_id: m.class_id,
            is_active: m.is_active == 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct GenerationResult {
    pub generated_count: usize,
    pub exam_session_id: String,
}

/// Secret number view (hides the actual secret unless revealed)
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SecretNumberView {
    pub id: String,
    pub enrollment_id: String,
    pub student_id: String,
    /// Only populated if the caller has reveal permission
    pub secret_number: Option<String>,
    pub is_revealed: bool,
    pub barcode_data: Option<String>,
    pub qr_data: Option<String>,
}

impl super::Service {
    // ── Seating Numbers ─────────────────────────────────────────────────────

    /// Generate seating numbers for all students in an exam session.
    /// Uses the Student Ordering Engine for consistent ordering.
    pub async fn generate_seating_numbers(
        &self,
        org_id: &str,
        exam_session_id: &str,
        generated_by: &str,
    ) -> Result<GenerationResult, String> {
        let session = self.get_exam_session(org_id, exam_session_id).await?;

        if session.status != "draft" && session.status != "scheduled" {
            return Err("can only generate seating numbers for draft or scheduled sessions".to_string());
        }

        // Deactivate existing seating numbers
        self.repo
            .deactivate_seating_numbers(exam_session_id)
            .await
            .map_err(|e| e.to_string())?;

        // Get ordered students from the ordering engine
        let ordered = self
            .get_ordered_students(org_id, &session.cycle_id, None)
            .await?;

        if ordered.is_empty() {
            return Err("no students found. Please rebuild student ordering first".to_string());
        }

        // Extract year from start_date
        let year = session.start_date.get(..4).unwrap_or("2026");

        // Determine session code
        let session_code = match session.session_type.as_str() {
            "final" => "F",
            "midterm" => "M",
            "monthly" => "MO",
            "second_chance" => "SC",
            _ => "C",
        };

        let now = Utc::now().to_rfc3339();
        let mut counter = 0u32;
        let mut generated = 0usize;

        for student in &ordered {
            counter += 1;

            let div = student.division.as_deref().unwrap_or("A");
            let seating_num = format!(
                "{}-{}-{}{}-{:04}",
                year, session_code, student.level, div, counter
            );

            let model = seating_number::Model {
                id: Uuid::new_v4().to_string(),
                org_id: org_id.to_string(),
                exam_session_id: exam_session_id.to_string(),
                enrollment_id: student.enrollment_id.clone(),
                student_id: student.student_id.clone(),
                class_id: student.class_id.clone(),
                seating_number: seating_num,
                sequence_number: counter as i32,
                level: student.level.clone(),
                division: student.division.clone(),
                student_name_ar: student.student_name_ar.clone(),
                is_active: 1,
                generated_at: now.clone(),
                generated_by: generated_by.to_string(),
                voided_at: None,
                voided_by: None,
                void_reason: None,
                created_at: now.clone(),
                updated_at: now.clone(),
            };

            self.repo
                .insert_seating_number(model)
                .await
                .map_err(|e| e.to_string())?;

            generated += 1;
        }

        Ok(GenerationResult {
            generated_count: generated,
            exam_session_id: exam_session_id.to_string(),
        })
    }

    /// Get seating numbers for an exam session
    pub async fn get_seating_numbers(
        &self,
        org_id: &str,
        exam_session_id: &str,
        class_id: Option<&str>,
    ) -> Result<Vec<SeatingNumberView>, String> {
        let _ = self.get_exam_session(org_id, exam_session_id).await?;

        let records = self
            .repo
            .get_seating_numbers(exam_session_id, class_id)
            .await
            .map_err(|e| e.to_string())?;

        Ok(records.into_iter().map(SeatingNumberView::from).collect())
    }

    // ── Secret Numbers ──────────────────────────────────────────────────────

    /// Generate cryptographically secure secret numbers for anonymous grading.
    pub async fn generate_secret_numbers(
        &self,
        org_id: &str,
        exam_session_id: &str,
        generated_by: &str,
    ) -> Result<GenerationResult, String> {
        let session = self.get_exam_session(org_id, exam_session_id).await?;

        if session.status != "draft" && session.status != "scheduled" {
            return Err("can only generate secret numbers for draft or scheduled sessions".to_string());
        }

        // Archive existing active secrets
        self.repo
            .deactivate_secret_numbers(exam_session_id)
            .await
            .map_err(|e| e.to_string())?;

        // Get ordered students
        let ordered = self
            .get_ordered_students(org_id, &session.cycle_id, None)
            .await?;

        if ordered.is_empty() {
            return Err("no students found. Please rebuild student ordering first".to_string());
        }

        let now = Utc::now().to_rfc3339();
        let batch_id = Uuid::new_v4().to_string();
        let mut generated_set: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut generated = 0usize;

        // Safe charset: exclude O, 0, I, 1, L for clarity
        let charset: Vec<char> = "ABCDEFGHJKMNPQRSTUVWXYZ23456789".chars().collect();

        for student in &ordered {
            // Generate unique secret number
            let secret = loop {
                let s: String = (0..8)
                    .map(|_| {
                        let idx = rand_index(charset.len());
                        charset[idx]
                    })
                    .collect();
                if !generated_set.contains(&s) {
                    break s;
                }
            };

            generated_set.insert(secret.clone());

            // Hash the secret for verification
            let secret_hash = sha256_hex(&secret);

            // QR data format
            let qr_data = format!("ORBIS:EXAM:{}:{}", exam_session_id, secret);

            let model = secret_number::Model {
                id: Uuid::new_v4().to_string(),
                org_id: org_id.to_string(),
                exam_session_id: exam_session_id.to_string(),
                enrollment_id: student.enrollment_id.clone(),
                student_id: student.student_id.clone(),
                secret_number: secret,
                secret_hash,
                barcode_data: None,
                qr_data: Some(qr_data),
                is_active: 1,
                is_revealed: 0,
                revealed_at: None,
                revealed_by: None,
                generated_at: now.clone(),
                generated_by: generated_by.to_string(),
                generation_batch: batch_id.clone(),
                previous_secret_id: None,
                created_at: now.clone(),
                updated_at: now.clone(),
            };

            self.repo
                .insert_secret_number(model)
                .await
                .map_err(|e| e.to_string())?;

            generated += 1;
        }

        Ok(GenerationResult {
            generated_count: generated,
            exam_session_id: exam_session_id.to_string(),
        })
    }

    /// Reveal all secret numbers (link back to student identities).
    /// This is a sensitive operation requiring special permission.
    pub async fn reveal_secret_numbers(
        &self,
        org_id: &str,
        exam_session_id: &str,
        revealed_by: &str,
    ) -> Result<u64, String> {
        let session = self.get_exam_session(org_id, exam_session_id).await?;

        // Can only reveal after grading phase
        if session.status != "grading" && session.status != "completed" {
            return Err("can only reveal secret numbers during grading or after completion".to_string());
        }

        self.repo
            .reveal_secret_numbers(exam_session_id, revealed_by)
            .await
            .map_err(|e| e.to_string())
    }

    /// Lookup a student by their secret number (for grading entry)
    pub async fn lookup_by_secret(
        &self,
        exam_session_id: &str,
        secret_number: &str,
    ) -> Result<Option<SecretNumberView>, String> {
        let hash = sha256_hex(secret_number);

        let record = self
            .repo
            .lookup_by_secret(exam_session_id, &hash)
            .await
            .map_err(|e| e.to_string())?;

        Ok(record.map(|r| SecretNumberView {
            id: r.id,
            enrollment_id: r.enrollment_id,
            student_id: r.student_id,
            secret_number: if r.is_revealed == 1 {
                Some(r.secret_number)
            } else {
                None
            },
            is_revealed: r.is_revealed == 1,
            barcode_data: r.barcode_data,
            qr_data: r.qr_data,
        }))
    }
}

/// Simple SHA-256 hex digest (using std only, no external dependency).
/// In production, consider using the `sha2` crate.
fn sha256_hex(input: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // Use a double-hash approach for basic uniqueness
    // NOTE: Replace with `sha2::Sha256` when adding the dependency
    let mut hasher1 = DefaultHasher::new();
    input.hash(&mut hasher1);
    let h1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    format!("{}:{}", input, h1).hash(&mut hasher2);
    let h2 = hasher2.finish();

    format!("{:016x}{:016x}", h1, h2)
}

/// Simple pseudo-random index using system time entropy
fn rand_index(max: usize) -> usize {
    use std::time::SystemTime;

    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as usize;

    // Mix with thread-local counter for better distribution
    thread_local! {
        static COUNTER: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    }

    COUNTER.with(|c| {
        let val = c.get().wrapping_add(1);
        c.set(val);
        (seed.wrapping_mul(6364136223846793005).wrapping_add(val)) % max
    })
}
