use crate::db::Db;
use crate::models::{Game, GameEvent, Move, Player, RoomType};
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
    room_type: RoomType,
}

impl RoomState {
    fn new(room_type: RoomType) -> Self {
        Self {
            users: HashSet::new(),
            tx: broadcast::channel(32).0,
            room_type,
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
        .route("/health", get(health_check)) // Keep simple utility endpoints as-is
        .route("/games", post(create_game)) // POST to create a bot game
        .route("/ws/rooms/:room_id/users/:user_id", get(websocket_handler)) // Clarify WebSocket path
        .route("/rooms", get(get_rooms)) // Pluralize resource names
        .route("/users", post(random_user)) // Pluralize resource names for user creation
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(state))
}

async fn random_user() -> String {
    Uuid::new_v4().to_string()
}

async fn get_rooms(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<(Uuid, Game, RoomType)>>, StatusCode> {
    let rooms = state.rooms.lock().await;
    let rooms: Vec<Uuid> = { rooms.keys().map(|x| x.to_owned()).collect() };
    let rooms = state
        .db
        .get_active_game_for_rooms(&rooms)
        .await
        .map_err(|err| {
            tracing::error!(?err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_iter()
        .filter(|(_, _, room_type)| !matches!(room_type, RoomType::Private))
        .collect();
    Ok(Json(rooms))
}

async fn health_check(State(_state): State<Arc<AppState>>) -> StatusCode {
    tracing::info!("Health check passed.");
    StatusCode::OK
}

#[derive(Debug, Deserialize)]
struct GamePayload {
    user_id: Uuid,
    room_type: RoomType,
}

async fn create_game(
    State(state): State<Arc<AppState>>,
    Json(GamePayload { room_type, user_id }): Json<GamePayload>,
) -> Result<String, StatusCode> {
    let room_id = Uuid::new_v4();
    let game_id = Uuid::new_v4();
    let mut game = Game::new(game_id, Player::X);
    game.x = Some(user_id);
    game.o = Some(Uuid::nil());

    state
        .db
        .create_room(&room_id, &room_type, &game)
        .await
        .map_err(|error| {
            tracing::error!(?error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

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
    tracing::info!("Connecting");
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
        let game = game.unwrap();
        let _ = stream
            .send(Message::Text(
                serde_json::to_string(&GameEvent::Game {
                    game: Box::new(game),
                })
                .unwrap(),
            ))
            .await;
    }

    #[allow(unused_assignments)]
    let mut game_type = RoomType::Normal;
    #[allow(unused)]
    let mut tx = None;
    {
        let mut rooms = state.rooms.lock().await;
        let room = rooms.get_mut(&room_id);
        if room.is_none() {
            let room_type = state.db.get_room_type(&room_id).await;
            if room_type.is_err() {
                tracing::error!("Room not found");
                let _ = stream
                    .send(Message::Close(Some(CloseFrame {
                        code: 0,
                        reason: "Room not found".into(),
                    })))
                    .await;
                return;
            }
            let room_type = room_type.unwrap();
            game_type = room_type.clone();
            let room = RoomState::new(room_type);
            tx = Some(room.tx.clone());
            rooms.insert(room_id, room);
        } else {
            let room = room.unwrap();
            game_type = room.room_type.clone();
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
            if game.is_err() {
                tracing::error!("Invalid game");
                continue;
            }
            let mut game = game.unwrap();
            match msg {
                GameEvent::Chat { msg, .. } => {
                    let _ = sender_tx.send(GameEvent::Chat {
                        msg,
                        user: user_id.to_string(),
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
                        let _ = sender_state
                            .db
                            .insert_move(&game.id, &mv, game.moves.len())
                            .await;
                        if let Ok(Some(win)) = game.check_winning_move(&mv.position) {
                            game.winner = Some(win);
                            if let Err(error) = sender_state.db.update_game_winner(&game).await {
                                tracing::error!(?error, "Error update game winner");
                            }
                            let _ = sender_tx.send(GameEvent::Winner {
                                moves: game.winner.unwrap(),
                                last_move: mv,
                            });
                            continue;
                        }
                        let _ = sender_tx.send(GameEvent::MoveEvent { mv });
                    }
                }
                GameEvent::PlayAgain => {
                    if game.o != Some(user_id) && game.x != Some(user_id) {
                        continue;
                    }
                    match game_type {
                        RoomType::Bot => {
                            if game.x != Some(user_id) {
                                continue;
                            }
                            let _ = sender_state.db.end_game(game.id).await;
                            let next_player = game.next_player;
                            let mut game = Game::new(Uuid::new_v4(), next_player);
                            game.x = Some(user_id);
                            game.o = Some(Uuid::nil());
                            if let Err(error) = sender_state
                                .db
                                .new_game(&game, &room_id, &game.next_player)
                                .await
                            {
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
                        _ => {
                            todo!("Feature in progress")
                        }
                    }
                }

                GameEvent::PredictBot { position } => {
                    if !matches!(game_type, RoomType::Bot) {
                        continue;
                    }
                    if game.x != Some(user_id) {
                        continue;
                    }
                    let next_move = Move::new(Player::X, position);
                    let result = game.play(&next_move);
                    if result.is_ok() {
                        let _ = sender_state
                            .db
                            .insert_move(&game.id, &next_move, game.moves.len())
                            .await;
                        let _ = sender_tx.send(GameEvent::MoveEvent { mv: next_move });
                        if let Ok(Some(win)) = game.check_winning_move(&position) {
                            game.winner = Some(win);
                            if let Err(error) = sender_state.db.update_game_winner(&game).await {
                                tracing::error!(?error, "Error update game winner");
                            }
                            let _ = sender_tx.send(GameEvent::Winner {
                                moves: game.winner.unwrap(),
                                last_move: next_move,
                            });
                            continue;
                        }
                        // if let  sender.send(Message::Text(
                        //     serde_json::to_string(&GameEvent::MoveEvent { mv: next_move }).unwrap(),
                        // ));
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
                                if let Err(error) = sender_state.db.update_game_winner(&game).await
                                {
                                    tracing::error!(?error, "Error update game winner");
                                }
                                let _ = sender_tx.send(GameEvent::Winner {
                                    moves: game.winner.unwrap(),
                                    last_move: bot_move,
                                });
                                continue;
                            }
                        }
                    } else {
                        let _ = sender_tx.send(GameEvent::InvalidMove {
                            player: game.next_player,
                        });
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

    tracing::info!("Closing websocket");
    {
        let mut rooms = state.rooms.lock().await;
        let room = rooms.get_mut(&room_id).unwrap();
        // let game = state.db.get_active_game(&room_id).await;
        // if let Ok(game) = game {
        //     let game = Game::try_from(game);
        //     if let Ok(game) = game {
        //         if game.x == Some(user_id) || game.o == Some(user_id) {}
        //     }
        // }
        room.users.remove(&user_id);
        tracing::info!("users: {:?}", room.users);

        if room.users.is_empty() {
            rooms.remove(&room_id);
        }
    }
}
