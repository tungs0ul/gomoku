use anyhow::Result;
use axum::Router;
use backend::{
    api,
    auth::{Claims, UserMetadata},
};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tokio::net::TcpListener;
use uuid::Uuid;

static JWT_SECRET: &str = "jwt_secret";

pub fn generate_access_token() -> String {
    let exp = chrono::Utc::now() + chrono::Duration::hours(1);
    let claims = Claims {
        sub: Uuid::new_v4(),
        exp: exp.timestamp() as usize,
        user_metadata: UserMetadata {
            avatar_url: None,
            name: None,
        },
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .expect("Failed to encode token")
}

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
    let app = api::app(pool.clone(), JWT_SECRET);
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Database migration failed");
    let listener = TcpListener::bind("0.0.0.0:0").await.expect("bind failed");
    Ok((pool, app, listener))
}
