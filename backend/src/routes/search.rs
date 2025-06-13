use axum::{extract::{Query, State}, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{routes::AppState, services::post_service::PostWithTags};

#[derive(Debug, Deserialize, ToSchema)]
pub struct SearchQuery {
    /// 搜索关键词
    pub q: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SearchResponse {
    /// 搜索结果文章列表
    pub posts: Vec<PostWithTags>,
    /// 搜索关键词
    pub query: String,
    /// 搜索结果数量
    pub count: usize,
}

/// 搜索文章
///
/// 根据关键词搜索文章
#[utoipa::path(
    get,
    path = "/api/search",
    tag = "search",
    params(
        ("q" = String, Query, description = "搜索关键词")
    ),
    responses(
        (status = 200, description = "搜索成功", body = SearchResponse),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    )
)]
pub async fn search_handler(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, crate::utils::error::AppError> {
    let posts = state.search_service.search_posts(&query.q, true).await?;
    
    Ok(Json(SearchResponse {
        posts: posts.clone(),
        query: query.q,
        count: posts.len(),
    }))
}
