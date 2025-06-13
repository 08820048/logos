use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    entities::link,
    routes::AppState,
    services::link_service::{CreateLinkDto, UpdateLinkDto},
    utils::auth::AuthUser,
};

#[derive(Debug, Deserialize, IntoParams)]
pub struct LinkQuery {
    /// 页码，默认为1
    pub page: Option<u64>,
    /// 每页条数，默认为10
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateLinkRequest {
    /// 友情链接名称
    pub name: String,
    /// 友情链接URL
    pub url: String,
    /// 友情链接描述
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateLinkRequest {
    /// 友情链接名称
    pub name: Option<String>,
    /// 友情链接URL
    pub url: Option<String>,
    /// 友情链接描述
    pub description: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LinksResponse {
    /// 友情链接列表
    pub links: Vec<link::Model>,
    /// 友情链接总数
    pub total: u64,
    /// 当前页码
    pub page: u64,
    /// 每页条数
    pub limit: u64,
    /// 总页数
    pub total_pages: u64,
}

/// 获取所有友情链接
///
/// 返回所有友情链接列表，可通过查询参数进行分页
#[utoipa::path(
    get,
    path = "/api/links",
    tag = "links",
    params(
        LinkQuery
    ),
    responses(
        (status = 200, description = "成功获取友情链接列表", body = LinksResponse),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    )
)]
pub async fn list_links_handler(
    State(state): State<AppState>,
    Query(query): Query<LinkQuery>,
) -> Result<Json<LinksResponse>, crate::utils::error::AppError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);

    let (links, total) = state.link_service.list_links(page, limit).await?;
    let total_pages = (total + limit - 1) / limit;

    Ok(Json(LinksResponse {
        links,
        total,
        page,
        limit,
        total_pages,
    }))
}

/// 创建友情链接
///
/// 创建新的友情链接，需要管理员权限
#[utoipa::path(
    post,
    path = "/api/links",
    tag = "links",
    request_body = CreateLinkRequest,
    responses(
        (status = 200, description = "友情链接创建成功", body = link::Model),
        (status = 401, description = "未授权", body = crate::utils::error::AppError),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_link_handler(
    State(state): State<AppState>,
    _auth: AuthUser,
    Json(req): Json<CreateLinkRequest>,
) -> Result<Json<link::Model>, crate::utils::error::AppError> {
    let dto = CreateLinkDto {
        name: req.name,
        url: req.url,
        description: req.description,
    };

    let link = state.link_service.create_link(dto).await?;
    Ok(Json(link))
}

/// 更新友情链接
///
/// 更新指定友情链接，需要管理员权限
#[utoipa::path(
    put,
    path = "/api/links/{id}",
    tag = "links",
    params(
        ("id" = i32, Path, description = "友情链接ID")
    ),
    request_body = UpdateLinkRequest,
    responses(
        (status = 200, description = "友情链接更新成功", body = link::Model),
        (status = 401, description = "未授权", body = crate::utils::error::AppError),
        (status = 404, description = "友情链接不存在", body = crate::utils::error::AppError),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_link_handler(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i32>,
    Json(req): Json<UpdateLinkRequest>,
) -> Result<Json<link::Model>, crate::utils::error::AppError> {
    let dto = UpdateLinkDto {
        name: req.name,
        url: req.url,
        description: req.description,
    };

    let link = state.link_service.update_link(id, dto).await?;
    Ok(Json(link))
}

/// 删除友情链接
///
/// 删除指定友情链接，需要管理员权限
#[utoipa::path(
    delete,
    path = "/api/links/{id}",
    tag = "links",
    params(
        ("id" = i32, Path, description = "友情链接ID")
    ),
    responses(
        (status = 200, description = "友情链接删除成功"),
        (status = 401, description = "未授权", body = crate::utils::error::AppError),
        (status = 404, description = "友情链接不存在", body = crate::utils::error::AppError),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_link_handler(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<()>, crate::utils::error::AppError> {
    state.link_service.delete_link(id).await?;
    Ok(Json(()))
}
