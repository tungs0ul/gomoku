use {
    backend::api,
    opentelemetry::global,
    serde::Deserialize,
    sqlx::PgPool,
    tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt},
};

lazy_static::lazy_static! {
    static ref CONFIG: Config = envy::prefixed("BACKEND_").from_env::<Config>().unwrap();
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let jaeger_url = std::env::var("BACKEND_JAEGER_URL").unwrap_or("jaeger:6831".to_string());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("backend")
        .with_agent_endpoint(jaeger_url)
        .install_simple()
        .expect("Error connecting jaeger");
    tracing::info!(?tracer);
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(opentelemetry)
        .with(
            fmt::Layer::default()
                .with_file(true)
                .with_line_number(true)
                .with_target(true)
                .compact()
                .with_level(true),
        )
        .try_init()
        .expect("");
    tracing::info!("Listening on port 11211");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:11211")
        .await
        .expect("Can't bind TCP socket");
    let pool = PgPool::connect_lazy(&CONFIG.database_url).expect("Can't connect to database");
    if let Err(error) = sqlx::migrate!("./migrations").run(&pool).await {
        tracing::error!(?error);
    }
    axum::serve(listener, api::app(pool, &CONFIG.jwt_secret))
        .await
        .expect("Failed to run server");
}
