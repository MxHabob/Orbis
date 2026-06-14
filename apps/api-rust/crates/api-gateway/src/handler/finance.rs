use axum::{
    extract::{State, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::json;
use crate::service::Service;
use crate::service::finance::{CreateLedgerAccountRequest, CreateTreasuryRequest, AssignStudentFeeRequest, RecordPaymentRequest};
use crate::middleware::AuthenticatedUser;

#[derive(Debug, Deserialize)]
pub struct FinanceQuery {
    pub enrollment_id: Option<String>,
}

#[utoipa::path(get, path = "/api/finance/ledger-accounts", responses((status = 200, description = "Ledger accounts list")))]
pub async fn get_ledger_accounts(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::FinanceRead, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let accounts = service.get_ledger_accounts(&user.org_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": accounts }))))
}

#[utoipa::path(post, path = "/api/finance/ledger-accounts", request_body = CreateLedgerAccountRequest, responses((status = 201, description = "Ledger account created")))]
pub async fn post_ledger_accounts(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateLedgerAccountRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::FinanceWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let account = service.create_ledger_account(&user.org_id, req).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": account }))))
}

#[utoipa::path(get, path = "/api/finance/treasuries", responses((status = 200, description = "Treasuries list")))]
pub async fn get_treasuries(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::FinanceRead, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let treasuries = service.get_treasuries(&user.org_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": treasuries }))))
}

#[utoipa::path(post, path = "/api/finance/treasuries", request_body = CreateTreasuryRequest, responses((status = 201, description = "Treasury created")))]
pub async fn post_treasuries(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateTreasuryRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::FinanceWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let treasury = service.create_treasury(&user.org_id, req).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": treasury }))))
}

#[utoipa::path(get, path = "/api/finance/student-fees", responses((status = 200, description = "Student fees list")))]
pub async fn get_student_fees(
    State(service): State<Service>,
    Query(query): Query<FinanceQuery>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::FinanceRead, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let fees = service.get_student_fees(&user.org_id, query.enrollment_id.as_deref()).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": fees }))))
}

#[utoipa::path(post, path = "/api/finance/student-fees", request_body = AssignStudentFeeRequest, responses((status = 201, description = "Student fee assigned")))]
pub async fn post_student_fees(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<AssignStudentFeeRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::FinanceWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let fee = service.assign_student_fee(&user.org_id, req).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": fee }))))
}

#[utoipa::path(get, path = "/api/finance/payments", responses((status = 200, description = "Payments list")))]
pub async fn get_payments(
    State(service): State<Service>,
    Query(query): Query<FinanceQuery>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::FinanceRead, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let payments = service.get_payments(&user.org_id, query.enrollment_id.as_deref()).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": payments }))))
}

#[utoipa::path(post, path = "/api/finance/payments", request_body = RecordPaymentRequest, responses((status = 201, description = "Payment recorded")))]
pub async fn post_payments(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<RecordPaymentRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::FinanceWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let payment = service.record_payment(&user.org_id, req, &user.id).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": payment }))))
}
