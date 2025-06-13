use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::error::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(username: &str) -> Self {
        let now = Utc::now();
        let expires_at = now + Duration::hours(24);
        
        Self {
            sub: username.to_string(),
            iat: now.timestamp(),
            exp: expires_at.timestamp(),
        }
    }
}

pub fn create_token(username: &str) -> AppResult<String> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims::new(username);
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::AuthError(format!("无法创建令牌: {}", e)))
}

pub fn validate_token(token: &str) -> AppResult<Claims> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::AuthError(format!("无效的令牌: {}", e)))
}

pub struct AuthUser {
    pub username: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 从请求头中提取 Authorization 头
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::AuthError("缺少授权头".to_string()))?;

        // 验证令牌
        let claims = validate_token(bearer.token())?;

        Ok(AuthUser {
            username: claims.sub,
        })
    }
}
