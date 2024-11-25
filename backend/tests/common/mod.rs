use anyhow::Result;
use axum::Router;
use backend::api;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tokio::net::TcpListener;

pub async fn spawn_router() -> Result<(PgPool, Router, TcpListener)> {
    let db_name = uuid::Uuid::new_v4().to_string().replace('-', "");
    let test_db_name = format!("test_{db_name}");
    let mut db = PgConnection::connect("postgres://postgres:password@127.0.0.1:5432/postgres")
        .await
        .expect("Database connection failed");
    db.execute(format!("create database {}", test_db_name).as_str())
        .await
        .expect("Error creating database");
    let pool = PgPool::connect(
        format!("postgres://postgres:password@127.0.0.1:5432/{test_db_name}").as_str(),
    )
    .await
    .expect("Database connection failed");
    let app = api::app(pool.clone());
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .expect("Database migration failed");
    let listener = TcpListener::bind("0.0.0.0:0").await.expect("bind failed");
    Ok((pool, app, listener))
}
