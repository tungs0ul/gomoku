mod common;

#[cfg(test)]
mod tests {
    // use http_body_util::BodyExt;
    use crate::common::{self, generate_access_token};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };

    use backend::{
        api::{GamePayload, GameResponse},
        models::GameType,
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_healthcheck() {
        let app = common::spawn_router().await;
        assert!(app.is_ok());
        let (_pool, router, _listener) = app.unwrap();
        let response = router
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_bot_game() {
        let app = common::spawn_router().await;
        assert!(app.is_ok());
        let (_pool, router, listener) = app.unwrap();

        let addr = listener.local_addr().unwrap().to_string();

        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        let payload = GamePayload {
            game_type: GameType::Bot,
        };
        let client = reqwest::Client::new();
        let token = generate_access_token();
        let res = client
            .post(format!("http://{addr}/api/games"))
            .bearer_auth(token)
            .json(&payload)
            .send()
            .await;

        assert!(res.is_ok());
        let res = res.unwrap().json::<GameResponse>().await;
        assert!(res.is_ok());
    }
}
