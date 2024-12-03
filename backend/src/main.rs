use backend::api;
use serde::Deserialize;
use sqlx::PgPool;

lazy_static::lazy_static! {
    static ref CONFIG: Config = envy::prefixed("BACKEND_").from_env::<Config>().unwrap();
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("Listening on port 11211");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:11211")
        .await
        .expect("Can't bind TCP socket");
    let pool = PgPool::connect(&CONFIG.database_url).await.expect("Can't connect to database");
    if let Err(error) = sqlx::migrate!("./migrations")
        .run(&pool)
        .await {
        tracing::error!(?error);
    }
    axum::serve(listener, api::app(pool)).await.expect("Failed to run server");
}
