use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use diesel::r2d2::PoolError;
use serde_json::json;

pub enum ActionError {
    BadRequest,
    InternalServerError,
}

impl IntoResponse for ActionError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ActionError::BadRequest => (StatusCode::BAD_REQUEST, ""),
            ActionError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, ""),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

impl From<PoolError> for ActionError {
    fn from(_value: PoolError) -> Self {
        ActionError::InternalServerError
    }
}
