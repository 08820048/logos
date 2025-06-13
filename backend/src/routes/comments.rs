use axum::{
    extract::{Path, Query, State, ConnectInfo},
    http::StatusCode,
    Json,
};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    entities::comment,
    routes::AppState,
    services::comment_service::{CreateCommentDto, UpdateCommentStatusDto},
    utils::auth::AuthUser,
    middleware::rate_limit::{RateLimiter, RateLimitConfig},
};

#[derive(Debug, Deserialize, IntoParams)]
pub struct CommentQuery {
    /// 页码，默认为1
    pub page: Option<u64>,
    /// 每页条数，默认为10
    pub limit: Option<u64>,
    /// 评论状态过滤，可选值：pending, approved, rejected
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCommentRequest {
    /// 评论者昵称，可选，默认为"匿名"
    pub nickname: Option<String>,
    /// 评论者邮箱
    pub email: String,
    /// 评论内容
    pub content: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCommentStatusRequest {
    /// 评论状态，可选值：pending, approved, rejected
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CommentsResponse {
    /// 评论列表
    pub comments: Vec<comment::Model>,
    /// 评论总数
    pub total: u64,
    /// 当前页码
    pub page: u64,
    /// 每页条数
    pub limit: u64,
    /// 总页数
    pub total_pages: u64,
}

/// 获取文章评论列表
///
/// 返回指定文章的评论列表，可通过查询参数进行分页和状态过滤
#[utoipa::path(
    get,
    path = "/api/posts/{post_id}/comments",
    tag = "comments",
    params(
        ("post_id" = i32, Path, description = "文章ID"),
        CommentQuery
    ),
    responses(
        (status = 200, description = "成功获取评论列表", body = CommentsResponse),
        (status = 404, description = "文章不存在", body = crate::utils::error::AppError),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    )
)]
pub async fn list_comments_handler(
    State(state): State<AppState>,
    Path(post_id): Path<i32>,
    Query(query): Query<CommentQuery>,
) -> Result<Json<CommentsResponse>, crate::utils::error::AppError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let status = query.status.as_deref();

    let (comments, total) = state
        .comment_service
        .list_comments(post_id, page, limit, status.is_some() && status.unwrap() == "approved")
        .await?;
    let total_pages = (total + limit - 1) / limit;

    Ok(Json(CommentsResponse {
        comments,
        total,
        page,
        limit,
        total_pages,
    }))
}

/// 创建评论
///
/// 为指定文章创建新评论
#[utoipa::path(
    post,
    path = "/api/posts/{post_id}/comments",
    tag = "comments",
    params(
        ("post_id" = i32, Path, description = "文章ID")
    ),
    request_body = CreateCommentRequest,
    responses(
        (status = 200, description = "评论创建成功", body = comment::Model),
        (status = 404, description = "文章不存在", body = crate::utils::error::AppError),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    )
)]
pub async fn create_comment_handler(
    State(state): State<AppState>,
    Path(post_id): Path<i32>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<CreateCommentRequest>,
) -> Result<Json<comment::Model>, crate::utils::error::AppError> {
    // 获取全局限流器
    let config = RateLimitConfig {
        window_seconds: 60,  // 1分钟窗口
        max_requests: 5,     // 每分钟最多5个请求
    };
    let limiter = RateLimiter::global(config);
    
    // 检查IP是否应该被限流
    if limiter.should_limit(&addr.ip()) {
        return Err(crate::utils::error::AppError::BadRequest(
            "Too many requests, please try again later".to_string()
        ));
    }
    let nickname = req.nickname.unwrap_or_else(|| "匿名".to_string());

    let dto = CreateCommentDto {
        post_id,
        nickname,
        email: req.email,
        content: req.content,
    };

    let comment = state.comment_service.create_comment(dto).await?;
    Ok(Json(comment))
}

/// 获取所有评论（管理后台）
///
/// 返回所有评论列表，需要管理员权限
#[utoipa::path(
    get,
    path = "/api/comments",
    tag = "comments",
    params(
        CommentQuery
    ),
    responses(
        (status = 200, description = "成功获取评论列表", body = CommentsResponse),
        (status = 401, description = "未授权", body = crate::utils::error::AppError),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_all_comments_handler(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(query): Query<CommentQuery>,
) -> Result<Json<CommentsResponse>, crate::utils::error::AppError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let status = query.status.as_deref();

    let (comments, total) = state
        .comment_service
        .list_all_comments(page, limit, status)
        .await?;
    let total_pages = (total + limit - 1) / limit;

    Ok(Json(CommentsResponse {
        comments,
        total,
        page,
        limit,
        total_pages,
    }))
}

/// 更新评论状态
///
/// 更新指定评论的状态，需要管理员权限
#[utoipa::path(
    put,
    path = "/api/comments/{id}/status",
    tag = "comments",
    params(
        ("id" = i32, Path, description = "评论ID")
    ),
    request_body = UpdateCommentStatusRequest,
    responses(
        (status = 200, description = "评论状态更新成功", body = comment::Model),
        (status = 401, description = "未授权", body = crate::utils::error::AppError),
        (status = 404, description = "评论不存在", body = crate::utils::error::AppError),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_comment_status_handler(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i32>,
    Json(req): Json<UpdateCommentStatusRequest>,
) -> Result<Json<comment::Model>, crate::utils::error::AppError> {
    let comment = state.comment_service.update_comment_status(id, &req.status).await?;
    Ok(Json(comment))
}

/// 删除评论
///
/// 删除指定评论，需要管理员权限
#[utoipa::path(
    delete,
    path = "/api/comments/{id}",
    tag = "comments",
    params(
        ("id" = i32, Path, description = "评论ID")
    ),
    responses(
        (status = 200, description = "评论删除成功"),
        (status = 401, description = "未授权", body = crate::utils::error::AppError),
        (status = 404, description = "评论不存在", body = crate::utils::error::AppError),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_comment_handler(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<()>, crate::utils::error::AppError> {
    state.comment_service.delete_comment(id).await?;
    Ok(Json(()))
}
