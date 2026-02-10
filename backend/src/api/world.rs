use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::control::AppState,
    errors::{ApiError, ApiResult},
    presets::Preset,
    world::Coord,
};

#[derive(Debug, Deserialize)]
pub struct GetRegionQuery {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

#[derive(Debug, Serialize)]
pub struct RegionResponse {
    pub cells: Vec<CellData>,
    pub bounds: BoundsData,
}

#[derive(Debug, Serialize)]
pub struct CellData {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize)]
pub struct BoundsData {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

#[derive(Debug, Deserialize)]
pub struct SetCellRequest {
    pub x: i32,
    pub y: i32,
    pub alive: bool,
}

#[derive(Debug, Serialize)]
pub struct SetCellResponse {
    pub x: i32,
    pub y: i32,
    pub alive: bool,
    pub active_cells: usize,
}

#[derive(Debug, Deserialize)]
pub struct LoadPresetRequest {
    pub name: String,
    #[serde(default)]
    pub offset_x: i32,
    #[serde(default)]
    pub offset_y: i32,
}

#[derive(Debug, Serialize)]
pub struct PresetsListResponse {
    pub presets: Vec<PresetInfo>,
}

#[derive(Debug, Serialize)]
pub struct PresetInfo {
    pub name: String,
    pub description: String,
    pub cell_count: usize,
}

#[derive(Debug, Serialize)]
pub struct LoadPresetResponse {
    pub preset_name: String,
    pub active_cells: usize,
}

pub async fn get_region(
    State(state): State<AppState>,
    Query(query): Query<GetRegionQuery>,
) -> Json<RegionResponse> {
    let world = state.simulation.get_world();

    let mut cells = Vec::new();

    for x in query.x1..=query.x2 {
        for y in query.y1..=query.y2 {
            let coord = Coord::new(x, y);
            if world.get_cell(coord) {
                cells.push(CellData { x, y });
            }
        }
    }

    Json(RegionResponse {
        cells,
        bounds: BoundsData {
            x1: query.x1,
            y1: query.y1,
            x2: query.x2,
            y2: query.y2,
        },
    })
}

pub async fn get_all_cells(
    State(state): State<AppState>,
) -> Json<Vec<CellData>> {
    let world = state.simulation.get_world();

    let cells: Vec<CellData> = world
        .iter_active_cells()
        .map(|coord| CellData {
            x: coord.x,
            y: coord.y,
        })
        .collect();

    Json(cells)
}

pub async fn set_cell(
    State(state): State<AppState>,
    Json(payload): Json<SetCellRequest>,
) -> ApiResult<(StatusCode, Json<SetCellResponse>)> {
    let mut world = state.simulation.get_world();
    let coord = Coord::new(payload.x, payload.y);

    world.set_cell(coord, payload.alive);
    state.simulation.set_world(world.clone());

    Ok((
        StatusCode::OK,
        Json(SetCellResponse {
            x: payload.x,
            y: payload.y,
            alive: payload.alive,
            active_cells: world.active_cell_count(),
        }),
    ))
}

pub async fn get_presets() -> Json<PresetsListResponse> {
    let presets = Preset::all();

    let presets_info: Vec<PresetInfo> = presets
        .iter()
        .map(|p| PresetInfo {
            name: p.name.clone(),
            description: p.description.clone(),
            cell_count: p.cells.len(),
        })
        .collect();

    Json(PresetsListResponse {
        presets: presets_info,
    })
}

pub async fn load_preset(
    State(state): State<AppState>,
    Json(payload): Json<LoadPresetRequest>,
) -> ApiResult<(StatusCode, Json<LoadPresetResponse>)> {
    let preset = Preset::find(&payload.name)
        .ok_or_else(|| ApiError::PresetNotFound(payload.name.clone()))?;

    let mut world = state.simulation.get_world();
    let offset = Coord::new(payload.offset_x, payload.offset_y);

    preset.load_into(&mut world, offset);
    state.simulation.set_world(world.clone());

    Ok((
        StatusCode::OK,
        Json(LoadPresetResponse {
            preset_name: preset.name,
            active_cells: world.active_cell_count(),
        }),
    ))
}

pub async fn clear_world(
    State(state): State<AppState>,
) -> ApiResult<(StatusCode, Json<SetCellResponse>)> {
    let mut world = state.simulation.get_world();
    world.clear();
    state.simulation.set_world(world);

    Ok((
        StatusCode::OK,
        Json(SetCellResponse {
            x: 0,
            y: 0,
            alive: false,
            active_cells: 0,
        }),
    ))
}
