use crate::models::{Game, GameDb, GameStatus, GameType, Move, Player};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct Db {
    pub pool: sqlx::PgPool,
}

impl Db {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn new_game(&self, game: &Game) -> Result<()> {
        sqlx::query!(
            "INSERT INTO game (id, room_id, x, o, init_player, game_type, status) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            game.id,
            game.room_id,
            game.x,
            game.o,
            game.next_player as _,
            game.game_type as _,
            game.status as _
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_game(&self, game: &Game) -> Result<()> {
        sqlx::query!(
            r#"update game set winner = $2, x = $3, o = $4, status = $5, x_ready = $6, o_ready = $7 where id = $1"#,
            game.id,
            serde_json::json!(game.winner),
            game.x,
            game.o,
            game.status as _,
            game.x_ready,
            game.o_ready,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_available_quick_games(&self, room_ids: &[Uuid]) -> Result<(Uuid, Game)> {
        let game = sqlx::query_as!(
            GameDb,
            r#"
            SELECT
                g.room_id,
                g.id,
                g.x_ready,
                g.o_ready,
                g.status as "status: GameStatus",
                g.game_type as "game_type: GameType",
                g.x,
                g.o,
                g.winner,
                g.init_player as "init_player: Player",
                jsonb_agg(
                    jsonb_build_object(
                        'row', gm.row,
                        'col', gm.col,
                        'player', gm.player
                    ) ORDER BY gm.turn
                ) AS moves
            FROM
                game g
            LEFT JOIN
                game_move gm
                ON g.id = gm.game_id
            where g.room_id IN (SELECT unnest($1::uuid[])) and g.status != 'ended'
            and ((g.x is null and g.o is not null) or (g.x is not null and g.o is null))
            GROUP BY
                g.id;
        "#,
            room_ids,
        )
        .fetch_one(&self.pool)
        .await?;
        let room_id = game.room_id;
        let game = Game::try_from(game)?;
        Ok((room_id, game))
    }

    pub async fn get_active_game_for_rooms(
        &self,
        room_ids: &[Uuid],
        game_types: &[GameType],
    ) -> Result<Vec<Game>> {
        let games = sqlx::query_as!(
            GameDb,
            r#"
            SELECT
                g.room_id,
                g.id,
                g.game_type as "game_type: GameType",
                g.x,
                g.x_ready,
                g.status as "status: GameStatus",
                g.o_ready,
                g.o,
                g.winner,
                g.init_player as "init_player: Player",
                jsonb_agg(
                    jsonb_build_object(
                        'row', gm.row,
                        'col', gm.col,
                        'player', gm.player
                    ) ORDER BY gm.turn
                ) AS moves
            FROM
                game g
            LEFT JOIN
                game_move gm
                ON g.id = gm.game_id
            where g.room_id IN (SELECT unnest($1::uuid[])) and g.status != 'ended'
            and g.game_type IN (select unnest($2::game_type[]))
            GROUP BY
                g.id;
        "#,
            room_ids,
            game_types as _
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .filter_map(|game| {
            let game = Game::try_from(game);
            match game {
                Ok(game) => Some(game),
                _ => None,
            }
        })
        .collect();
        Ok(games)
    }

    pub async fn get_active_game_for_room(&self, room_id: &Uuid) -> Result<Game> {
        let game = sqlx::query_as!(
            GameDb,
            r#"
            SELECT
                g.room_id,
                g.id,
                g.x,
                g.o,
                g.status as "status: GameStatus",
                g.x_ready,
                g.o_ready,
                g.winner,
                g.game_type as "game_type: GameType",
                g.init_player as "init_player: Player",
                jsonb_agg(
                    jsonb_build_object(
                        'row', gm.row,
                        'col', gm.col,
                        'player', gm.player
                    ) ORDER BY gm.turn
                ) AS moves
            FROM
                game g
            LEFT JOIN
                game_move gm
                ON g.id = gm.game_id
            where g.room_id = $1 and g.status != 'ended'
            GROUP BY
                g.id;
        "#,
            room_id,
        )
        .fetch_one(&self.pool)
        .await?;

        let game = Game::try_from(game)?;
        Ok(game)
    }

    // pub async fn end_game(&self, id: Uuid) -> Result<()> {
    //     sqlx::query!("update game set status = 'ended' where id = $1", id)
    //         .execute(&self.pool)
    //         .await?;
    //     Ok(())
    // }

    pub async fn insert_move(&self, game_id: &Uuid, mv: &Move, turn: usize) -> Result<()> {
        sqlx::query!(
            r#"insert into game_move(game_id, row, col, player, turn) values ($1, $2, $3, $4, $5)"#,
            game_id,
            mv.position.row as i32,
            mv.position.col as i32,
            mv.player as _,
            turn as i32
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
