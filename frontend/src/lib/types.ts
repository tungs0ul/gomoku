export type Player = 'x' | 'o'
export type GameType = 'normal' | 'bot' | 'private'
export type GameStatus = 'ready' | 'playing' | 'ended'
export type Position = {
  row: number
  col: number
}
export type Move = {
  position: Position
  player: Player
}

export type Board = (Player | null)[][]

export type Game = {
  id: string
  board: Board
  x: string | null
  o: string | null
  next_player: Player
  winner: Move[] | null
  moves: Move[]
  transposition_table: Record<string, number>
  room_id: string
  game_type: GameType
  status: GameStatus
  x_ready: boolean
  o_ready: boolean
}

export type GameEvent =
  | {
    event: 'Game'
    game: Game
  }
  | {
    event: 'MiniMax'
    position: Position
    score: number
  }
  | {
    event: 'MoveEvent'
    mv: Move
  }
  | {
    event: 'InvalidMove'
    player: Player
  }
  | {
    event: 'Winner'
    moves: Move[]
    last_move: Move
  }
  | {
    event: 'PlayAgain'
  }
  | ({
    event: 'Message'
  } & Message)
  | { event: "Status", status: GameStatus }
  | { event: "PlayerLeft" }

export type Message = {
  msg: string
  user: string | null
  id: string
}
