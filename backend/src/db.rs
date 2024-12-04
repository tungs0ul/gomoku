use crate::models::{Game, GameDb, Move, Player, RoomType};
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

    pub async fn get_room_type(&self, room_id: &Uuid) -> Result<RoomType> {
        let room_type = sqlx::query_scalar!(
            r#"select room_type as "room_type: RoomType" from room where id = $1"#,
            room_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(room_type)
    }
    pub async fn create_room(
        &self,
        room_id: &Uuid,
        room_type: &RoomType,
        game: &Game,
    ) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            r#"insert into room(id, room_type) values ($1, $2)"#,
            room_id,
            room_type as _
        )
        .execute(&mut *tx)
        .await?;
        sqlx::query!(
            "INSERT INTO game (id, room_id, x, o, init_player) VALUES ($1, $2, $3, $4, $5)",
            game.id,
            room_id,
            game.x,
            game.o,
            game.next_player as _
        )
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn new_game(&self, game: &Game, room_id: &Uuid, init_player: &Player) -> Result<()> {
        sqlx::query!(
            "INSERT INTO game (id, room_id, x, o, init_player) VALUES ($1, $2, $3, $4, $5)",
            game.id,
            room_id,
            game.x,
            game.o,
            init_player as _
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_game_winner(&self, game: &Game) -> Result<()> {
        sqlx::query!(
            r#"update game set winner = $2 where id = $1"#,
            game.id,
            serde_json::json!(game.winner)
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_active_game_for_rooms(
        &self,
        room_ids: &[Uuid],
    ) -> Result<Vec<(Uuid, Game, RoomType)>> {
        let games = sqlx::query_as!(
            GameDb,
            r#"
            SELECT
                g.room_id,
                g.id,
                r.room_type as "room_type: RoomType",
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
            join room r
                on r.id = g.room_id
            LEFT JOIN
                game_move gm
                ON g.id = gm.game_id
            where g.room_id IN (SELECT unnest($1::uuid[])) and g.status != 'ended'
            GROUP BY
                g.id, r.room_type;
        "#,
            room_ids,
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .filter_map(|game| {
            let room = game.room_id;
            let room_type = game.room_type.clone();
            let game = Game::try_from(game);
            match game {
                Ok(game) => Some((room, game, room_type)),
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
                g.winner,
                r.room_type as "room_type: RoomType",
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
            join room r
                on r.id = g.room_id
            LEFT JOIN
                game_move gm
                ON g.id = gm.game_id
            where g.room_id = $1 and g.status != 'ended'
            GROUP BY
                g.id, r.room_type;
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

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "game_status", rename_all = "lowercase")]
pub enum GameStatus {
    Ready,
    Playing,
    Ended,
}
