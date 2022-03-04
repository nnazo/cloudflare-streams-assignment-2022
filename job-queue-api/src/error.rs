use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// A custom response error enum for various queueing errors
#[derive(Debug, Clone)]
pub enum QueueError {
    JobNotFound,
    JobFinished,
    JobNotStarted,
    QueuePoisoned,
    Empty,
}

impl IntoResponse for QueueError {
    /// Convert the queue error into a response type
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            QueueError::JobNotFound => (StatusCode::NOT_FOUND, "Job not found"),
            QueueError::JobFinished => (StatusCode::BAD_REQUEST, "Job already finished"),
            QueueError::QueuePoisoned => (StatusCode::INTERNAL_SERVER_ERROR, "Poisoned queue"),
            QueueError::Empty => (StatusCode::NOT_FOUND, "Job queue empty"),
            QueueError::JobNotStarted => (
                StatusCode::BAD_REQUEST,
                "Cannot conclude a job that has not started",
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
