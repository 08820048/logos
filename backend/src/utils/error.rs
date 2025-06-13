use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug, ToSchema)]
pub enum AppError {
    /// 认证失败错误
    #[error("认证失败: {0}")]
    #[schema(value_type = String, example = json!("认证失败: 无效的用户名或密码"))]
    AuthError(String),
    
    /// 数据库错误
    #[error("数据库错误: {0}")]
    #[schema(value_type = String)]
    DbError(#[from] sea_orm::DbErr),
    
    /// 输入验证错误
    #[error("验证错误: {0}")]
    ValidationError(String),
    
    /// 资源不存在错误
    #[error("资源不存在: {0}")]
    NotFound(String),
    
    /// 请求参数错误
    #[error("请求错误: {0}")]
    BadRequest(String),
    
    /// 服务器内部错误
    #[error("内部服务器错误: {0}")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::AuthError(message) => (StatusCode::UNAUTHORIZED, message),
            AppError::DbError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::ValidationError(message) => (StatusCode::BAD_REQUEST, message),
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            AppError::InternalServerError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
