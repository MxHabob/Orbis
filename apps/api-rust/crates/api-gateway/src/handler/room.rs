use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::middleware::AuthenticatedUser;
use crate::service::Service;
use crate::service::room::{CreateRoomRequest, UpdateRoomRequest};

#[utoipa::path(get, path = "/api/schedule/rooms", responses((status = 200)))]
pub async fn get_rooms(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ScheduleRead,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let rooms = service.get_rooms(&user.org_id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "error": e })),
        )
    })?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": rooms }))))
}

#[utoipa::path(post, path = "/api/schedule/rooms", responses((status = 201)))]
pub async fn post_rooms(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateRoomRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ScheduleWrite,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let room = service.create_room(&user.org_id, req).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "error": e })),
        )
    })?;

    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": room }))))
}

#[utoipa::path(put, path = "/api/schedule/rooms/{id}", responses((status = 200)))]
pub async fn put_room(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<UpdateRoomRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ScheduleWrite,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let room = service.update_room(&user.org_id, &id, req).await.map_err(|e| {
        let status = if e.contains("not found") {
            StatusCode::NOT_FOUND
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };
        (status, Json(json!({ "success": false, "error": e })))
    })?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": room }))))
}

#[utoipa::path(delete, path = "/api/schedule/rooms/{id}", responses((status = 200)))]
pub async fn delete_room(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ScheduleDelete,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    service.delete_room(&user.org_id, &id).await.map_err(|e| {
        let status = if e.contains("not found") {
            StatusCode::NOT_FOUND
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };
        (status, Json(json!({ "success": false, "error": e })))
    })?;

    Ok((
        StatusCode::OK,
        Json(json!({ "success": true, "message": "room deactivated" })),
    ))
}
