use crate::db::Db;
use crate::models::{Game, GameEvent, Player};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use uuid::Uuid;

pub struct AppState {
    _rooms: Mutex<HashMap<Uuid, RoomState>>,
    db: Db,
}

#[derive(Debug)]
struct RoomState {
    _users: HashSet<Uuid>,
    _tx: broadcast::Sender<GameEvent>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            _rooms: Mutex::new(HashMap::new()),
            db: Db::new(pool),
        }
    }
}

pub fn app(pool: PgPool) -> Router {
    let state = AppState::new(pool);
    Router::new()
        .route("/health_check", get(health_check))
        .route("/game/bot/:user_id", get(create_bot_game))
        .with_state(Arc::new(state))
}

async fn health_check(State(_state): State<Arc<AppState>>) -> StatusCode {
    tracing::info!("Health check passed.");
    StatusCode::OK
}

async fn create_bot_game(
    Path(user_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<String, StatusCode> {
    let room_id = Uuid::new_v4();
    let game_id = Uuid::new_v4();
    let mut game = Game::new(game_id, Player::X);
    game.x = Some(user_id);
    game.o = Some(Uuid::nil());
    println!("Create game");
    state.db.new_game(&game, room_id).await.map_err(|error| {
        tracing::error!(?error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(room_id.to_string())
}
