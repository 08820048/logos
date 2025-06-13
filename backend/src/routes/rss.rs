use axum::{extract::State, response::{IntoResponse, Response}, http::{header, StatusCode}};
use chrono::Utc;
use rss::{ChannelBuilder, ItemBuilder};
use std::env;
use utoipa::ToResponse;

use crate::routes::AppState;

/// 生成 RSS Feed
///
/// 生成博客的 RSS 订阅源
#[utoipa::path(
    get,
    path = "/rss.xml",
    tag = "rss",
    responses(
        (status = 200, description = "RSS订阅源生成成功", content_type = "application/rss+xml"),
        (status = 500, description = "服务器内部错误", body = crate::utils::error::AppError)
    )
)]
pub async fn rss_handler(
    State(state): State<AppState>,
) -> Result<Response, crate::utils::error::AppError> {
    // 获取最新的文章列表
    let (posts, _) = state.post_service.list_posts(1, 20, true).await?;
    
    // 获取博客配置
    let blog_title = env::var("BLOG_TITLE").unwrap_or_else(|_| "Logos Blog".to_string());
    let blog_description = env::var("BLOG_DESCRIPTION").unwrap_or_else(|_| "A lightweight, high-performance blog system".to_string());
    let blog_link = env::var("BLOG_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    
    // 创建 RSS Channel
    let mut channel_builder = ChannelBuilder::default();
    channel_builder
        .title(blog_title)
        .link(blog_link.clone())
        .description(blog_description)
        .language(Some("zh-CN".to_string()))
        .last_build_date(Some(Utc::now().to_rfc2822()))
        .generator(Some("Logos Blog".to_string()));
    
    // 添加文章到 RSS
    for post_with_tags in posts {
        let post = post_with_tags.post;
        let tags = post_with_tags.tags;
        
        let post_url = format!("{}/post/{}", blog_link, post.slug);
        
        let mut item_builder = ItemBuilder::default();
        item_builder
            .title(Some(post.title))
            .link(Some(post_url))
            .description(Some(post.summary))
            .content(Some(post.content_md))
            .pub_date(Some(post.created_at.to_rfc2822()));
        
        // 添加标签
        for tag in tags {
            item_builder.category(
                rss::CategoryBuilder::default()
                    .name(tag.name)
                    .build(),
            );
        }
        
        channel_builder.item(item_builder.build());
    }
    
    // 构建 RSS
    let channel = channel_builder.build();
    let rss_content = channel.to_string();
    
    // 返回 RSS
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")],
        rss_content,
    ).into_response())
}
