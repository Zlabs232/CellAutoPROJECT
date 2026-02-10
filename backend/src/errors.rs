use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    PresetNotFound(String),
    CommandSendError,
    SimulationAlreadyRunning,
    SimulationNotRunning,
    InvalidRequest(String),
    InvalidTps(u32),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::PresetNotFound(name) => {
                (StatusCode::NOT_FOUND, format!("Preset not found: {}", name))
            }
            ApiError::CommandSendError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to send command to simulation".to_string(),
            ),
            ApiError::SimulationAlreadyRunning => {
                (StatusCode::CONFLICT, "Simulation is already running".to_string())
            }
            ApiError::SimulationNotRunning => {
                (StatusCode::CONFLICT, "Simulation is not running".to_string())
            }
            ApiError::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::InvalidTps(tps) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid TPS value: {}. Must be between 1 and 1000", tps),
            ),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
