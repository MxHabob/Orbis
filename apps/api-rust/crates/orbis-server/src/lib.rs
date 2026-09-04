pub mod api;
pub mod state;

use axum::Router;
use state::AppState;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::org_routes::create_org,
        api::org_routes::create_org_unit,
    ),
    components(
        schemas(
            api::org_routes::CreateOrgRequest,
            api::org_routes::OrgResponse,
            api::org_routes::CreateOrgUnitRequest,
            api::org_routes::OrgUnitResponse,
            api::org_routes::Organization,
            api::exam_routes::ExamSession,
            api::exam_routes::ExamSubject,
            api::academic_routes::AcademicYear,
            api::academic_routes::AcademicYearInfo,
            api::academic_routes::AcademicContextResponse,
            api::academic_routes::AcademicPeriod,
            api::iam_routes::User,
            api::student_routes::Enrollment,
            api::OrbisError,
        )
    ),
    tags(
        (name = "Organization", description = "Organization management endpoints")
    )
)]
pub struct ApiDoc;

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(api::iam_routes::routes())
        .merge(api::org_routes::routes())
        .merge(api::calendar_routes::routes())
        .merge(api::academic_routes::routes())
        .merge(api::student_routes::routes())
        .merge(api::scheduling_routes::routes())
        .merge(api::assessment_routes::routes())
        .merge(api::exam_routes::routes())
        .merge(api::finance_routes::routes())
        .merge(api::document_routes::routes())
        .merge(api::integration_routes::routes())
        .merge(api::report_routes::routes())
        .with_state(state)
}
