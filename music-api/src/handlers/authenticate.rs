/*
*curl -X POST "https://accounts.spotify.com/api/token" \
     -H "Content-Type: application/x-www-form-urlencoded" \
     -d "grant_type=client_credentials&client_id=your-client-id&client_secret=your-client-secret"
* */

/*
*{
  "access_token": "BQDBKJ5eo5jxbtpWjVOj7ryS84khybFpP_lTqzV7uV-T_m0cTfwvdn5BnBSKPxKgEb11",
  "token_type": "Bearer",
  "expires_in": 3600
}
* */
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
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{config::Config, AppState};

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    TokenCreation,
    InvalidToken,
}

#[async_trait]
impl<S> FromRequestParts<S> for Token
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // let config = Config::new().map_err(|_| AuthError::TokenCreation)?;
        // let jwt_secret = DecodingKey::from_secret(config.jwt_token.as_bytes());
        // let token_data =
        //     decode::<AuthResponse>(bearer.token(), &jwt_secret, &Validation::default())
        //         .map_err(|_| AuthError::InvalidToken)?;

        let key = DecodingKey::from_secret(&[]);
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.insecure_disable_signature_validation();
        let token = bearer.token();
        let token_data: AuthResponse =
            decode(&token, &key, &validation).map_err(|_| AuthError::InvalidToken)?;
        let response = token_data.claims;
        Ok(Self {
            expires_in: response.expires_in,
            token: response.access_token,
        })
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
pub struct Token {
    pub token: String,
    pub expires_in: u64,
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
                headers.insert(
                    "Authorization",
                    res.access_token.clone().as_str().parse().unwrap(),
                );

                let encoding = EncodingKey::from_secret(app.jwt_secret.as_bytes());
                let token = encode(&Header::default(), &res, &encoding)
                    .map_err(|_| AuthError::TokenCreation)?;
                Ok((
                    headers,
                    Json(Token {
                        expires_in: res.expires_in,
                        token,
                    }),
                ))
            }
            Err(_) => Err(AuthError::InvalidToken),
        },
        Err(_) => Err(AuthError::WrongCredentials),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    country: String,
    display_name: String,
    email: String,
    href: String,
    id: String,
    #[serde(rename = "type")]
    type_: String,
    uri: String,
}
pub async fn get_user(token: Token) -> Result<impl IntoResponse, AuthError> {
    println!("user = {:#?}", token.clone());
    let res = Client::new()
        .get("https://api.spotify.com/v1/me")
        .header("Authorization", format!("Bearer: {}", token.token).as_str())
        .send()
        .await;

    match res {
        Ok(res) => match res.json::<User>().await {
            Ok(res) => Ok((StatusCode::OK, Json(res))),
            Err(_) => Err(AuthError::InvalidToken),
        },
        Err(_) => Err(AuthError::WrongCredentials),
    }
}
