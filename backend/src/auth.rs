use {
    axum::{
        async_trait,
        extract::{FromRef, FromRequestParts},
        http::{request::Parts, StatusCode},
        response::{IntoResponse, Response},
        Json, RequestPartsExt,
    },
    axum_extra::{
        headers::{authorization::Bearer, Authorization},
        TypedHeader,
    },
    jsonwebtoken::{decode, DecodingKey, Validation},
    serde::{Deserialize, Serialize},
    serde_json::json,
    std::{fmt::Display, sync::Arc},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserMetadata {
    pub avatar_url: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub user_metadata: UserMetadata,
}

#[derive(Debug)]
pub enum AuthError {
    // MissingPermission,
    // WrongCredentials,
    // MissingCredentials,
    // TokenCreation,
    InvalidToken,
    // StateReadError,
}

/*
 * ERROR HANDLING
 */

#[derive(Debug)]
pub enum AppError {
    InternalServerError(anyhow::Error),
    AuthError(AuthError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::InternalServerError(error) => {
                tracing::error!(?error, "Internal server error");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AppError::AuthError(err) => err.into_response(),
        }
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>`
// to turn them into `Result<_, AppError>`.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::InternalServerError(err.into())
    }
}

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        Self::AuthError(err)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            // AuthError::MissingPermission => (StatusCode::UNAUTHORIZED, "Missing permission"),
            // AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            // AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            // AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            // AuthError::StateReadError => {
            // (StatusCode::INTERNAL_SERVER_ERROR, "Token validation error")
            // }
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user_id: {}", self.sub)
    }
}

pub trait DecodingKeyProvider {
    fn decoding_key(&self) -> &DecodingKey;
}

impl<T: DecodingKeyProvider> DecodingKeyProvider for Arc<T> {
    fn decoding_key(&self) -> &DecodingKey {
        (**self).decoding_key()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: FromRef<S> + DecodingKeyProvider + Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let mut validation = Validation::default();
        validation.set_audience(&["authenticated"]);
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), state.decoding_key(), &validation)
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
