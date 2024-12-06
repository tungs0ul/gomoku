use crate::db::Db;
use crate::models::{Game, GameEvent, GameStatus, GameType, Move, Player, PlayerStatus};
use axum::extract::ws::{CloseFrame, Message, WebSocket};
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use futures::SinkExt;
use futures::StreamExt;
use serde::Deserialize;
use sqlx::PgPool;
use std::collections::HashSet;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, Mutex};
use tower_http::cors::CorsLayer;
use uuid::Uuid;

pub struct AppState {
    rooms: Mutex<HashMap<Uuid, RoomState>>,
    db: Db,
}

#[derive(Debug)]
struct RoomState {
    users: HashSet<Uuid>,
    tx: broadcast::Sender<GameEvent>,
}

impl RoomState {
    fn new() -> Self {
        Self {
            users: HashSet::new(),
            tx: broadcast::channel(32).0,
        }
    }
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            rooms: Mutex::new(HashMap::new()),
            db: Db::new(pool),
        }
    }
}

pub fn app(pool: PgPool) -> Router {
    let state = AppState::new(pool);
    Router::new()
        //api
        .route("/api/health", get(health_check))
        .route("/api/games", post(play))
        .route("/api/rooms", get(get_rooms))
        .route("/api/users", post(random_user))
        //ws
        .route("/ws/rooms/:room_id/users/:user_id", get(websocket_handler))
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(state))
}

async fn random_user() -> String {
    Uuid::new_v4().to_string()
}

async fn get_rooms(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Game>>, StatusCode> {
    let rooms = state.rooms.lock().await;
    let rooms: Vec<Uuid> = { rooms.keys().map(|x| x.to_owned()).collect() };
    let rooms = state
        .db
        .get_active_game_for_rooms(&rooms, &[GameType::Bot, GameType::Normal])
        .await
        .map_err(|err| {
            tracing::error!(?err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(rooms))
}

async fn health_check(State(_state): State<Arc<AppState>>) -> StatusCode {
    tracing::info!("Health check passed.");
    StatusCode::OK
}

#[derive(Debug, Deserialize)]
struct GamePayload {
    user_id: Uuid,
    game_type: GameType,
}

async fn play(
    State(state): State<Arc<AppState>>,
    Json(GamePayload { game_type, user_id }): Json<GamePayload>,
) -> Result<String, StatusCode> {
    let (user_id, room_id) = match game_type {
        GameType::Bot => {
            let room_id = Uuid::new_v4();
            let mut game = Game::new(room_id, Player::X, GameType::Bot);
            game.x = Some(user_id);
            game.o = Some(Uuid::nil());
            state.db.new_game(&game).await.map_err(|error| {
                tracing::error!(?error);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            (user_id, room_id)
        }
        GameType::Normal => {
            #[allow(unused_assignments)]
            let mut room = None;
            {
                let rooms = state.rooms.lock().await;
                let rooms: Vec<Uuid> = { rooms.keys().map(|x| x.to_owned()).collect() };
                match state.db.get_available_quick_games(&rooms).await {
                    Ok(r) => room = Some(r),
                    _ => room = None,
                }
            }
            match room {
                Some(room) => {
                    let mut game = room.to_owned();
                    if game.x.is_none() {
                        game.x = Some(user_id)
                    } else if game.o.is_none() {
                        game.o = Some(user_id)
                    }
                    state.db.update_game(&game).await.map_err(|err| {
                        tracing::error!(?err);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;
                    (user_id, game.room_id)
                }
                None => {
                    let room_id: Uuid = Uuid::new_v4();
                    let mut game = Game::new(room_id, Player::X, GameType::Normal);
                    game.x = Some(user_id);
                    state.db.new_game(&game).await.map_err(|err| {
                        tracing::error!(?err);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;
                    (user_id, room_id)
                }
            }
        }
        _ => {
            todo!()
        }
    };

    Ok(format!("/ws/rooms/{room_id}/users/{user_id}"))
}

async fn websocket_handler(
    Path((room_id, user_id)): Path<(String, String)>,
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| websocket(socket, state, room_id, user_id))
}

async fn websocket(mut stream: WebSocket, state: Arc<AppState>, room_id: String, user_id: String) {
    // let (mut sender, mut receiver) = stream.split();
    let user_id = Uuid::parse_str(&user_id);
    let room_id = Uuid::parse_str(&room_id);
    if user_id.is_err() || room_id.is_err() {
        tracing::error!("Invalid user or room id");
        let _ = stream
            .send(Message::Close(Some(CloseFrame {
                code: 0,
                reason: "Invalid user or room id".into(),
            })))
            .await;
        return;
    }
    let room_id = room_id.unwrap();
    let user_id = user_id.unwrap();

    {
        let game = state.db.get_active_game_for_room(&room_id).await;
        if game.is_err() {
            tracing::error!("Game not found");
            let _ = stream
                .send(Message::Close(Some(CloseFrame {
                    code: 0,
                    reason: "Game not found".into(),
                })))
                .await;
            return;
        }
        let mut game = game.unwrap();
        if game.x == Some(user_id) {
            game.x_status = PlayerStatus::Confirmed;
        } else if game.o == Some(user_id) {
            game.o_status = PlayerStatus::Confirmed;
        }
        if let Err(error) = state.db.update_game(&game).await {
            tracing::error!(?error, "Error updating game");
        }

        {
            let rooms = state.rooms.lock().await;
            if let Some(room) = rooms.get(&room_id) {
                if let Err(error) = room.tx.send(GameEvent::Message {
                    user: None,
                    id: Uuid::new_v4(),
                    msg: format!("{user_id} has joined room"),
                }) {
                    tracing::error!(?error, "Error sending game status");
                }
            }
        }

        if matches!(game.x_status, PlayerStatus::Confirmed)
            && matches!(game.o_status, PlayerStatus::Confirmed)
            && (game.x == Some(user_id) || game.o == Some(user_id))
        {
            game.status = GameStatus::Playing;
            let rooms = state.rooms.lock().await;
            if let Some(room) = rooms.get(&room_id) {
                if let Err(error) = room.tx.send(GameEvent::Status {
                    status: GameStatus::Playing,
                }) {
                    tracing::error!(?error, "Error sending game status");
                }
            }
        }

        if let Err(error) = stream
            .send(Message::Text(
                serde_json::to_string(&GameEvent::Game {
                    game: Box::new(game),
                })
                .unwrap(),
            ))
            .await
        {
            tracing::error!(?error, "Error sending game");
        }
    }

    #[allow(unused_assignments)]
    #[allow(unused)]
    let mut tx = None;
    {
        let mut rooms = state.rooms.lock().await;
        let room = rooms.get_mut(&room_id);
        if room.is_none() {
            let room = RoomState::new();
            tx = Some(room.tx.clone());
            rooms.insert(room_id, room);
        } else {
            let room = room.unwrap();
            if room.users.contains(&user_id) {
                let _ = stream
                    .send(Message::Close(Some(CloseFrame {
                        code: 0,
                        reason: "User already in room".into(),
                    })))
                    .await;
                return;
            }
            tx = Some(room.tx.clone());
            room.users.insert(user_id);
        }
    }

    if tx.is_none() {
        let _ = stream
            .send(Message::Close(Some(CloseFrame {
                code: 0,
                reason: "No room found".into(),
            })))
            .await;
        return;
    }

    let tx = tx.unwrap();
    let mut rx = tx.subscribe();

    let sender_state = state.clone();
    let sender_tx = tx.clone();
    let (mut sender, mut receiver) = stream.split();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender
                .send(Message::Text(serde_json::to_string(&msg).unwrap()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let msg = serde_json::from_str::<GameEvent>(&text);
            if msg.is_err() {
                continue;
            }
            let msg = msg.unwrap();
            let game = sender_state.db.get_active_game_for_room(&room_id).await;
            if let Err(error) = game {
                tracing::error!(?error, "Invalid game");
                continue;
            }
            let mut game = game.unwrap();
            match msg {
                GameEvent::Message { msg, .. } => {
                    let _ = sender_tx.send(GameEvent::Message {
                        msg,
                        user: Some(user_id.to_string()),
                        id: Uuid::new_v4(),
                    });
                }
                GameEvent::MoveEvent { mv } => {
                    if mv.player == Player::O && Some(user_id) != game.o {
                        continue;
                    }
                    if mv.player == Player::X && Some(user_id) != game.x {
                        continue;
                    }
                    if game.play(&mv).is_ok() {
                        if let Err(error) = sender_state
                            .db
                            .insert_move(&game.id, &mv, game.moves.len())
                            .await
                        {
                            tracing::error!(?error, "Error inserting move");
                        }
                        if let Ok(Some(win)) = game.check_winning_move(&mv.position) {
                            game.winner = Some(win);
                            game.status = GameStatus::Ready;
                            game.x_status = PlayerStatus::Ready;
                            game.o_status = match game.game_type {
                                GameType::Bot => PlayerStatus::Confirmed,
                                GameType::Normal | GameType::Private => PlayerStatus::Ready,
                            };
                            if let Err(error) = sender_state.db.update_game(&game).await {
                                tracing::error!(?error, "Error update game winner");
                            }
                            let _ = sender_tx.send(GameEvent::Winner {
                                moves: game.winner.unwrap(),
                                last_move: mv,
                            });
                            continue;
                        }
                        if let Err(error) = sender_tx.send(GameEvent::MoveEvent { mv }) {
                            tracing::error!(?error, "Error sending move event");
                        }
                        if !matches!(game.game_type, GameType::Bot) {
                            continue;
                        }
                        let predict = game.find_bot_move(2);

                        if let Some(pos) = predict {
                            let bot_move = Move::new(Player::O, pos);
                            game.play(&bot_move).unwrap();
                            let _ = sender_tx.send(GameEvent::MoveEvent { mv: bot_move });
                            let _ = sender_state
                                .db
                                .insert_move(&game.id, &bot_move, game.moves.len())
                                .await;
                            if let Ok(Some(win)) = game.check_winning_move(&pos) {
                                game.winner = Some(win);
                                game.status = GameStatus::Ready;
                                game.x_status = PlayerStatus::Ready;
                                if let Err(error) = sender_state.db.update_game(&game).await {
                                    tracing::error!(?error, "Error update game winner");
                                }
                                let _ = sender_tx.send(GameEvent::Winner {
                                    moves: game.winner.unwrap(),
                                    last_move: bot_move,
                                });
                                continue;
                            }
                        }
                    }
                }
                GameEvent::PlayAgain => {
                    if game.o != Some(user_id) && game.x != Some(user_id) {
                        continue;
                    }
                    match game.game_type {
                        GameType::Bot => {
                            if game.x != Some(user_id) {
                                continue;
                            }
                            game.status = GameStatus::Ended;
                            let _ = sender_state.db.update_game(&game).await;
                            let next_player = game.next_player;
                            let mut game = Game::new(room_id, next_player, GameType::Bot);
                            game.x = Some(user_id);
                            game.o = Some(Uuid::nil());
                            if let Err(error) = sender_state.db.new_game(&game).await {
                                tracing::error!(?error, "Error saving game");
                            }
                            if game.next_player == Player::O {
                                let predict = game.find_bot_move(2);
                                if let Some(pos) = predict {
                                    let bot_move = Move::new(Player::O, pos);
                                    game.play(&bot_move).unwrap();
                                    let _ = sender_tx.send(GameEvent::MoveEvent { mv: bot_move });
                                    let _ = sender_state
                                        .db
                                        .insert_move(&game.id, &bot_move, game.moves.len())
                                        .await;
                                }
                            }
                            let _ = sender_tx.send(GameEvent::Game {
                                game: Box::new(game),
                            });
                        }
                        GameType::Normal | GameType::Private => {
                            if game.x == Some(user_id) {
                                game.x_status = PlayerStatus::Confirmed;
                            } else if game.o == Some(user_id) {
                                game.o_status = PlayerStatus::Confirmed;
                            }
                            if let Err(error) = sender_state.db.update_game(&game).await {
                                tracing::error!(?error, "Error update game");
                            }
                            if matches!(game.x_status, PlayerStatus::Confirmed)
                                && matches!(game.o_status, PlayerStatus::Confirmed)
                            {
                                game.status = GameStatus::Ended;
                                if let Err(error) = sender_state.db.update_game(&game).await {
                                    tracing::error!(?error, "Error update game");
                                }
                                let x_player = game.x;
                                let o_player = game.o;
                                let mut game = Game::new(room_id, game.next_player, game.game_type);
                                game.x = x_player;
                                game.o = o_player;
                                game.status = GameStatus::Playing;
                                if let Err(error) = sender_state.db.update_game(&game).await {
                                    tracing::error!(?error, "Error creating new game");
                                }
                                if let Err(error) = sender_tx.send(GameEvent::Game {
                                    game: Box::new(game),
                                }) {
                                    tracing::error!(?error, "Error sending new game");
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    {
        let game = state.db.get_active_game_for_room(&room_id).await;
        if game.is_ok() {
            let mut game = game.unwrap();
            {
                let mut rooms = state.rooms.lock().await;

                if let Some(room) = rooms.get_mut(&room_id) {
                    room.users.remove(&user_id);
                    tracing::info!(?game.x, ?game.o, ?user_id);

                    if game.x == Some(user_id) || game.o == Some(user_id) {
                        if game.x == Some(user_id) {
                            game.x_status = match game.x_status {
                                PlayerStatus::Confirmed => PlayerStatus::ConfirmedThenLeft,
                                _ => PlayerStatus::Left,
                            }
                        } else if game.o == Some(user_id) {
                            game.o_status = match game.o_status {
                                PlayerStatus::Confirmed => PlayerStatus::ConfirmedThenLeft,
                                _ => PlayerStatus::Left,
                            }
                        }
                        if let Err(error) = state.db.update_game(&game).await {
                            tracing::error!(?error, "Error updating game status");
                        }
                        if let Err(error) = room.tx.send(GameEvent::PlayerLeft) {
                            tracing::error!(?error, "Error sending player left");
                        }
                    }
                }
            }
            if matches!(
                game.x_status,
                PlayerStatus::Left | PlayerStatus::ConfirmedThenLeft
            ) && (matches!(
                game.o_status,
                PlayerStatus::Left | PlayerStatus::ConfirmedThenLeft
            ) || matches!(game.game_type, GameType::Bot))
            {
                tracing::info!("Game ended");
                game.status = GameStatus::Ended;
                if let Err(error) = state.db.update_game(&game).await {
                    tracing::error!(?error, "Error updating game status");
                }
                {
                    let mut rooms = state.rooms.lock().await;
                    if let Some(room) = rooms.remove(&room_id) {
                        if let Err(error) = room.tx.send(GameEvent::Status {
                            status: GameStatus::Ended,
                        }) {
                            tracing::error!(?error, "Error sending ended");
                        }
                    }
                }
            }
        }
    }
}
