use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    entities::tag,
    routes::AppState,
    services::post_service::PostWithTags,
    utils::auth::AuthUser,
};

#[derive(Debug, Deserialize, IntoParams)]
pub struct TagQuery {
    /// 页码，默认为1
    pub page: Option<u64>,
    /// 每页条数，默认为10
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTagRequest {
    /// 标签名称
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TagPostsResponse {
    /// 标签信息
    pub tag: tag::Model,
    /// 标签下的文章列表
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

/// 获取所有标签
///
/// 返回所有标签列表
#[utoipa::path(
    get,
    path = "/api/tags",
    tag = "tags",
    responses(
        (status = 200, description = "成功获取标签列表", body = Vec<tag::Model>),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    )
)]
/// 获取所有标签
pub async fn list_tags_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<tag::Model>>, crate::utils::error::AppError> {
    let tags = state.tag_service.list_tags().await?;
    Ok(Json(tags))
}

/// 创建标签
///
/// 创建新标签，需要管理员权限
#[utoipa::path(
    post,
    path = "/api/tags",
    tag = "tags",
    request_body = CreateTagRequest,
    responses(
        (status = 200, description = "标签创建成功", body = tag::Model),
        (status = 401, description = "未授权", body = crate::utils::error::AppError),
        (status = 400, description = "标签名称已存在", body = crate::utils::error::AppError),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
/// 创建标签
pub async fn create_tag_handler(
    State(state): State<AppState>,
    _auth: AuthUser,
    Json(req): Json<CreateTagRequest>,
) -> Result<Json<tag::Model>, crate::utils::error::AppError> {
    let tag = state.tag_service.create_tag(&req.name).await?;
    Ok(Json(tag))
}

/// 删除标签
///
/// 删除指定标签，需要管理员权限
#[utoipa::path(
    delete,
    path = "/api/tags/{id}",
    tag = "tags",
    params(
        ("id" = i32, Path, description = "标签ID")
    ),
    responses(
        (status = 200, description = "标签删除成功"),
        (status = 401, description = "未授权", body = crate::utils::error::AppError),
        (status = 404, description = "标签不存在", body = crate::utils::error::AppError),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
/// 删除标签
pub async fn delete_tag_handler(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<()>, crate::utils::error::AppError> {
    state.tag_service.delete_tag(id).await?;
    Ok(Json(()))
}

/// 获取标签下的文章
///
/// 返回指定标签下的文章列表
#[utoipa::path(
    get,
    path = "/api/tags/{id}/posts",
    tag = "tags",
    params(
        ("id" = i32, Path, description = "标签ID"),
        ("page" = Option<u64>, Query, description = "页码，默认为1"),
        ("limit" = Option<u64>, Query, description = "每页条数，默认为10")
    ),
    responses(
        (status = 200, description = "成功获取标签下的文章列表", body = TagPostsResponse),
        (status = 404, description = "标签不存在", body = crate::utils::error::AppError),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    )
)]
/// 获取标签下的文章
pub async fn get_tag_posts_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Query(query): Query<TagQuery>,
) -> Result<Json<TagPostsResponse>, crate::utils::error::AppError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);

    let tag = state.tag_service.get_tag(id).await?;
    let (posts, total) = state.tag_service.get_tag_posts(id, page, limit, true).await?;
    let total_pages = (total + limit - 1) / limit;

    Ok(Json(TagPostsResponse {
        tag,
        posts,
        total,
        page,
        limit,
        total_pages,
    }))
}
