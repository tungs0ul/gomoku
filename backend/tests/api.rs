mod common;

#[cfg(test)]
mod tests {
    // use http_body_util::BodyExt;
    use crate::common;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };

    use backend::{api::GamePayload, models::GameType};
    use tower::ServiceExt;
    use uuid::Uuid;

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
            user_id: Uuid::new_v4(),
        };
        let client = reqwest::Client::new();
        let res = client
            .post(format!("http://{addr}/api/games"))
            .json(&payload)
            .send()
            .await;
        assert!(res.is_ok());
        let res = res.unwrap().text().await;
        assert!(res.is_ok());
        let res = res.unwrap();
        let res: Vec<&str> = res.split('/').collect();
        match res[..] {
            ["", "ws", "rooms", a, "users", b] => {
                let a = Uuid::parse_str(a);
                let b = Uuid::parse_str(b);
                assert!(a.is_ok());
                assert!(b.is_ok());
            }
            _ => {
                panic!("Failed");
            }
        }
    }
}
