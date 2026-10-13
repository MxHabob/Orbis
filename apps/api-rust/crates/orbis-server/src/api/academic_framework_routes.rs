//! Configurable academic calendars, periods, course offerings, and enrollments.

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_domain::entities::{
    academic_calendar::{
        AcademicCalendar, AcademicPeriod, PeriodType,
    },
    subject::Course,
    enrollment::Enrollment,
    section_subject::SectionSubject,
};

#[derive(Serialize, ToSchema)]
#[schema(as = FrameworkAcademicCalendarDto)]
pub struct AcademicCalendarDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
#[schema(as = FrameworkPeriodTypeDto)]
pub struct PeriodTypeDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
#[schema(as = FrameworkAcademicPeriodDto)]
pub struct AcademicPeriodDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub calendar_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub period_type_id: Uuid,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
pub struct CourseDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub credit_units: Option<Decimal>,
    pub workload_units: Option<Decimal>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
#[schema(as = FrameworkSectionSubjectDto)]
pub struct SectionSubjectDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub course_id: Uuid,
    pub period_id: Option<Uuid>,
    pub section_code: Option<String>,
    pub name: Option<String>,
    pub status: String,
    pub capacity: Option<u32>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
pub struct EnrollmentDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub offering_id: Uuid,
    pub student_id: Uuid,
    pub attempt_number: u32,
    pub status: String,
    pub enrolled_at: DateTime<Utc>,
    pub withdrawn_at: Option<DateTime<Utc>>,
    pub withdrawal_reason: Option<String>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateAcademicCalendarRequest {
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreatePeriodTypeRequest {
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateAcademicPeriodRequest {
    pub parent_id: Option<Uuid>,
    pub period_type_id: Uuid,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub sort_order: Option<i32>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateCourseRequest {
    pub unit_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub credit_units: Option<Decimal>,
    pub workload_units: Option<Decimal>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSectionSubjectRequest {
    pub course_id: Uuid,
    pub period_id: Option<Uuid>,
    pub section_code: Option<String>,
    pub name: Option<String>,
    pub capacity: Option<u32>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateEnrollmentRequest {
    pub student_id: Uuid,
    pub attempt_number: u32,
}

#[derive(Deserialize)]
pub struct SectionSubjectQuery {
    pub course_id: Option<Uuid>,
}

fn calendar_dto(item: AcademicCalendar) -> AcademicCalendarDto {
    AcademicCalendarDto {
        id: item.id,
        org_id: item.org_id,
        code: item.code,
        name: item.name,
        name_ar: item.name_ar,
        starts_at: item.starts_at,
        ends_at: item.ends_at,
        status: item.status.to_string(),
        created_at: item.created_at,
        updated_at: item.updated_at,
    }
}

fn period_type_dto(item: PeriodType) -> PeriodTypeDto {
    PeriodTypeDto {
        id: item.id,
        org_id: item.org_id,
        code: item.code,
        name: item.name,
        name_ar: item.name_ar,
        sort_order: item.sort_order,
        created_at: item.created_at,
        updated_at: item.updated_at,
    }
}

fn calendar_period_dto(item: AcademicPeriod) -> AcademicPeriodDto {
    AcademicPeriodDto {
        id: item.id,
        org_id: item.org_id,
        calendar_id: item.calendar_id,
        parent_id: item.parent_id,
        period_type_id: item.period_type_id,
        code: item.code,
        name: item.name,
        name_ar: item.name_ar,
        starts_at: item.starts_at,
        ends_at: item.ends_at,
        sort_order: item.sort_order,
        created_at: item.created_at,
        updated_at: item.updated_at,
    }
}

fn course_dto(item: Course) -> CourseDto {
    CourseDto {
        id: item.id,
        org_id: item.org_id,
        unit_id: item.unit_id,
        code: item.code,
        name: item.name,
        name_ar: item.name_ar,
        description: item.description,
        credit_units: item.credit_units,
        workload_units: item.workload_units,
        is_active: item.is_active,
        created_at: item.created_at,
        updated_at: item.updated_at,
    }
}

fn offering_dto(item: SectionSubject) -> SectionSubjectDto {
    SectionSubjectDto {
        id: item.id,
        org_id: item.org_id,
        course_id: item.subject_id,
        period_id: Some(item.period_id),
        section_code: Some(item.section_id.to_string()),
        name: None,
        status: item.status,
        capacity: item.capacity.map(|c| c as u32),
        starts_at: None,
        ends_at: None,
        created_at: item.created_at.unwrap_or_else(Utc::now),
        updated_at: item.updated_at.unwrap_or_else(Utc::now),
    }
}

fn enrollment_dto(item: Enrollment) -> EnrollmentDto {
    EnrollmentDto {
        id: item.id,
        org_id: item.org_id,
        offering_id: item.section_id.unwrap_or(item.id),
        student_id: item.student_id,
        attempt_number: item.attempt_number as u32,
        status: item.status.to_string(),
        enrolled_at: item.enrolled_at,
        withdrawn_at: item.withdrawn_at,
        withdrawal_reason: item.withdrawal_reason,
        created_by: item.created_by,
        updated_by: item.updated_by,
        created_at: item.created_at,
        updated_at: item.updated_at,
    }
}

#[utoipa::path(get, path = "/api/v1/academic-framework/calendars",
    responses((status = 200, body = Vec<AcademicCalendarDto>)), security(("bearer" = [])), tag = "Calendar")]
pub async fn list_calendars(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<AcademicCalendarDto>>>, ApiError> {
    let items = state
        .academic_framework_service
        .list_calendars(&ctx, ctx.tenant_id)
        .await?;
    Ok(ApiResponse::ok(items.into_iter().map(calendar_dto).collect()))
}

#[utoipa::path(post, path = "/api/v1/academic-framework/calendars", request_body = CreateAcademicCalendarRequest,
    responses((status = 200, body = AcademicCalendarDto)), security(("bearer" = [])), tag = "Calendar")]
pub async fn create_calendar(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(request): Json<CreateAcademicCalendarRequest>,
) -> Result<Json<ApiResponse<AcademicCalendarDto>>, ApiError> {
    let item = state
        .academic_framework_service
        .create_calendar(
            &ctx,
            ctx.tenant_id,
            request.code,
            request.name,
            request.name_ar,
            request.starts_at,
            request.ends_at,
        )
        .await?;
    Ok(ApiResponse::ok(calendar_dto(item)))
}

#[utoipa::path(get, path = "/api/v1/academic-framework/period-types",
    responses((status = 200, body = Vec<PeriodTypeDto>)), security(("bearer" = [])), tag = "Calendar")]
pub async fn list_period_types(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<PeriodTypeDto>>>, ApiError> {
    let items = state
        .academic_framework_service
        .list_period_types(&ctx, ctx.tenant_id)
        .await?;
    Ok(ApiResponse::ok(items.into_iter().map(period_type_dto).collect()))
}

#[utoipa::path(post, path = "/api/v1/academic-framework/period-types", request_body = CreatePeriodTypeRequest,
    responses((status = 200, body = PeriodTypeDto)), security(("bearer" = [])), tag = "Calendar")]
pub async fn create_period_type(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(request): Json<CreatePeriodTypeRequest>,
) -> Result<Json<ApiResponse<PeriodTypeDto>>, ApiError> {
    let item = state
        .academic_framework_service
        .create_period_type(
            &ctx,
            ctx.tenant_id,
            request.code,
            request.name,
            request.name_ar,
            request.sort_order.unwrap_or_default(),
        )
        .await?;
    Ok(ApiResponse::ok(period_type_dto(item)))
}

#[utoipa::path(get, path = "/api/v1/academic-framework/calendars/{calendar_id}/periods",
    params(("calendar_id" = Uuid, Path, description = "Academic calendar ID")),
    responses((status = 200, body = Vec<AcademicPeriodDto>)), security(("bearer" = [])), tag = "Calendar")]
pub async fn list_calendar_periods(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(calendar_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<AcademicPeriodDto>>>, ApiError> {
    let items = state
        .academic_framework_service
        .list_calendar_periods(&ctx, ctx.tenant_id, calendar_id)
        .await?;
    Ok(ApiResponse::ok(
        items.into_iter().map(calendar_period_dto).collect(),
    ))
}

#[utoipa::path(post, path = "/api/v1/academic-framework/calendars/{calendar_id}/periods", request_body = CreateAcademicPeriodRequest,
    params(("calendar_id" = Uuid, Path, description = "Academic calendar ID")),
    responses((status = 200, body = AcademicPeriodDto)), security(("bearer" = [])), tag = "Calendar")]
pub async fn create_calendar_period(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(calendar_id): Path<Uuid>,
    Json(request): Json<CreateAcademicPeriodRequest>,
) -> Result<Json<ApiResponse<AcademicPeriodDto>>, ApiError> {
    let item = state
        .academic_framework_service
        .create_calendar_period(
            &ctx,
            ctx.tenant_id,
            calendar_id,
            request.parent_id,
            request.period_type_id,
            request.code,
            request.name,
            request.name_ar,
            request.starts_at,
            request.ends_at,
            request.sort_order.unwrap_or_default(),
        )
        .await?;
    Ok(ApiResponse::ok(calendar_period_dto(item)))
}

#[utoipa::path(get, path = "/api/v1/academic-framework/courses",
    responses((status = 200, body = Vec<CourseDto>)), security(("bearer" = [])), tag = "Academic")]
pub async fn list_courses(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<CourseDto>>>, ApiError> {
    let items = state
        .academic_framework_service
        .list_courses(&ctx, ctx.tenant_id)
        .await?;
    Ok(ApiResponse::ok(items.into_iter().map(course_dto).collect()))
}

#[utoipa::path(post, path = "/api/v1/academic-framework/courses", request_body = CreateCourseRequest,
    responses((status = 200, body = CourseDto)), security(("bearer" = [])), tag = "Academic")]
pub async fn create_course(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(request): Json<CreateCourseRequest>,
) -> Result<Json<ApiResponse<CourseDto>>, ApiError> {
    let item = state
        .academic_framework_service
        .create_course(
            &ctx,
            ctx.tenant_id,
            request.unit_id,
            request.code,
            request.name,
            request.name_ar,
            request.description,
            request.credit_units,
            request.workload_units,
        )
        .await?;
    Ok(ApiResponse::ok(course_dto(item)))
}

#[utoipa::path(get, path = "/api/v1/academic-framework/course-offerings",
    params(("course_id" = Option<Uuid>, Query, description = "Optional course ID")),
    responses((status = 200, body = Vec<SectionSubjectDto>)), security(("bearer" = [])), tag = "Academic")]
pub async fn list_section_subjects(
    ctx: AuthContext,
    State(state): State<AppState>,
    Query(query): Query<SectionSubjectQuery>,
) -> Result<Json<ApiResponse<Vec<SectionSubjectDto>>>, ApiError> {
    let items = state
        .academic_framework_service
        .list_section_subjects(&ctx, ctx.tenant_id, query.course_id)
        .await?;
    Ok(ApiResponse::ok(items.into_iter().map(offering_dto).collect()))
}

#[utoipa::path(post, path = "/api/v1/academic-framework/course-offerings", request_body = CreateSectionSubjectRequest,
    responses((status = 200, body = SectionSubjectDto)), security(("bearer" = [])), tag = "Academic")]
pub async fn create_section_subject(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(request): Json<CreateSectionSubjectRequest>,
) -> Result<Json<ApiResponse<SectionSubjectDto>>, ApiError> {
    let item = state
        .academic_framework_service
        .create_section_subject(
            &ctx,
            ctx.tenant_id,
            request.course_id,
            request.period_id,
            request.section_code,
            request.name,
            request.capacity,
            request.starts_at,
            request.ends_at,
        )
        .await?;
    Ok(ApiResponse::ok(offering_dto(item)))
}

#[utoipa::path(get, path = "/api/v1/academic-framework/course-offerings/{offering_id}/enrollments",
    params(("offering_id" = Uuid, Path, description = "Course offering ID")),
    responses((status = 200, body = Vec<EnrollmentDto>)), security(("bearer" = [])), tag = "Student")]
pub async fn list_enrollments(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(offering_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<EnrollmentDto>>>, ApiError> {
    let items = state
        .academic_framework_service
        .list_enrollments(&ctx, ctx.tenant_id, offering_id)
        .await?;
    Ok(ApiResponse::ok(
        items.into_iter().map(enrollment_dto).collect(),
    ))
}

#[utoipa::path(post, path = "/api/v1/academic-framework/course-offerings/{offering_id}/enrollments", request_body = CreateEnrollmentRequest,
    params(("offering_id" = Uuid, Path, description = "Course offering ID")),
    responses((status = 200, body = EnrollmentDto)), security(("bearer" = [])), tag = "Student")]
pub async fn enroll_student(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(offering_id): Path<Uuid>,
    Json(request): Json<CreateEnrollmentRequest>,
) -> Result<Json<ApiResponse<EnrollmentDto>>, ApiError> {
    let item = state
        .academic_framework_service
        .enroll_student(
            &ctx,
            ctx.tenant_id,
            offering_id,
            request.student_id,
            request.attempt_number,
        )
        .await?;
    Ok(ApiResponse::ok(enrollment_dto(item)))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/academic-framework/calendars",
            get(list_calendars).post(create_calendar),
        )
        .route(
            "/api/v1/academic-framework/period-types",
            get(list_period_types).post(create_period_type),
        )
        .route(
            "/api/v1/academic-framework/calendars/:calendar_id/periods",
            get(list_calendar_periods).post(create_calendar_period),
        )
        .route(
            "/api/v1/academic-framework/courses",
            get(list_courses).post(create_course),
        )
        .route(
            "/api/v1/academic-framework/course-offerings",
            get(list_section_subjects).post(create_section_subject),
        )
        .route(
            "/api/v1/academic-framework/course-offerings/:offering_id/enrollments",
            get(list_enrollments).post(enroll_student),
        )
}
