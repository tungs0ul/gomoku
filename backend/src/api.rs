use crate::db::Db;
use crate::models::{Game, GameEvent, Move, Player};
use axum::extract::ws::{CloseFrame, Message, WebSocket};
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use futures::SinkExt;
use futures::StreamExt;
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
        .route("/health_check", get(health_check))
        .route("/game/bot/:user_id", get(create_bot_game))
        .route("/ws/:room_id/:user_id", get(websocket_handler))
        .layer(CorsLayer::permissive())
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
    state.db.new_game(&game, &room_id).await.map_err(|error| {
        tracing::error!(?error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(room_id.to_string())
}

async fn websocket_handler(
    Path((room_id, user_id)): Path<(String, String)>,
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    tracing::info!("Trying to connect");
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
            if let Ok(msg) = msg {
                let game = sender_state.db.get_active_game_for_room(&room_id).await;
                if game.is_err() {
                    tracing::error!("Game not found");
                    break;
                }
                let game = Game::try_from(game.unwrap());
                if game.is_err() {
                    tracing::error!("Invalid game");
                    break;
                }
                let mut game = game.unwrap();
                match msg {
                    GameEvent::Chat { msg } => {
                        let _ = sender_tx.send(GameEvent::Chat { msg });
                    }
                    GameEvent::MoveEvent { mv } => {
                        if game.play(&mv).is_ok() {
                            let _ = sender_state
                                .db
                                .insert_move(&game.id, &mv, game.moves.len())
                                .await;
                            if let Ok(Some(win)) = game.check_winning_move(&mv.position) {
                                game.winner = Some(win);
                                let _ = sender_state.db.end_game(game.id).await;
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
                        if game.o == Some(Uuid::nil()) {
                            let _ = sender_state.db.end_game(game.id).await;
                            let next_player = game.next_player;
                            let mut game = Game::new(Uuid::new_v4(), next_player);
                            game.x = Some(user_id);
                            game.o = Some(Uuid::nil());
                            if let Err(error) = sender_state.db.new_game(&game, &room_id).await {
                                tracing::error!(?error, "Error saving game");
                            }
                            if game.next_player == Player::O {
                                let predict = game.find_bot_move(2);
                                if let Some(pos) = predict {
                                    let bot_move = Move::new(Player::O, pos);
                                    game.play(&bot_move).unwrap();
                                    let _ = sender_tx.send(GameEvent::MoveEvent { mv: bot_move });
                                    let _ = state
                                        .db
                                        .insert_move(&game.id, &bot_move, game.moves.len())
                                        .await;
                                }
                            }
                            let _ = sender_tx.send(GameEvent::Game {
                                game: Box::new(game),
                            });
                        }
                    }

                    GameEvent::PredictBot { position } => {
                        let next_move = Move::new(Player::X, position);
                        let result = game.play(&next_move);
                        if result.is_ok() {
                            let _ = state
                                .db
                                .insert_move(&game.id, &next_move, game.moves.len())
                                .await;
                            let _ = sender_tx.send(GameEvent::MoveEvent { mv: next_move });
                            if let Ok(Some(win)) = game.check_winning_move(&position) {
                                game.winner = Some(win);
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
                                let _ = state
                                    .db
                                    .insert_move(&game.id, &bot_move, game.moves.len())
                                    .await;
                                if let Ok(Some(win)) = game.check_winning_move(&pos) {
                                    game.winner = Some(win);
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
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    tracing::info!("Closing websocket");
    {
        // let mut rooms = state.rooms.lock().await;
        // let room = rooms.get_mut(&room_id).unwrap();
        // let game = state.db.get_active_game(&room_id).await;
        // if let Ok(game) = game {
        //     let game = Game::try_from(game);
        //     if let Ok(game) = game {
        //         if game.x == Some(user_id) || game.o == Some(user_id) {}
        //     }
        // }
        // room.users.remove(&user_id);
        // tracing::info!("users: {:?}", room.users);
        //
        // if room.users.is_empty() {
        //     rooms.remove(&room_id);
        // }
    }
}
