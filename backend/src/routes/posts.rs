use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};


use crate::{
    routes::AppState,
    services::post_service::{CreatePostDto, PostWithTags, UpdatePostDto},
    utils::auth::AuthUser,
};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PostQuery {
    /// 页码，默认为1
    pub page: Option<u64>,
    /// 每页条数，默认为10
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PostsResponse {
    /// 文章列表
    pub posts: Vec<PostWithTags>,
    /// 文章总数
    pub total: u64,
    /// 当前页码
    pub page: u64,
    /// 每页条数
    pub limit: u64,
    /// 总页数
    pub total_pages: u64,
}

/// 获取文章列表
/// 
/// 返回分页的文章列表，可通过查询参数指定页码和每页条数
#[utoipa::path(
    get,
    path = "/api/posts",
    params(PostQuery),
    responses(
        (status = 200, description = "成功获取文章列表", body = PostsResponse),
        (status = 500, description = "服务器内部错误", body = AppError)
    ),
    tag = "posts"
)]
pub async fn list_posts_handler(
    State(state): State<AppState>,
    Query(query): Query<PostQuery>,
) -> Result<Json<PostsResponse>, crate::utils::error::AppError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);

    let (posts, total) = state.post_service.list_posts(page, limit, true).await?;
    let total_pages = (total + limit - 1) / limit;

    Ok(Json(PostsResponse {
        posts,
        total,
        page,
        limit,
        total_pages,
    }))
}

/// 获取文章详情
/// 
/// 根据ID获取文章详情
#[utoipa::path(
    get,
    path = "/api/posts/{id}",
    params(
        ("id" = i32, Path, description = "文章ID")
    ),
    responses(
        (status = 200, description = "成功获取文章详情", body = PostWithTags),
        (status = 404, description = "文章不存在", body = AppError),
        (status = 500, description = "服务器内部错误", body = AppError)
    ),
    tag = "posts"
)]
pub async fn get_post_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<PostWithTags>, crate::utils::error::AppError> {
    let post = state.post_service.get_post(id).await?;
    Ok(Json(post))
}

/// 创建文章
/// 
/// 创建新的博客文章，需要管理员权限
#[utoipa::path(
    post,
    path = "/api/posts",
    request_body = CreatePostDto,
    responses(
        (status = 200, description = "文章创建成功", body = PostWithTags),
        (status = 401, description = "未授权", body = AppError),
        (status = 500, description = "服务器内部错误", body = AppError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "posts"
)]
#[axum::debug_handler]
pub async fn create_post_handler(
    State(state): State<AppState>,
    _auth: AuthUser,
    Json(dto): Json<CreatePostDto>,
) -> Result<Json<PostWithTags>, crate::utils::error::AppError> {
    let post = state.post_service.create_post(dto).await?;
    Ok(Json(post))
}

/// 更新文章
/// 
/// 更新指定ID的文章，需要管理员权限
#[utoipa::path(
    put,
    path = "/api/posts/{id}",
    params(
        ("id" = i32, Path, description = "文章ID")
    ),
    request_body = UpdatePostDto,
    responses(
        (status = 200, description = "文章更新成功", body = PostWithTags),
        (status = 401, description = "未授权", body = AppError),
        (status = 404, description = "文章不存在", body = AppError),
        (status = 500, description = "服务器内部错误", body = AppError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "posts"
)]
#[axum::debug_handler]
pub async fn update_post_handler(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i32>,
    Json(dto): Json<UpdatePostDto>,
) -> Result<Json<PostWithTags>, crate::utils::error::AppError> {
    let post = state.post_service.update_post(id, dto).await?;
    Ok(Json(post))
}

/// 删除文章
/// 
/// 删除指定ID的文章，需要管理员权限
#[utoipa::path(
    delete,
    path = "/api/posts/{id}",
    params(
        ("id" = i32, Path, description = "文章ID")
    ),
    responses(
        (status = 200, description = "文章删除成功"),
        (status = 401, description = "未授权", body = AppError),
        (status = 404, description = "文章不存在", body = AppError),
        (status = 500, description = "服务器内部错误", body = AppError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "posts"
)]
pub async fn delete_post_handler(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<()>, crate::utils::error::AppError> {
    state.post_service.delete_post(id).await?;
    Ok(Json(()))
}
