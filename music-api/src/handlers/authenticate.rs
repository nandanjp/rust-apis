use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, HeaderMap, StatusCode},
    response::IntoResponse,
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{config::Config, AppState};

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = Config::new()
        .expect("failed to parse the configuration stuff")
        .jwt_token;
    Keys::new(secret.as_bytes())
});

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    TokenCreation,
    InvalidToken,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        let token_data =
            match decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default()) {
                Ok(token) => token.claims,
                Err(e) => {
                    tracing::debug!("Failed to parse the jwt token: {}", e);
                    println!("Failed to parse the jwt token: {}", e);
                    return Err(AuthError::InvalidToken);
                }
            };

        Ok(token_data)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

impl std::fmt::Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token: {}\nExpires In: {}", self.sub, self.company)
    }
}

pub async fn authorize(State(app): State<AppState>) -> Result<impl IntoResponse, AuthError> {
    let res = Client::new()
        .post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(
            format!(
                "grant_type=client_credentials&client_id={}&client_secret={}",
                app.client_id, app.client_secret
            )
            .as_str()
            .to_owned(),
        )
        .send()
        .await;

    match res {
        Ok(res) => match res.json::<AuthResponse>().await {
            Ok(res) => {
                let mut headers = HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());

                let claims = Claims {
                    sub: res.access_token.clone(),
                    company: res.access_token.clone(),
                    exp: 2400000000,
                };

                let token = encode(&Header::default(), &claims, &KEYS.encoding)
                    .map_err(|_| AuthError::TokenCreation)?;

                headers.insert("Authorization", token.clone().as_str().parse().unwrap());
                Ok((headers, Json(claims)))
            }
            Err(_) => Err(AuthError::InvalidToken),
        },
        Err(_) => Err(AuthError::WrongCredentials),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplicitContent {
    filter_enabled: bool,
    filter_locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalUrls {
    spotify: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Followers {
    href: String,
    total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    url: String,
    height: usize,
    width: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    country: String,
    display_name: String,
    email: String,
    explicit_content: ExplicitContent,
    external_urls: ExternalUrls,
    followers: Followers,
    href: String,
    id: String,
    images: Vec<Image>,
    product: String,
    #[serde(rename = "type")]
    type_: String,
    uri: String,
}
pub async fn get_user(token: Claims) -> Result<impl IntoResponse, AuthError> {
    let res = Client::new()
        .get("https://api.spotify.com/v1/me")
        .header("Authorization", format!("Bearer {}", token.sub.clone()))
        .send()
        .await
        .map_err(|_| AuthError::WrongCredentials)?;
    println!("res = {:#?}", res);

    let text = res.text().await.map_err(|_| AuthError::InvalidToken)?;
    println!("res.body: {}", text);

    let res = Client::new()
        .get("https://api.spotify.com/v1/me")
        .header("Authorization", format!("Bearer {}", token.sub))
        .send()
        .await
        .map_err(|_| AuthError::WrongCredentials)?;
    match res.json::<User>().await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => {
            println!("error in unwrapping = {e}");
            Err(AuthError::InvalidToken)
        }
    }
}
