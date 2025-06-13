use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::routes::AppState;

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    /// JWT 认证令牌
    pub token: String,
}

/// 用户登录处理
/// 
/// 用户登录并获取JWT令牌
#[utoipa::path(
    post,
    path = "/api/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "登录成功", body = LoginResponse),
        (status = 401, description = "用户名或密码错误", body = AppError),
        (status = 500, description = "服务器内部错误", body = AppError)
    ),
    tag = "auth"
)]
pub async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, crate::utils::error::AppError> {
    let token = state.user_service.login(&req.username, &req.password).await?;
    
    Ok(Json(LoginResponse { token }))
}
