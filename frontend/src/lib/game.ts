import type { Board, Player, Position } from '$lib/types'

const SCORE_5 = 10000
const SCORE_4 = 900
const SCORE_3 = 800
const SCORE_2 = 700
const WINNING_MOVE_COUNT = 5
const BOARD_SIZE = 15

let hashTable = new Map<string, number>()

export const boardToString = (board: Board) => {
  return board
    .map((row) => row.map((cell) => (cell === null ? '_' : cell)).join(''))
    .join('')
}

// pub async fn find_best_move(
//     &mut self,
//     depth: i32,
//     sender: &Sender<GameEvent>,
// ) -> Option<Position> {
//     let mut best_score = -MAX_SCORE - 1;
//     let mut best_moves = vec![];
//     // if self.moves.is_empty() {
//     //     return Some(Position::new(BOARD_SIZE / 2, BOARD_SIZE / 2));
//     // }
//     let alpha = -MAX_SCORE;
//     let beta = MAX_SCORE;

//     let (_, mut threats) = self.evaluate();

//     if threats.is_empty() {
//         let mut neighbors = vec![];
//         for row in 0..BOARD_SIZE {
//             for col in 0..BOARD_SIZE {
//                 if self.board[row][col].is_some() {
//                     continue;
//                 }

//                 if !self.is_near_existing_move(row, col) {
//                     continue;
//                 }
//                 let pos = Position::new(col, row);
//                 //push random score
//                 neighbors.push((rand::thread_rng().gen_range(0..100), pos));
//             }
//         }
//         neighbors.sort_by(|a, b| a.0.cmp(&b.0).reverse());
//         threats.extend(neighbors);
//     }

//     for (_, pos @ Position { row, col }) in threats {
//         // if self.board[row][col].is_some() {
//         //     continue;
//         // }

//         // if !self.is_near_existing_move(row, col) {
//         //     continue;
//         // }
//         self.next_player = Player::X;
//         self.board[row][col] = Some(Player::O);
//         // let score = self.evaluate();
//         let score = self.minimax(depth, false, alpha, beta, &mut HashMap::new());
//         if let Err(error) = sender.send(GameEvent::MiniMax {
//             position: pos,
//             score,
//         }) {
//             tracing::error!(?error, "Error sending mini max message");
//         }
//         tokio::time::sleep(std::time::Duration::from_millis(2)).await;
//         // .await;
//         self.next_player = Player::O;

//         self.board[row][col] = None;
//         if score > best_score {
//             best_score = score;
//             best_moves = vec![pos];
//         } else if score == best_score {
//             best_moves.push(pos);
//         }
//     }

//     if best_moves.is_empty() {
//         return None;
//     }
//     Some(*best_moves.choose(&mut rand::thread_rng()).unwrap())
// }

export const setMove = (
  board: Board,
  player: Player,
  { row, col }: { row: number; col: number }
) => {
  board[row][col] = player
  return player === 'X' ? 'O' : 'X'
}

const undo = (board: Board, { row, col }: Position) => {
  board[row][col] = null
}

type Threat = {
  score: number
  position: Position
}

const movesToString = (moves: (Player | null)[]) => {
  return moves.map((e) => (e === null ? '_' : e)).join('')
}

const evaluate = (
  board: Board,
  nextPlayer: Player
): { score: number; threats: Threat[] } => {
  let player: number[] = []
  let bot: number[] = []
  let threats: Threat[] = []
  board.forEach((row, i) => {
    let rowStr = movesToString(row)
    threats.push(
      ...findThreats(
        rowStr,
        player,
        bot,
        row.map((_, j) => ({ row: i, col: j })),
        nextPlayer
      )
    )
    let colMoves = board.map((row, j) => ({ player: row[i], row: j, col: i }))
    let colStr = movesToString(colMoves.map((m) => m.player))
    threats.push(...findThreats(colStr, player, bot, colMoves, nextPlayer))
  })

  console.log(threats)

  for (let rowStart = 0; rowStart < BOARD_SIZE; ++rowStart) {
    let diagonalMoves = []
    for (let i = 0; i < BOARD_SIZE - rowStart; ++i) {
      diagonalMoves.push({
        player: board[rowStart + i][i],
        row: rowStart + i,
        col: i
      })
    }
    if (diagonalMoves.length >= 5) {
      let diagonalStr = movesToString(diagonalMoves.map((m) => m.player))
      threats.push(
        ...findThreats(diagonalStr, player, bot, diagonalMoves, nextPlayer)
      )
    }
  }

  for (let colStart = 1; colStart < BOARD_SIZE; ++colStart) {
    let diagonalMoves = []
    for (let i = 0; i < BOARD_SIZE - colStart; ++i) {
      diagonalMoves.push({
        player: board[i][colStart + i],
        row: i,
        col: colStart + i
      })
    }
    if (diagonalMoves.length >= 5) {
      let diagonalStr = movesToString(diagonalMoves.map((m) => m.player))
      threats.push(
        ...findThreats(diagonalStr, player, bot, diagonalMoves, nextPlayer)
      )
    }
  }

  for (let colStart = 0; colStart < BOARD_SIZE; ++colStart) {
    let diagonalMoves = []
    for (let i = 0; i < BOARD_SIZE - colStart; ++i) {
      diagonalMoves.push({
        player: board[BOARD_SIZE - 1 - i][colStart + i],
        row: BOARD_SIZE - 1 - i,
        col: colStart + i
      })
    }
    if (diagonalMoves.length >= 5) {
      let diagonalStr = movesToString(diagonalMoves.map((m) => m.player))
      threats.push(
        ...findThreats(diagonalStr, player, bot, diagonalMoves, nextPlayer)
      )
    }
  }

  for (let rowStart = BOARD_SIZE - 2; rowStart >= 0; --rowStart) {
    let diagonalMoves = []
    for (let i = 0; i < rowStart + 1; ++i) {
      diagonalMoves.push({
        player: board[rowStart - i][i],
        row: rowStart - i,
        col: i
      })
    }
    if (diagonalMoves.length >= 5) {
      let diagonalStr = movesToString(diagonalMoves.map((m) => m.player))
      threats.push(
        ...findThreats(diagonalStr, player, bot, diagonalMoves, nextPlayer)
      )
    }
  }

  let score = 0
  if (player.length > 0 && bot.length > 0) {
    if (player[0] >= bot[0]) {
      score = -player[0]
    } else {
      score = bot[0]
    }
  } else if (player.length > 0) {
    score = -player[0]
  } else if (bot.length > 0) {
    score = bot[0]
  }
  threats.sort((a, b) => b.score - a.score)
  return { score, threats }
}

const checkWin = (str: string, positions: Position[]) => {
  let idx = str.search('OOOOO')
  if (idx !== -1) {
    return { player: 'O', position: positions.slice(idx, idx + 5) }
  }
  idx = str.search('XXXXX')
  if (idx !== -1) {
    return { player: 'X', position: positions.slice(idx, idx + 5) }
  }
  return null
}

const nextPlayerScore = (nextPlayer: Player, player: Player) => {
  // return 0;
  return player === nextPlayer ? 50 : 0
}

const checkPattern = (
  str: string,
  pattern: string,
  nextPlayer: Player,
  positions: Position[]
) => {
  let o = pattern.split('').filter((e) => e === 'o').length
  let x = pattern.split('').filter((e) => e === 'x').length
  let player: Player = 'O'
  if (o < x) {
    player = 'X'
  }
  let score = (o - x) * 100
  let pos = pattern
    .split('')
    .map((e, i) => (e === '_' ? i : null))
    .filter((e) => e !== null)
  let idx = str.search(pattern)
  if (idx !== -1) {
    let scr = score + nextPlayerScore(nextPlayer, player)
    return {
      stop: scr === SCORE_5,
      threats: pos.map((p) => ({
        score: scr,
        position: positions[idx + p]
      }))
    }
  }
  return { stop: false, threats: [] }
}

// console.log(checkPattern("x_ooo_x", "_ooo_", SCORE_4, "O", "O", [{ row: 0, col: 0 }, { row: 0, col: 1 }, { row: 0, col: 2 }, { row: 0, col: 3 }, { row: 0, col: 4 }, { row: 0, col: 5 }, { row: 0, col: 6 }]))

const findThreats = (
  str: string,
  player: number[],
  bot: number[],
  positions: Position[],
  nextPlayer: Player
) => {
  str = str.toLowerCase()
  let threats: Threat[] = []
  let win = checkWin(str, positions)
  if (win !== null) {
    if (win.player === 'O') {
      bot.push(SCORE_5)
    } else {
      player.push(SCORE_5)
    }
    return threats
  }
  ;[
    'oooo_',
    '_oooo',
    'oo_oo',
    'ooo_o',
    'o_ooo',
    'xxxx_',
    '_xxxx',
    'xx_xx',
    'x_xxx',
    'xxx_x',
    '_ooo_',
    '_oo_o_',
    '_o_oo_',
    '_xxx_',
    '_xx_x_',
    '_x_xx_',
    'xooo__',
    '__ooox',
    'oxxx__',
    '__xxxo'
  ].forEach((pattern) => {
    threats.push(...checkPattern(str, pattern, nextPlayer, positions).threats)
  })

  return threats
}

const isNearExistingMove = (board: Board, row: number, col: number) => {
  let offsets = [-1, 0, 1]
  for (let dx of offsets) {
    for (let dy of offsets) {
      let new_row = row + dy
      let new_col = col + dx
      if (
        new_row >= 0 &&
        new_row < BOARD_SIZE &&
        new_col >= 0 &&
        new_col < BOARD_SIZE &&
        board[new_row][new_col] !== null
      ) {
        return true
      }
    }
  }
  return false
}

const minimax = (
  board: Board,
  nextPlayer: Player,
  depth: number,
  isMaximizingPlayer: boolean,
  alpha: number,
  beta: number
) => {
  let str = boardToString(board)
  if (hashTable.has(str)) {
    return hashTable.get(str)!
  }
  let { score, threats } = evaluate(board, nextPlayer)
  if (score === SCORE_5 || score === -SCORE_5 || depth == 0) {
    return score
  }
  if (board.every((row) => row.every((e) => e !== null))) {
    return 0
  }

  if (threats.length === 0) {
    for (let row = 0; row < BOARD_SIZE; row++) {
      for (let col = 0; col < BOARD_SIZE; col++) {
        if (board[row][col] !== null) continue
        if (!isNearExistingMove(board, row, col)) continue
        threats.push({ score: 0, position: { row, col } })
      }
    }
  }
  if (isMaximizingPlayer) {
    let best_score = -SCORE_5
    for (let { position } of threats) {
      setMove(board, 'O', position)
      let score = minimax(board, 'X', depth - 1, false, alpha, beta)
      undo(board, position)
      best_score = Math.max(best_score, score)
      alpha = Math.max(alpha, score)
      if (beta <= alpha) {
        break
      }
    }
    hashTable.set(str, best_score)
    return best_score
  } else {
    let best_score = SCORE_5
    for (let { position } of threats) {
      setMove(board, 'X', position)
      let score = minimax(board, 'O', depth - 1, false, alpha, beta)
      undo(board, position)
      best_score = Math.min(best_score, score)
      alpha = Math.max(alpha, score)
      if (beta <= alpha) {
        break
      }
    }
    hashTable.set(str, best_score)
    return best_score
  }
}

export const findBotMove = (board: (Player | null)[][], depth: number) => {
  let best_score = -SCORE_5 - 1
  let best_moves: { row: number; col: number }[] = []
  let alpha = -SCORE_5
  let beta = SCORE_5

  // if (game.moves.length === 0) {
  //     return { row: Math.floor(BOARD_SIZE / 2), col: Math.floor(BOARD_SIZE / 2) }
  // }

  let { threats } = evaluate(board, 'O')
  if (threats.length === 0) {
    let neighbors: Threat[] = []
    for (let row = 0; row < BOARD_SIZE; row++) {
      for (let col = 0; col < BOARD_SIZE; col++) {
        if (board[row][col] !== null) continue
        if (!isNearExistingMove(board, row, col)) continue
        neighbors.push({
          score: Math.floor(Math.random() * 100),
          position: { row, col }
        })
      }
    }
    neighbors.sort((a, b) => b.score - a.score)
    threats = neighbors
  }

  if (threats.length === 0) {
    return { row: Math.floor(BOARD_SIZE / 2), col: Math.floor(BOARD_SIZE / 2) }
  }

  threats.forEach(({ position }) => {
    setMove(board, 'O', position)
    let score = minimax(board, 'X', depth, false, alpha, beta)
    if (score > best_score) {
      best_score = score
      best_moves = [position]
    } else if (score === best_score) {
      best_moves.push(position)
    }
    undo(board, position)
  })

  if (best_moves.length === 0) {
    return null
  }
  let idx = Math.floor(Math.random() * best_moves.length)
  return best_moves[idx]
}
