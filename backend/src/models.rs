use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use uuid::Uuid;

const WINNING_MOVE_COUNT: usize = 5;
const MAX_SCORE: i32 = 500;
const BOARD_SIZE: usize = 15;

#[derive(Debug, sqlx::Type, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "game_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum GameType {
    Private,
    Bot,
    Normal,
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "player_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PlayerStatus {
    Ready,
    Confirmed,
    ConfirmedThenLeft,
    Left,
}

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone)]
#[sqlx(type_name = "game_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum GameStatus {
    Ready,
    Playing,
    Ended,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub id: Uuid,
    pub board: [[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
    pub x: Option<Uuid>,
    pub o: Option<Uuid>,
    pub next_player: Player,
    pub moves: Vec<Move>,
    pub winner: Option<Vec<Move>>,
    pub x_status: PlayerStatus,
    pub o_status: PlayerStatus,
    pub game_type: GameType,
    pub room_id: Uuid,
    pub status: GameStatus,
}

impl Game {
    pub fn new(room_id: Uuid, next_player: Player, game_type: GameType) -> Self {
        Self {
            id: Uuid::new_v4(),
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            x: None,
            o: None,
            winner: None,
            next_player,
            moves: vec![],
            x_status: PlayerStatus::Ready,
            o_status: PlayerStatus::Ready,
            room_id,
            status: match game_type {
                GameType::Bot => GameStatus::Playing,
                GameType::Normal | GameType::Private => GameStatus::Ready,
            },
            game_type,
        }
    }

    pub fn play(&mut self, next_move @ Move { player, position }: &Move) -> Result<()> {
        if self.winner.is_some() {
            return Err(anyhow::anyhow!("Game already won"));
        }
        // if self.x.is_none() || self.o.is_none() {
        //     return Err(anyhow::anyhow!("Game not started"));
        // }
        if self.next_player != *player {
            return Err(anyhow::anyhow!("Invalid player"));
        }
        if position.col > BOARD_SIZE || position.row > BOARD_SIZE {
            return Err(anyhow::anyhow!("Invalid move"));
        }
        if self.board[position.row][position.col].is_some() {
            return Err(anyhow::anyhow!("Cell already taken"));
        }
        let last_move = self.moves.last();
        if let Some(mv) = last_move {
            if mv.player == *player {
                return Err(anyhow::anyhow!("Invalid move"));
            }
        }
        self.board[position.row][position.col] = Some(*player);
        self.next_player = match player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
        self.moves.push(*next_move);
        Ok(())
    }

    pub fn find_bot_move(&mut self, depth: i32) -> Option<Position> {
        let mut bot_score = -MAX_SCORE - 1;
        let mut bot_moves = vec![];
        if self.moves.is_empty() {
            return Some(Position::new(BOARD_SIZE / 2, BOARD_SIZE / 2));
        }
        let alpha = -MAX_SCORE;
        let beta = MAX_SCORE;

        let (_, mut threats) = self.evaluate();

        let mut player_threat = 0;
        let mut bot_threat = 0;

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.board[row][col].is_some() {
                    continue;
                }

                if !self.is_near_existing_move(row, col) {
                    continue;
                }

                let pos = Position::new(col, row);
                self.board[row][col] = Some(Player::O);
                if self.check_winning_move(&pos).unwrap().is_some() {
                    self.board[row][col] = None;
                    return Some(pos);
                }
                self.board[row][col] = None;
            }
        }

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.board[row][col].is_some() {
                    continue;
                }
                if !self.is_near_existing_move(row, col) {
                    continue;
                }
                let pos = Position::new(col, row);
                self.board[row][col] = Some(Player::X);
                if self.check_winning_move(&pos).unwrap().is_some() {
                    self.board[row][col] = None;
                    return Some(pos);
                }
                self.board[row][col] = None;
            }
        }

        threats.iter().for_each(|threat| match threat.2 {
            Player::X => player_threat = player_threat.max(threat.0),
            Player::O => bot_threat = bot_threat.max(threat.0),
        });

        for threat in threats.iter() {
            if threat.2 == Player::O && threat.0 == 4 {
                return Some(threat.1);
            }
            if threat.2 == Player::X && threat.0 == 4 {
                return Some(threat.1);
            }
            match threat.2 {
                Player::O => player_threat = player_threat.max(threat.0),
                Player::X => bot_threat = bot_threat.max(threat.0),
            }
        }

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.board[row][col].is_some() {
                    continue;
                }

                if !self.is_near_existing_move(row, col) {
                    continue;
                }
                self.board[row][col] = Some(Player::O);
                let count = self.double_winning_threats(
                    row,
                    col,
                    &[
                        "_oooo_", "_oooox", "xoooo_", "oo_oo", "o_ooo", "ooo_o", "_ooo_", "_oo_o_",
                        "_o_oo_",
                    ],
                );
                self.board[row][col] = None;
                if count > player_threat {
                    return Some(Position::new(col, row));
                }
            }
        }

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.board[row][col].is_some() {
                    continue;
                }

                if !self.is_near_existing_move(row, col) {
                    continue;
                }
                self.board[row][col] = Some(Player::O);
                let count = self.double_winning_threats(
                    row,
                    col,
                    &[
                        "_xxxx_", "_xxxxo", "oxxxx_", "xx_xx", "x_xxx", "xxx_x", "_xxx_", "_x_xx_",
                        "_xx_x_",
                    ],
                );
                self.board[row][col] = None;
                if count > bot_threat {
                    return Some(Position::new(col, row));
                }
            }
        }

        if threats.is_empty() {
            let last_move = self.moves.last().unwrap().position;
            let neighbors = self.get_neighbors(last_move.row, last_move.col);
            threats.extend(neighbors.into_iter().map(|pos| (0, pos, Player::O)));
            if self.moves.len() > 1 {
                let last_move = self.moves[self.moves.len() - 2].position;
                let neighbors = self.get_neighbors(last_move.row, last_move.col);
                for pos in neighbors {
                    if self.board[pos.row][pos.col].is_none() {
                        continue;
                    }
                    if threats
                        .iter()
                        .any(|(_, p, _)| p.col == pos.col && p.row == pos.row)
                    {
                        continue;
                    }
                    threats.push((0, pos, self.board[pos.row][pos.col].unwrap()));
                }
            }
        }

        if threats.is_empty() {
            let mut neighbors = vec![];
            for row in 0..BOARD_SIZE {
                for col in 0..BOARD_SIZE {
                    if self.board[row][col].is_some() {
                        continue;
                    }

                    if !self.is_near_existing_move(row, col) {
                        continue;
                    }
                    let pos = Position::new(col, row);
                    neighbors.push((0, pos, Player::O));
                }
            }
            threats.extend(neighbors);
        }

        for (_, pos @ Position { row, col }, _) in threats {
            self.next_player = Player::X;
            self.board[row][col] = Some(Player::O);
            let score = self.minimax(depth, false, alpha, beta, &mut HashMap::new());

            self.next_player = Player::O;

            self.board[row][col] = None;
            match score.cmp(&bot_score) {
                Ordering::Greater => {
                    bot_score = score;
                    bot_moves = vec![pos];
                }
                Ordering::Equal => {
                    bot_moves.push(pos);
                }
                Ordering::Less => {}
            }
        }

        bot_moves.sort_by(|a, b| {
            match self
                .near_by_same_player(a.row, a.col)
                .cmp(&self.near_by_same_player(b.row, b.col))
            {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => match rand::thread_rng().gen_range(0..=2) {
                    0 => Ordering::Less,
                    1 => Ordering::Equal,
                    _ => Ordering::Greater,
                },
            }
        });
        bot_moves.last().cloned()
    }

    fn board_to_string(&self) -> String {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        Some(p) => match p {
                            Player::O => "o",
                            Player::X => "x",
                        },
                        None => "_",
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("")
    }

    fn check_winning_horizontal(
        &self,
        Position { row, col }: &Position,
    ) -> Result<Option<Vec<Move>>> {
        if *col >= BOARD_SIZE || *row >= BOARD_SIZE {
            return Err(anyhow::anyhow!("Invalid position"));
        }

        if self.board[*row][*col].is_none() {
            return Err(anyhow::anyhow!("Position is empty"));
        }

        let cell = self.board[*row][*col].unwrap();
        let mut winning_count = 0;
        let mut moves = vec![];
        for i in col.saturating_sub(WINNING_MOVE_COUNT)
            ..col.saturating_add(WINNING_MOVE_COUNT).min(BOARD_SIZE)
        {
            if let Some(c) = self.board[*row][i] {
                if c == cell {
                    winning_count += 1;
                    moves.push(Move::new(cell, Position::new(i, *row)));
                    if winning_count >= WINNING_MOVE_COUNT {
                        return Ok(Some(moves));
                    }
                } else {
                    winning_count = 0;
                    moves = vec![];
                }
            } else {
                winning_count = 0;
                moves = vec![];
            }
        }

        Ok(None)
    }

    fn patterns(&self, str: &str, pattern: &str) -> usize {
        let o = pattern.chars().filter(|c| *c == 'o').count();
        let x = pattern.chars().filter(|c| *c == 'x').count();
        let chr = if o > x { 'o' } else { 'x' };
        if str.contains(pattern) {
            return pattern.chars().filter(|c| *c == chr).count();
        }
        0
    }

    fn double_winning_threats(&self, row: usize, col: usize, patterns: &[&str]) -> usize {
        let mut threats = 0;
        let mut count_row = 0;
        let mut count_col = 0;
        let mut count_upper_diagonal = 0;
        let mut count_lower_diagonal = 0;
        patterns.iter().for_each(|pattern| {
            count_row = count_row.max(self.patterns(&moves_to_string(&self.board[row]), pattern));
            let column: Vec<Option<Player>> = self.board.iter().map(|row| row[col]).collect();
            count_col = count_col.max(self.patterns(&moves_to_string(&column), pattern));

            let diagonal = self.get_diagonal(row, col, true);
            count_upper_diagonal =
                count_upper_diagonal.max(self.patterns(&moves_to_string(&diagonal), pattern));

            let diagonal = self.get_diagonal(row, col, false);
            count_lower_diagonal =
                count_lower_diagonal.max(self.patterns(&moves_to_string(&diagonal), pattern));
        });
        threats += if count_row > 0 { 1 } else { 0 };
        threats += if count_col > 0 { 1 } else { 0 };
        threats += if count_lower_diagonal > 0 { 1 } else { 0 };
        threats += if count_upper_diagonal > 0 { 1 } else { 0 };

        if threats >= 2 {
            count_row
                .max(count_col)
                .max(count_lower_diagonal)
                .max(count_upper_diagonal)
        } else {
            0
        }
    }

    fn find_patterns(
        &self,
        vect_str: &str,
        pattern: &str,
        bot_scores: &mut Vec<i32>,
        player_scores: &mut Vec<i32>,
        threats: &mut Vec<(usize, Position, Player)>,
        positions: &[Position],
    ) {
        let o = pattern.chars().filter(|c| *c == 'o').count();
        let x = pattern.chars().filter(|c| *c == 'x').count();
        let score = (o.max(x) as i32) * 100;
        let player = if o > x { Player::O } else { Player::X };

        let pos = pattern
            .chars()
            .enumerate()
            .filter_map(|(i, c)| if c == '_' { Some(i) } else { None });
        if let Some(idx) = vect_str.find(pattern) {
            pos.for_each(|p| {
                threats.push((
                    match player {
                        Player::O => o,
                        Player::X => x,
                    },
                    positions[idx + p],
                    player,
                ))
            });
            match player {
                Player::O => bot_scores.push(score),
                Player::X => player_scores.push(score),
            }
        }
    }

    fn find_threats(
        &self,
        vect_str: &str,
        player_scores: &mut Vec<i32>,
        bot_scores: &mut Vec<i32>,
        positions: Vec<Position>,
    ) -> Vec<(usize, Position, Player)> {
        let mut threats = vec![];
        if vect_str.contains("ooooo") {
            bot_scores.push(MAX_SCORE);
            return threats;
        }
        if vect_str.contains("xxxxx") {
            player_scores.push(MAX_SCORE);
            return threats;
        }

        for pattern in [
            "_oooo_", "_oooox", "xoooo_", "oo_oo", "o_ooo", "ooo_o", "_ooo_", "_oo_o_", "_o_oo_",
            "_oo_o_", "_o_oo_", "_xxxx_", "_xxxxo", "oxxxx_", "xx_xx", "x_xxx", "xxx_x", "_xxx_",
            "_xx_x_", "_x_xx_", "_xx_x_", "_x_xx_",
        ] {
            self.find_patterns(
                vect_str,
                pattern,
                bot_scores,
                player_scores,
                &mut threats,
                &positions,
            );
        }

        threats.sort_by(|a, b| a.0.cmp(&b.0).reverse());
        // println!("{threats:?}");
        threats
    }

    fn check_winning_vertical(
        &self,
        Position { row, col }: &Position,
    ) -> Result<Option<Vec<Move>>> {
        if *col >= BOARD_SIZE || *row >= BOARD_SIZE {
            return Err(anyhow::anyhow!("Invalid position"));
        }

        if self.board[*row][*col].is_none() {
            return Err(anyhow::anyhow!("Position is empty"));
        }

        let cell = self.board[*row][*col].unwrap();
        let mut winning_count = 0;
        let mut moves = vec![];
        for i in row.saturating_sub(WINNING_MOVE_COUNT)
            ..row.saturating_add(WINNING_MOVE_COUNT).min(BOARD_SIZE)
        {
            if let Some(c) = self.board[i][*col] {
                if c == cell {
                    winning_count += 1;
                    moves.push(Move::new(cell, Position::new(*col, i)));
                    if winning_count >= WINNING_MOVE_COUNT {
                        return Ok(Some(moves));
                    }
                } else {
                    winning_count = 0;
                    moves = vec![];
                }
            } else {
                winning_count = 0;
                moves = vec![];
            }
        }

        Ok(None)
    }

    fn get_diagonal(&self, row: usize, col: usize, is_upper_left: bool) -> Vec<Option<Player>> {
        let mut diagonal = vec![];
        for i in -(WINNING_MOVE_COUNT as i32)..(WINNING_MOVE_COUNT as i32) + 1 {
            let new_row = (row as i32) + i;
            let new_col = (col as i32)
                + match is_upper_left {
                    true => i,
                    false => -i,
                };
            if new_row < 0
                || new_row >= BOARD_SIZE as i32
                || new_col < 0
                || new_col >= BOARD_SIZE as i32
            {
                continue;
            }
            diagonal.push(self.board[new_row as usize][new_col as usize]);
        }
        diagonal
    }

    fn check_winning_diagonal(
        &self,
        Position { row, col }: &Position,
        is_upper_left: bool,
    ) -> Result<Option<Vec<Move>>> {
        if *col >= BOARD_SIZE || *row >= BOARD_SIZE {
            return Err(anyhow::anyhow!("Invalid position"));
        }

        if self.board[*row][*col].is_none() {
            return Err(anyhow::anyhow!("Position is empty"));
        }

        let cell = self.board[*row][*col].unwrap();
        let mut winning_count = 0;
        let mut moves = vec![];

        for i in -(WINNING_MOVE_COUNT as i32)..(WINNING_MOVE_COUNT as i32) + 1 {
            if (*row as i32) + i < 0 {
                continue;
            }
            if (*col as i32)
                + match is_upper_left {
                    true => i,
                    false => -i,
                }
                < 0
            {
                continue;
            }

            if (*row as i32) + i >= BOARD_SIZE as i32 {
                continue;
            }
            if (*col as i32)
                + match is_upper_left {
                    true => i,
                    false => -i,
                }
                >= BOARD_SIZE as i32
            {
                continue;
            }

            if let Some(c) = self.board[(*row as i32 + i) as usize][(*col as i32
                + match is_upper_left {
                    true => i,
                    false => -i,
                }) as usize]
            {
                if c == cell {
                    winning_count += 1;
                    moves.push(Move::new(
                        cell,
                        Position::new(
                            (*col as i32
                                + match is_upper_left {
                                    true => i,
                                    false => -i,
                                }) as usize,
                            (*row as i32 + i) as usize,
                        ),
                    ));
                    if winning_count >= WINNING_MOVE_COUNT {
                        return Ok(Some(moves));
                    }
                } else {
                    winning_count = 0;
                    moves = vec![];
                }
            } else {
                winning_count = 0;
                moves = vec![];
            }
        }

        Ok(None)
    }

    pub fn check_winning_move(&self, pos: &Position) -> Result<Option<Vec<Move>>> {
        let vertical = self.check_winning_vertical(pos)?;
        if vertical.is_some() {
            return Ok(vertical);
        }
        let horizontal = self.check_winning_horizontal(pos)?;
        if horizontal.is_some() {
            return Ok(horizontal);
        }
        let upper = self.check_winning_diagonal(pos, true)?;
        if upper.is_some() {
            return Ok(upper);
        }
        let lower = self.check_winning_diagonal(pos, false)?;
        if lower.is_some() {
            return Ok(lower);
        }
        Ok(None)
    }

    pub fn check_win(&self) -> Option<Vec<Move>> {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let pos = Position::new(col, row);
                if let Ok(check) = self.check_winning_move(&pos) {
                    if check.is_some() {
                        return check;
                    }
                }
            }
        }
        None
    }

    fn evaluate(&self) -> (i32, Vec<(usize, Position, Player)>) {
        let mut player = vec![];
        let mut bot = vec![];
        let mut threats = vec![];
        for row in 0..BOARD_SIZE {
            let row_str = moves_to_string(&self.board[row]);
            threats.extend(
                self.find_threats(
                    &row_str,
                    &mut player,
                    &mut bot,
                    self.board[row]
                        .iter()
                        .enumerate()
                        .map(|(i, _)| Position::new(i, row))
                        .collect(),
                ),
            );
        }

        for col in 0..BOARD_SIZE {
            let col_moves = self.board.iter().map(|row| row[col]).collect::<Vec<_>>();
            threats.extend(
                self.find_threats(
                    &moves_to_string(&col_moves),
                    &mut player,
                    &mut bot,
                    col_moves
                        .iter()
                        .enumerate()
                        .map(|(i, _)| Position::new(col, i))
                        .collect(),
                ),
            );
        }

        for row_start in 0..BOARD_SIZE {
            let mut diagonal = vec![];
            let mut positions = vec![];
            for i in 0..BOARD_SIZE - row_start {
                diagonal.push(self.board[row_start + i][i]);
                positions.push(Position::new(i, row_start + i));
            }
            if diagonal.len() >= 5 {
                let str = moves_to_string(&diagonal);
                threats.extend(self.find_threats(&str, &mut player, &mut bot, positions));
            }
        }

        for col_start in 1..BOARD_SIZE {
            let mut diagonal = vec![];
            let mut positions = vec![];

            for i in 0..BOARD_SIZE - col_start {
                diagonal.push(self.board[i][col_start + i]);
                positions.push(Position::new(col_start + i, i));
            }
            if diagonal.len() >= 5 {
                let str = moves_to_string(&diagonal);
                threats.extend(self.find_threats(&str, &mut player, &mut bot, positions));
            }
        }

        // for col_start in range(N):
        // diag = [board[N - 1 - i][col_start + i] for i in range(N - col_start)]
        // diagonals.append(diag)
        for col_start in 0..BOARD_SIZE {
            let mut diagonal = vec![];
            let mut positions = vec![];

            for i in 0..BOARD_SIZE - col_start {
                diagonal.push(self.board[BOARD_SIZE - 1 - i][col_start + i]);
                positions.push(Position::new(col_start + i, BOARD_SIZE - 1 - i));
            }
            if diagonal.len() >= 5 {
                let str = moves_to_string(&diagonal);
                threats.extend(self.find_threats(&str, &mut player, &mut bot, positions));
            }
        }

        for row_start in (0..BOARD_SIZE - 1).rev() {
            let mut diagonal = vec![];
            let mut positions = vec![];
            for i in 0..row_start + 1 {
                diagonal.push(self.board[row_start - i][i]);
                positions.push(Position::new(i, row_start - i));
            }
            if diagonal.len() >= 5 {
                let str = moves_to_string(&diagonal);
                threats.extend(self.find_threats(&str, &mut player, &mut bot, positions));
            }
        }

        // for row in 0..BOARD_SIZE {
        //     for col in 0..BOARD_SIZE {
        //         if self.board[row][col].is_none() {
        //             continue;
        //         }
        //         if !self.is_near_existing_move(row, col) {
        //             continue;
        //         }

        //         let p = self.board[row][col].unwrap();

        //         let count = self.double_winning_threats(
        //             row,
        //             col,
        //             match p {
        //                 Player::O => &[
        //                     "_oooo_", "_oooox", "xoooo_", "oo_oo", "o_ooo", "ooo_o", "_ooo_",
        //                     "_oo_o_", "_o_oo_",
        //                 ],
        //                 Player::X => &[
        //                     "_xxxx_", "_xxxxo", "oxxxx_", "xx_xx", "x_xxx", "xxx_x", "_xxx_",
        //                     "_x_xx_", "_xx_x_",
        //                 ],
        //             },
        //         ) as i32;
        //         if count > 0 {
        //             threats.push((count as usize, Position::new(col, row), p));
        //             match p {
        //                 Player::O => bot.push(count * 100),
        //                 Player::X => player.push(count * 100),
        //             }
        //         }
        //     }
        // }

        // # Diagonals starting from the left column, excluding the first one to avoid duplication
        // for row_start in range(N - 2, -1, -1):
        //     diag = [board[row_start - i][i] for i in range(row_start + 1)]
        //     diagonals.append(diag)

        threats.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Equal => {
                if matches!(a.2, Player::O) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
        });

        (
            match (player.first(), bot.first()) {
                (Some(p), None) => *p * -1,
                (None, Some(b)) => *b,
                (Some(p), Some(b)) => match p.cmp(b) {
                    Ordering::Greater | Ordering::Equal => *p * -1,
                    Ordering::Less => *b,
                },
                _ => 0,
            },
            threats,
        )
    }

    pub fn is_full(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|cell| cell.is_some()))
    }

    fn near_by_same_player(&self, row: usize, col: usize) -> i32 {
        let offsets = [-1, 0, 1];
        let mut count = 0;
        for &dx in &offsets {
            for &dy in &offsets {
                let new_row = row as isize + dy;
                let new_col = col as isize + dx;
                if new_row >= 0
                    && new_row < BOARD_SIZE as isize
                    && new_col >= 0
                    && new_col < BOARD_SIZE as isize
                    && self.board[new_row as usize][new_col as usize].is_some()
                    && self.board[new_row as usize][new_col as usize].unwrap() == self.next_player
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_neighbors(&self, row: usize, col: usize) -> Vec<Position> {
        let offsets = [-1, 0, 1];
        let mut neighbors = vec![];
        for &dx in &offsets {
            for &dy in &offsets {
                let new_row = row as isize + dy;
                let new_col = col as isize + dx;
                if new_row >= 0
                    && new_row < BOARD_SIZE as isize
                    && new_col >= 0
                    && new_col < BOARD_SIZE as isize
                    && self.board[new_row as usize][new_col as usize].is_none()
                {
                    neighbors.push(Position::new(new_col as usize, new_row as usize));
                }
            }
        }
        neighbors
    }

    fn is_near_existing_move(&self, row: usize, col: usize) -> bool {
        let offsets = [-1, 0, 1];
        for &dx in &offsets {
            for &dy in &offsets {
                let new_row = row as isize + dy;
                let new_col = col as isize + dx;
                if new_row >= 0
                    && new_row < BOARD_SIZE as isize
                    && new_col >= 0
                    && new_col < BOARD_SIZE as isize
                    && self.board[new_row as usize][new_col as usize].is_some()
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn minimax(
        &mut self,
        depth: i32,
        is_maximizing: bool,
        mut alpha: i32,
        mut beta: i32,
        transposition_table: &mut HashMap<String, i32>,
        // sender: Option<&Sender<GameEvent>>,
        // sender: &mut SplitSink<WebSocket, Message>,
    ) -> i32 {
        let board_key = self.board_to_string();
        if let Some(&cached_score) = transposition_table.get(&board_key) {
            return cached_score;
        }

        let (score, mut threats) = self.evaluate();

        if score >= MAX_SCORE || score <= -MAX_SCORE || depth == 0 {
            return score + if is_maximizing { -depth } else { depth };
        }

        if self.is_full() {
            return 0;
        }

        if threats.is_empty() {
            for row in 0..BOARD_SIZE {
                for col in 0..BOARD_SIZE {
                    if self.board[row][col].is_some() {
                        continue;
                    }

                    if !self.is_near_existing_move(row, col) {
                        continue;
                    }
                    let pos = Position::new(col, row);
                    threats.push((0, pos, Player::O));
                }
            }
        }

        if is_maximizing {
            let mut best_score = -MAX_SCORE;
            for (_, Position { row, col }, _) in threats {
                self.board[row][col] = Some(Player::O);
                self.next_player = Player::X;

                let score = self.minimax(depth - 1, false, alpha, beta, transposition_table);
                // if let Some(sender) = sender {
                //     let _ = sender.send(GameEvent::MiniMax {
                //         position: pos,
                //         score,
                //     });
                // }
                // tokio::time::sleep(std::time::Duration::from_nanos(1)).await;
                self.next_player = Player::O;

                self.board[row][col] = None;

                best_score = best_score.max(score);
                alpha = alpha.max(score);

                if beta <= alpha {
                    break;
                }
            }
            transposition_table.insert(board_key, best_score);
            best_score
        } else {
            let mut best_score = MAX_SCORE;

            for (_, Position { row, col }, _) in threats {
                self.board[row][col] = Some(Player::X);
                self.next_player = Player::O;

                let score = self.minimax(depth - 1, true, alpha, beta, transposition_table);

                // if let Some(sender) = sender {
                //     let _ = sender.send(GameEvent::MiniMax {
                //         position: pos,
                //         score,
                //     });
                // }
                // tokio::time::sleep(std::time::Duration::from_nanos(1)).await;

                self.board[row][col] = None;
                self.next_player = Player::X;

                best_score = best_score.min(score);
                beta = beta.min(score);
                if beta <= alpha {
                    break;
                }
            }
            transposition_table.insert(board_key, best_score);
            best_score
        }
    }
}

fn moves_to_string(vect: &[Option<Player>]) -> String {
    vect.iter()
        .map(|mv| match mv {
            Some(Player::O) => "o",
            Some(Player::X) => "x",
            None => "_",
        })
        .collect::<String>()
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
    pub fn new(player: Player, position: Position) -> Self {
        Self { position, player }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Position {
    pub fn new(col: usize, row: usize) -> Self {
        Self { row, col }
    }
}

pub struct GameDb {
    pub room_id: Uuid,
    pub id: Uuid,
    pub x: Option<Uuid>,
    pub o: Option<Uuid>,
    pub moves: serde_json::Value,
    pub winner: serde_json::Value,
    pub init_player: Player,
    pub game_type: GameType,
    pub x_status: PlayerStatus,
    pub o_status: PlayerStatus,
    pub status: GameStatus,
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
            .map(|m| Move::new(m.player, Position::new(m.col, m.row)))
            .collect();
        let next_player = match moves.last() {
            Some(mv) => match mv.player {
                Player::X => Player::O,
                Player::O => Player::X,
            },
            None => game.init_player,
        };
        let mut board = [[None; BOARD_SIZE]; BOARD_SIZE];
        moves.iter().for_each(|mv| {
            board[mv.position.row][mv.position.col] = Some(mv.player);
        });
        let winner: Option<Vec<Move>> = serde_json::from_value(game.winner)?;
        let game = Game {
            room_id: game.room_id,
            board,
            id: game.id,
            moves,
            next_player,
            winner,
            o: game.o,
            x: game.x,
            x_status: game.x_status,
            o_status: game.o_status,
            game_type: game.game_type,
            status: game.status,
        };

        Ok(game)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event")]
pub enum GameEvent {
    Game {
        game: Box<Game>,
    },
    MoveEvent {
        mv: Move,
    },
    InvalidMove {
        player: Player,
    },
    Winner {
        moves: Vec<Move>,
        last_move: Move,
    },
    MiniMax {
        position: Position,
        score: i32,
    },
    Message {
        msg: String,
        id: Uuid,
        user: Option<User>,
    },
    Status {
        status: GameStatus,
    },
    PlayerLeft,
    PlayAgain,
    Ended,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub avatar: String,
    pub name: String,
    pub id: Uuid,
}
