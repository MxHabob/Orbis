use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, Statement, ActiveModelTrait, ActiveValue::Set};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Structs & Types ──────────────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GradeParams {
    pub enrollment_id: Option<String>,
    pub cycle_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubjectGradeSummary {
    pub subject_name: String,
    pub subject_name_ar: Option<String>,
    pub final_score: f64,
    pub max_score: f64,
    pub percentage: f64,
    pub grade_letter: String,
    pub grade_points: f64,
    pub passed: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GradeReportCard {
    pub enrollment_id: String,
    pub student_id: String,
    pub student_name: String,
    pub cycle_name: String,
    pub subjects: Vec<SubjectGradeSummary>,
    pub overall_percentage: f64,
    pub overall_gpa: f64,
    pub overall_letter: String,
    pub total_subjects: usize,
    pub passed_subjects: usize,
    pub failed_subjects: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GradeDistribution {
    pub excellent: usize,
    pub very_good: usize,
    pub good: usize,
    pub acceptable: usize,
    pub weak: usize,
    pub fail: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GradeAnalytics {
    pub class_average: f64,
    pub class_median: f64,
    pub class_min: f64,
    pub class_max: f64,
    pub pass_count: usize,
    pub fail_count: usize,
    pub pass_rate: f64,
    pub grade_distribution: GradeDistribution,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GradeCell {
    pub grade_id: Option<String>,
    pub assessment_id: Option<String>,
    pub exam_type: String,
    pub score: Option<f64>,
    pub max_score: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StudentGradeRow {
    pub enrollment_id: String,
    pub student_id: String,
    pub student_name: String,
    pub student_name_ar: Option<String>,
    pub grades: Vec<GradeCell>,
    pub total: f64,
    pub max_total: f64,
    pub percentage: f64,
    pub grade_letter: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GradeMatrixResponse {
    pub cycle_id: String,
    pub class_subject_id: String,
    pub students: Vec<StudentGradeRow>,
    pub assessment_types: Vec<String>,
}

// ── Commands ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_grades(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: GradeParams,
) -> CmdResult<Vec<domain_assessment::entities::grade::Model>> {
    let db = state.active_db().await;
    let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(db.clone()));
    let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(db));
    let service = domain_assessment::services::grade::GradeService::new(cmd, query);
    
    let domain_params = domain_assessment::params::GradeParams {
        enrollment_id: params.enrollment_id,
        cycle_id: params.cycle_id,
        class_id: None,
        subject_name: None,
        exam_type: None,
        semester: None,
        is_final: None,
        assessment_id: None,
        class_subject_id: None,
        limit: None,
        offset: None,
    };

    service.get_grades(&org_id, domain_params)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn submit_grade(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    graded_by: String,
    request: domain_assessment::services::grade::CreateGradeRequest,
) -> CmdResult<domain_assessment::entities::grade::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(db.clone()));
    let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(db));
    let service = domain_assessment::services::grade::GradeService::new(cmd, query);

    service.submit_grade(&org_id, &graded_by, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn submit_grades_bulk(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    graded_by: String,
    request: domain_assessment::services::grade::BulkGradeRequest,
) -> CmdResult<Vec<domain_assessment::entities::grade::Model>> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(db.clone()));
    let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(db));
    let service = domain_assessment::services::grade::GradeService::new(cmd, query);

    service.submit_grades_bulk(&org_id, &graded_by, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn update_grade(
    state: State<'_, Arc<AppState>>,
    grade_id: String,
    request: domain_assessment::services::grade::UpdateGradeRequest,
) -> CmdResult<domain_assessment::entities::grade::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(db.clone()));
    let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(db));
    let service = domain_assessment::services::grade::GradeService::new(cmd, query);

    service.update_grade(&grade_id, "admin", request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_report_card(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
) -> CmdResult<GradeReportCard> {
    let db = state.active_db().await;
    
    // Find enrollment
    let enrollment = domain_academic::entities::enrollment::Entity::find_by_id(&enrollment_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Enrollment not found".to_string()))?;
        
    let student = domain_iam::entities::user::Entity::find_by_id(&enrollment.student_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
        
    // Fetch grades
    let grades = domain_assessment::entities::grade::Entity::find()
        .filter(domain_assessment::entities::grade::Column::EnrollmentId.eq(&enrollment_id))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
        
    // Group by subject
    let mut subjects_map = std::collections::HashMap::new();
    for g in &grades {
        let entry = subjects_map.entry(g.subject_name.clone()).or_insert_with(|| {
            (0.0, 0.0, g.subject_name_ar.clone())
        });
        entry.0 += g.score;
        entry.1 += g.max_score;
    }
    
    let mut summaries = Vec::new();
    let mut total_pct = 0.0;
    let mut passed = 0;
    let mut failed = 0;
    
    for (name, (score, max, name_ar)) in subjects_map {
        let pct = if max > 0.0 { (score / max) * 100.0 } else { 0.0 };
        total_pct += pct;
        let is_passed = pct >= 50.0;
        if is_passed { passed += 1; } else { failed += 1; }
        
        let letter = if pct >= 90.0 { " ممتاز " }
                     else if pct >= 80.0 { " جيد جداً " }
                     else if pct >= 70.0 { " جيد " }
                     else if pct >= 60.0 { " مقبول " }
                     else if pct >= 50.0 { " مقبول ضعيف " }
                     else { " راسب " }.to_string();
                    
        summaries.push(SubjectGradeSummary {
            subject_name: name,
            subject_name_ar: name_ar,
            final_score: score,
            max_score: max,
            percentage: pct,
            grade_letter: letter,
            grade_points: (pct / 20.0).max(1.0),
            passed: is_passed,
        });
    }
    
    let total_subjects = summaries.len();
    let overall_pct = if total_subjects > 0 { total_pct / total_subjects as f64 } else { 0.0 };
    let overall_letter = if overall_pct >= 90.0 { " ممتاز " }
                        else if overall_pct >= 80.0 { " جيد جداً " }
                        else if overall_pct >= 70.0 { " جيد " }
                        else if overall_pct >= 60.0 { " مقبول " }
                        else if overall_pct >= 50.0 { " مقبول ضعيف " }
                        else { " راسب " }.to_string();
   
    Ok(GradeReportCard {
        enrollment_id,
        student_id: enrollment.student_id,
        student_name: student.map(|s| s.full_name).unwrap_or_else(|| "Unknown Student".to_string()),
        cycle_name: "Current Term".to_string(),
        subjects: summaries,
        overall_percentage: overall_pct,
        overall_gpa: (overall_pct / 20.0).max(1.0),
        overall_letter,
        total_subjects,
        passed_subjects: passed,
        failed_subjects: failed,
    })
}

#[tauri::command]
pub async fn get_grade_analytics(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: GradeParams,
) -> CmdResult<GradeAnalytics> {
    let db = state.active_db().await;
    let mut query = domain_assessment::entities::grade::Entity::find()
        .filter(domain_assessment::entities::grade::Column::OrgId.eq(org_id));
    if let Some(enrollment_id) = params.enrollment_id {
        query = query.filter(domain_assessment::entities::grade::Column::EnrollmentId.eq(enrollment_id));
    }
    if let Some(cycle_id) = params.cycle_id {
        query = query.filter(domain_assessment::entities::grade::Column::CycleId.eq(cycle_id));
    }
    let grades = query.all(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    
    if grades.is_empty() {
        return Ok(GradeAnalytics {
            class_average: 0.0,
            class_median: 0.0,
            class_min: 0.0,
            class_max: 0.0,
            pass_count: 0,
            fail_count: 0,
            pass_rate: 0.0,
            grade_distribution: GradeDistribution {
                excellent: 0, very_good: 0, good: 0, acceptable: 0, weak: 0, fail: 0
            }
        });
    }
    
    let mut scores: Vec<f64> = grades.iter().map(|g| (g.score / g.max_score) * 100.0).collect();
    scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let sum: f64 = scores.iter().sum();
    let avg = sum / scores.len() as f64;
    let min = *scores.first().unwrap_or(&0.0);
    let max = *scores.last().unwrap_or(&0.0);
    let median = if scores.len() % 2 == 1 {
        scores[scores.len() / 2]
    } else {
        (scores[scores.len() / 2 - 1] + scores[scores.len() / 2]) / 2.0
    };
    
    let mut excellent = 0;
    let mut very_good = 0;
    let mut good = 0;
    let mut acceptable = 0;
    let mut weak = 0;
    let mut fail = 0;
    
    for &pct in &scores {
        if pct >= 90.0 { excellent += 1; }
        else if pct >= 80.0 { very_good += 1; }
        else if pct >= 70.0 { good += 1; }
        else if pct >= 60.0 { acceptable += 1; }
        else if pct >= 50.0 { weak += 1; }
        else { fail += 1; }
    }
    
    let pass_count = excellent + very_good + good + acceptable + weak;
    let pass_rate = (pass_count as f64 / scores.len() as f64) * 100.0;
    
    Ok(GradeAnalytics {
        class_average: avg,
        class_median: median,
        class_min: min,
        class_max: max,
        pass_count,
        fail_count: fail,
        pass_rate,
        grade_distribution: GradeDistribution {
            excellent, very_good, good, acceptable, weak, fail
        }
    })
}

#[tauri::command]
pub async fn bulk_subject_grade_entry(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    graded_by: String,
    request: domain_assessment::services::grade::BulkSubjectGradeEntry,
) -> CmdResult<domain_assessment::services::grade::BulkSubjectGradeResult> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(db.clone()));
    let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(db));
    let service = domain_assessment::services::grade::GradeService::new(cmd, query);

    service.bulk_subject_grade_entry(&org_id, &graded_by, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_grade_matrix(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
    class_id: String,
    subject_id: String,
) -> CmdResult<GradeMatrixResponse> {
    let db = state.active_db().await;
    
    // Fetch all enrollments in the class
    let enrollments = domain_academic::entities::enrollment::Entity::find()
        .filter(domain_academic::entities::enrollment::Column::ClassId.eq(class_id))
        .filter(domain_academic::entities::enrollment::Column::Status.eq("active"))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
        
    let mut students = Vec::new();
    let mut assessment_types = std::collections::HashSet::new();
    
    for e in enrollments {
        let student = domain_iam::entities::user::Entity::find_by_id(&e.student_id)
            .one(&db)
            .await
            .map_err(|e| CommandError::new(e.to_string()))?;
            
        let student_name = student.as_ref().map(|s| s.full_name.clone()).unwrap_or_else(|| "Unknown Student".to_string());
        let student_name_ar = student.as_ref().and_then(|s| s.name_ar.clone());
        
        let grades = domain_assessment::entities::grade::Entity::find()
            .filter(domain_assessment::entities::grade::Column::EnrollmentId.eq(&e.id))
            .all(&db)
            .await
            .map_err(|e| CommandError::new(e.to_string()))?;
            
        let mut cells = Vec::new();
        let mut total = 0.0;
        let mut max_total = 0.0;
        
        for g in grades {
            assessment_types.insert(g.exam_type.clone());
            cells.push(GradeCell {
                grade_id: Some(g.id),
                assessment_id: g.assessment_id,
                exam_type: g.exam_type,
                score: Some(g.score),
                max_score: g.max_score,
            });
            total += g.score;
            max_total += g.max_score;
        }
        
        let pct = if max_total > 0.0 { (total / max_total) * 100.0 } else { 0.0 };
        let letter = if pct >= 90.0 { " ممتاز " }
                     else if pct >= 80.0 { " جيد جداً " }
                     else if pct >= 70.0 { " جيد " }
                     else if pct >= 60.0 { " مقبول " }
                     else if pct >= 50.0 { " ضعيف " }
                     else { " راسب " }.to_string();
                    
        students.push(StudentGradeRow {
            enrollment_id: e.id,
            student_id: e.student_id,
            student_name,
            student_name_ar,
            grades: cells,
            total,
            max_total,
            percentage: pct,
            grade_letter: letter,
        });
    }
    
    Ok(GradeMatrixResponse {
        cycle_id,
        class_subject_id: subject_id,
        students,
        assessment_types: assessment_types.into_iter().collect(),
    })
}

#[tauri::command]
pub async fn compile_semester_grades(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    _semester_num: u32,
    _graded_by: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    use sea_orm::ConnectionTrait;
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "UPDATE grades SET is_final = 1 WHERE org_id = ?",
        vec![org_id.into()]
    ))
    .await
    .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn upsert_grade_cell(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    graded_by: String,
    request: domain_assessment::services::grade::UpsertCellRequest,
) -> CmdResult<domain_assessment::services::grade::UpsertCellResult> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(db.clone()));
    let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(db));
    let service = domain_assessment::services::grade::GradeService::new(cmd, query);

    service.upsert_grade_cell(&org_id, &graded_by, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}
