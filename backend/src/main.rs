mod db;
mod docs;
mod entities;
mod routes;
mod services;
mod utils;

use std::{net::SocketAddr, sync::Arc};

use axum::http::{header, Method};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    db::get_conn,
    services::{
        comment_service::CommentService,
        link_service::LinkService,
        post_service::PostService,
        search_service::SearchService,
        tag_service::TagService,
        user_service::UserService,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载环境变量
    dotenv().ok();

    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("正在启动 Logos 博客系统...");

    // 获取数据库连接
    let db = get_conn().await;

    // 运行数据库迁移
    info!("正在运行数据库迁移...");
    Migrator::up(&*db, None).await?;

    // 创建服务
    let user_service = Arc::new(UserService::new(db.clone()));
    let post_service = Arc::new(PostService::new(db.clone()));
    let tag_service = Arc::new(TagService::new(db.clone()));
    let comment_service = Arc::new(CommentService::new(db.clone()));
    let link_service = Arc::new(LinkService::new(db.clone()));
    let search_service = Arc::new(SearchService::new(db.clone()));

    // 初始化管理员用户
    let admin_username = std::env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let admin_password = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin".to_string());

    // 检查管理员用户是否存在，不存在则创建
    if user_service.find_by_username(&admin_username).await?.is_none() {
        info!("创建管理员用户: {}", admin_username);
        user_service.create_user(&admin_username, &admin_password).await?;
    }

    
    // 创建路由
    let app = routes::create_router(
        user_service,
        post_service,
        tag_service,
        comment_service,
        link_service,
        search_service,
    )
    // 添加API文档路由
    .merge(docs::swagger_routes());

    // 添加中间件
    let app = app
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]),
        )
        .layer(TraceLayer::new_for_http());

    // 获取服务器地址
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Logos 博客系统已启动，监听地址: {}", addr);
    info!("API文档地址: http://{}:{}/swagger-ui/", "localhost", port);

    // 启动服务器
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
