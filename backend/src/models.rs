use serde::{Deserialize, Serialize};
use uuid::Uuid;

const _WINNING_MOVE_COUNT: usize = 5;
const _MAX_SCORE: i32 = (_WINNING_MOVE_COUNT as i32) * 100;
const BOARD_SIZE: usize = 15;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub id: Uuid,
    pub board: [[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
    pub x: Option<Uuid>,
    pub o: Option<Uuid>,
    pub next_player: Player,
    pub moves: Vec<Move>,
}

impl Game {
    pub fn new(id: Uuid, next_player: Player) -> Self {
        Self {
            id,
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            x: None,
            o: None,
            next_player,
            moves: vec![],
        }
    }
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize, Clone, Eq, PartialEq, Copy)]
#[sqlx(type_name = "player", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Player {
    X,
    O,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Move {
    pub position: Position,
    pub player: Player,
}

impl Move {
    pub fn new(row: usize, col: usize, player: Player) -> Self {
        Self {
            position: Position::new(row, col),
            player,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { col, row }
    }
}

pub struct GameDb {
    pub id: Uuid,
    pub x: Option<Uuid>,
    pub o: Option<Uuid>,
    pub moves: serde_json::Value,
}

#[derive(Deserialize)]
pub struct GameMoveDb {
    pub row: usize,
    pub col: usize,
    pub player: Player,
}

impl TryFrom<GameDb> for Game {
    type Error = serde_json::Error;
    fn try_from(game: GameDb) -> Result<Self, Self::Error> {
        let moves = serde_json::from_value::<Vec<GameMoveDb>>(game.moves);
        let moves = moves.unwrap_or_default();
        let moves: Vec<Move> = moves
            .into_iter()
            .map(|m| Move::new(m.row, m.col, m.player))
            .collect();
        let next_player = match moves.last() {
            Some(mv) => match mv.player {
                Player::X => Player::O,
                Player::O => Player::X,
            },
            None => Player::X,
        };
        let mut board = [[None; BOARD_SIZE]; BOARD_SIZE];
        moves.iter().for_each(|mv| {
            board[mv.position.row][mv.position.col] = Some(mv.player);
        });
        let game = Game {
            board,
            id: game.id,
            moves,
            next_player,
            o: game.o,
            x: game.x,
        };

        Ok(game)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event")]
pub enum GameEvent {
    Game { game: Box<Game> },
    PredictBot { position: Position },
    MoveEvent { mv: Move },
    InvalidMove { player: Player },
    Winner { moves: Vec<Move>, last_move: Move },
    MiniMax { position: Position, score: i32 },
    PlayerLeft { player: Player, game: Uuid },
    PlayerJoined { player: Player, game: Uuid },
    NextPlayer { player: Player },
    Chat { msg: String },
    PlayAgain,
}
