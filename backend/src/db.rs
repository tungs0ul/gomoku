use crate::models::{Game, GameDb, Move};
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

    pub async fn new_game(&self, game: &Game, room_id: &Uuid) -> Result<()> {
        sqlx::query!(
            "INSERT INTO game (id, room, x, o) VALUES ($1, $2, $3, $4)",
            game.id,
            room_id,
            game.x,
            game.o
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_active_game_for_room(&self, room_id: &Uuid) -> Result<Game> {
        let game = sqlx::query_as!(
            GameDb,
            r#"
            SELECT
                g.id,
                g.x,
                g.o,
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
            where g.room = $1 and g.status != 'ended'
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

    pub async fn end_game(&self, id: Uuid) -> Result<()> {
        sqlx::query!("update game set status = 'ended' where id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

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
