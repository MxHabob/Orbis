/// Common macros and utilities for thin handlers.

/// Standard JSON error response.
#[macro_export]
macro_rules! err {
    ($status:expr, $msg:expr) => {
        Err(($status, Json(json!({ "success": false, "error": $msg }))))
    };
}

/// Standard JSON success response.
#[macro_export]
macro_rules! ok {
    ($data:expr) => {
        Ok((StatusCode::OK, Json(json!({ "success": true, "data": $data }))))
    };
    ($status:expr, $data:expr) => {
        Ok(($status, Json(json!({ "success": true, "data": $data }))))
    };
}

/// Build context + call ops, returning standard JSON.
#[macro_export]
macro_rules! ops_call {
    ($state:expr, $user:expr, $body:expr) => {{
        let ctx = $state.build_context(&$user.org_id, &$user.id);
        let result = $body(&ctx).await.map_err(|e: shared_kernel::errors::AppError| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e.to_string() })))
        })?;
        ok!(result)
    }};
}
