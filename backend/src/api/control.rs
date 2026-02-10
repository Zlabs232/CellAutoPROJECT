use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::{
    errors::{ApiError, ApiResult},
    simulation::{Simulation, SimulationCommand, SimulationState},
};

#[derive(Clone)]
pub struct AppState {
    pub simulation: Simulation,
    pub command_tx: Arc<Mutex<std::sync::mpsc::Sender<SimulationCommand>>>,
}

impl AppState {
    pub fn new(simulation: Simulation, command_tx: std::sync::mpsc::Sender<SimulationCommand>) -> Self {
        Self {
            simulation,
            command_tx: Arc::new(Mutex::new(command_tx)),
        }
    }

    pub fn send_command(&self, cmd: SimulationCommand) -> ApiResult<()> {
        self.command_tx
            .lock()
            .unwrap()
            .send(cmd)
            .map_err(|_| ApiError::CommandSendError)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub state: String,
    pub tick_count: u64,
    pub tps: u32,
    pub active_cells: usize,
}

#[derive(Debug, Deserialize)]
pub struct SetSpeedRequest {
    pub tps: u32,
}

pub async fn start_simulation(
    State(state): State<AppState>,
) -> ApiResult<(StatusCode, Json<StatusResponse>)> {
    let current_state = state.simulation.get_state();

    if current_state == SimulationState::Running {
        return Err(ApiError::SimulationAlreadyRunning);
    }

    state.send_command(SimulationCommand::Start)?;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    Ok((StatusCode::OK, Json(get_status_response(&state))))
}

pub async fn stop_simulation(
    State(state): State<AppState>,
) -> ApiResult<(StatusCode, Json<StatusResponse>)> {
    state.send_command(SimulationCommand::Stop)?;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    Ok((StatusCode::OK, Json(get_status_response(&state))))
}

pub async fn pause_simulation(
    State(state): State<AppState>,
) -> ApiResult<(StatusCode, Json<StatusResponse>)> {
    let current_state = state.simulation.get_state();

    if current_state != SimulationState::Running {
        return Err(ApiError::SimulationNotRunning);
    }

    state.send_command(SimulationCommand::Pause)?;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    Ok((StatusCode::OK, Json(get_status_response(&state))))
}

pub async fn resume_simulation(
    State(state): State<AppState>,
) -> ApiResult<(StatusCode, Json<StatusResponse>)> {
    let current_state = state.simulation.get_state();

    if current_state != SimulationState::Paused {
        return Err(ApiError::InvalidRequest("Simulation is not paused".to_string()));
    }

    state.send_command(SimulationCommand::Resume)?;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    Ok((StatusCode::OK, Json(get_status_response(&state))))
}

pub async fn step_simulation(
    State(state): State<AppState>,
) -> ApiResult<(StatusCode, Json<StatusResponse>)> {
    state.send_command(SimulationCommand::Step)?;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    Ok((StatusCode::OK, Json(get_status_response(&state))))
}

pub async fn set_speed(
    State(state): State<AppState>,
    Json(payload): Json<SetSpeedRequest>,
) -> ApiResult<(StatusCode, Json<StatusResponse>)> {
    if payload.tps < 1 || payload.tps > 1000 {
        return Err(ApiError::InvalidTps(payload.tps));
    }

    state.send_command(SimulationCommand::SetSpeed(payload.tps))?;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    Ok((StatusCode::OK, Json(get_status_response(&state))))
}

pub async fn get_status(
    State(state): State<AppState>,
) -> Json<StatusResponse> {
    Json(get_status_response(&state))
}

fn get_status_response(state: &AppState) -> StatusResponse {
    let sim_state = state.simulation.get_state();
    let world = state.simulation.get_world();

    StatusResponse {
        state: match sim_state {
            SimulationState::Stopped => "stopped".to_string(),
            SimulationState::Running => "running".to_string(),
            SimulationState::Paused => "paused".to_string(),
        },
        tick_count: state.simulation.get_tick_count(),
        tps: state.simulation.get_tps(),
        active_cells: world.active_cell_count(),
    }
}
