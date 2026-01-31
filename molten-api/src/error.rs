use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use molten_service::ServiceError;
use serde_json::json;

/// A wrapper to allow us to implement IntoResponse for foreign errors
pub struct ApiError(pub ServiceError);

// Allow ? operator to auto-convert ServiceError -> ApiError
impl From<ServiceError> for ApiError {
    fn from(inner: ServiceError) -> Self {
        ApiError(inner)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self.0 {
            // 404 Not Found
            ServiceError::FormNotFound(id) => (StatusCode::NOT_FOUND, format!("Form '{}' not found", id)),
            ServiceError::WorkflowNotFound(id) => (StatusCode::NOT_FOUND, format!("Workflow '{}' not found", id)),
            
            // 400 Bad Request (Validation)
            ServiceError::ValidationErrors(errs) => {
                // Return the detailed list of validation failures
                return (StatusCode::BAD_REQUEST, Json(json!({ "error": "Validation Failed", "details": errs }))).into_response();
            },
            ServiceError::WorkflowRuleViolation(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            
            // 500 Internal Server Error
            ServiceError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ServiceError::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}