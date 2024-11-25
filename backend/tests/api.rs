mod common;

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    // use http_body_util::BodyExt;
    use crate::common;

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
                    .uri("/health_check")
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

        let user_id = Uuid::new_v4();

        let client = reqwest::Client::new();
        let res = client
            .get(format!("http://{addr}/game/bot/{user_id}"))
            .send()
            .await;
        assert!(res.is_ok());
        let res = res.unwrap().text().await;
        assert!(res.is_ok());
        let id = Uuid::parse_str(res.unwrap().as_str());
        assert!(id.is_ok());
    }
}
