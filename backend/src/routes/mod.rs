use axum::{
    Router,
    http::Method,
    routing::MethodFilter,
    routing::{get, post, delete, put, on},
    middleware,
};
use std::sync::Arc;

use crate::services::{
    comment_service::CommentService,
    link_service::LinkService,
    post_service::PostService,
    search_service::SearchService,
    tag_service::TagService,
    user_service::UserService,
};

// 将模块公开以便API文档生成
pub mod auth;
pub mod comments;
pub mod links;
pub mod posts;
pub mod search;
pub mod tags;
pub mod rss;

pub fn create_router(
    user_service: Arc<UserService>,
    post_service: Arc<PostService>,
    tag_service: Arc<TagService>,
    comment_service: Arc<CommentService>,
    link_service: Arc<LinkService>,
    search_service: Arc<SearchService>,
) -> Router {
    Router::new()
        // 认证路由
        .route("/api/login", post(auth::login_handler))
        
        // 文章路由
        .route("/api/posts", get(posts::list_posts_handler))
        .route("/api/posts", on(MethodFilter::POST, posts::create_post_handler))
        .route("/api/posts/:id", get(posts::get_post_handler))
        .route("/api/posts/:id", on(MethodFilter::PUT, posts::update_post_handler))
        .route("/api/posts/:id", delete(posts::delete_post_handler))
        
        // 标签路由
        .route("/api/tags", get(tags::list_tags_handler))
        .route("/api/tags", post(tags::create_tag_handler))
        .route("/api/tags/:id", delete(tags::delete_tag_handler))
        .route("/api/tags/:id/posts", get(tags::get_tag_posts_handler))
        
        // 评论路由
        .route("/api/posts/:id/comments", get(comments::list_comments_handler))
        .route("/api/posts/:id/comments", post(comments::create_comment_handler))
        .route("/api/comments", get(comments::list_all_comments_handler))
        .route("/api/comments/:id/status", put(comments::update_comment_status_handler))
        .route("/api/comments/:id", delete(comments::delete_comment_handler))
        
        // 友情链接路由
        .route("/api/links", get(links::list_links_handler))
        .route("/api/links", post(links::create_link_handler))
        .route("/api/links/:id", put(links::update_link_handler))
        .route("/api/links/:id", delete(links::delete_link_handler))
        
        // 搜索路由
        .route("/api/search", get(search::search_handler))
        
        // RSS 路由
        .route("/rss.xml", get(rss::rss_handler))
        
        // 注入服务
        .with_state(AppState {
            user_service,
            post_service,
            tag_service,
            comment_service,
            link_service,
            search_service,
        })
}

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
    pub post_service: Arc<PostService>,
    pub tag_service: Arc<TagService>,
    pub comment_service: Arc<CommentService>,
    pub link_service: Arc<LinkService>,
    pub search_service: Arc<SearchService>,
}
