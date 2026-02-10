mod api;
mod automaton;
mod errors;
mod presets;
mod simulation;
mod world;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    api::{control, world as world_api, AppState},
    automaton::GameOfLife,
    presets::Preset,
    simulation::Simulation,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cellular_automata_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("бек встает..");

    let simulation = Simulation::new();

    let preset = Preset::glider();
    let world = preset.to_world();
    simulation.set_world(world);

    info!("Loaded initial preset: {}", preset.name);

    let (command_tx, command_rx) = std::sync::mpsc::channel();


    let rule = Arc::new(GameOfLife::new());
    let sim_clone = simulation.clone();
    let _sim_handle = sim_clone.run(rule, command_rx);

    info!("Simulation thread started");

    let app_state = AppState::new(simulation, command_tx);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        // Control API
        .route("/api/control/start", post(control::start_simulation))
        .route("/api/control/stop", post(control::stop_simulation))
        .route("/api/control/pause", post(control::pause_simulation))
        .route("/api/control/resume", post(control::resume_simulation))
        .route("/api/control/step", post(control::step_simulation))
        .route("/api/control/speed", post(control::set_speed))
        .route("/api/control/status", get(control::get_status))
        // World API
        .route("/api/world/region", get(world_api::get_region))
        .route("/api/world/all", get(world_api::get_all_cells))
        .route("/api/world/cell", post(world_api::set_cell))
        .route("/api/world/presets", get(world_api::get_presets))
        .route("/api/world/preset", post(world_api::load_preset))
        .route("/api/world/clear", post(world_api::clear_world))

        .with_state(app_state)
        .layer(cors);

    let addr = "127.0.0.1:3000";
    info!("Server listening on http://{}", addr);
    info!("API Documentation:");
    info!("   GET  /                     - Root endpoint");
    info!("   GET  /health               - Health check");
    info!("   POST /api/control/start    - Start simulation");
    info!("   POST /api/control/stop     - Stop simulation");
    info!("   POST /api/control/pause    - Pause simulation");
    info!("   POST /api/control/resume   - Resume simulation");
    info!("   POST /api/control/step     - Execute one step");
    info!("   POST /api/control/speed    - Set speed (TPS)");
    info!("   GET  /api/control/status   - Get simulation status");
    info!("   GET  /api/world/region     - Get cells in region");
    info!("   GET  /api/world/all        - Get all active cells");
    info!("   POST /api/world/cell       - Set cell state");
    info!("   GET  /api/world/presets    - List all presets");
    info!("   POST /api/world/preset     - Load preset");
    info!("   POST /api/world/clear      - Clear world");

    // Запускаем сервер
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Cellular Automata Backend API\n\nVisit /health for health check\nAPI endpoints available at /api/*"
}

async fn health_handler() -> &'static str {
    "OK"
}
