use utoipa::OpenApi;
use axum::Router;
use axum::routing::get;
use axum::response::Html;

// 定义API文档结构
#[derive(OpenApi)]
#[openapi(
    paths(
        // 认证接口
        crate::routes::auth::login_handler,
        
        // 文章接口
        crate::routes::posts::list_posts_handler,
        crate::routes::posts::get_post_handler,
        crate::routes::posts::create_post_handler,
        crate::routes::posts::update_post_handler,
        crate::routes::posts::delete_post_handler,
        
        // 标签接口
        crate::routes::tags::list_tags_handler,
        crate::routes::tags::create_tag_handler,
        crate::routes::tags::delete_tag_handler,
        crate::routes::tags::get_tag_posts_handler,
        
        // 评论接口
        crate::routes::comments::list_comments_handler,
        crate::routes::comments::create_comment_handler,
        crate::routes::comments::list_all_comments_handler,
        crate::routes::comments::update_comment_status_handler,
        crate::routes::comments::delete_comment_handler,
        
        // 友情链接接口
        crate::routes::links::list_links_handler,
        crate::routes::links::create_link_handler,
        crate::routes::links::update_link_handler,
        crate::routes::links::delete_link_handler,
        
        // 搜索接口
        crate::routes::search::search_handler,
        
        // RSS接口
        crate::routes::rss::rss_handler,
    ),
    components(
        schemas(
            // 认证相关
            crate::routes::auth::LoginRequest,
            crate::routes::auth::LoginResponse,
            
            // 文章相关
            crate::services::post_service::CreatePostDto,
            crate::services::post_service::UpdatePostDto,
            crate::services::post_service::PostWithTags,
            crate::routes::posts::PostsResponse,
            
            // 标签相关
            crate::entities::tag::Model,
            crate::routes::tags::CreateTagRequest,
            crate::routes::tags::TagPostsResponse,
            
            // 评论相关
            crate::entities::comment::Model,
            crate::routes::comments::CommentsResponse,
            crate::routes::comments::CreateCommentRequest,
            crate::routes::comments::UpdateCommentStatusRequest,
            crate::services::comment_service::CreateCommentDto,
            crate::services::comment_service::UpdateCommentStatusDto,
            
            // 友情链接相关
            crate::entities::link::Model,
            crate::services::link_service::CreateLinkDto,
            crate::services::link_service::UpdateLinkDto,
            
            // 搜索相关
            crate::routes::search::SearchQuery,
            crate::routes::search::SearchResponse,
            
            // 错误相关
            crate::utils::error::AppError,
        )
    ),
    security(
        ("bearer_auth" = [])
    ),
    tags(
        (name = "auth", description = "认证相关接口"),
        (name = "posts", description = "文章管理接口"),
        (name = "tags", description = "标签管理接口"),
        (name = "comments", description = "评论管理接口"),
        (name = "links", description = "友情链接管理接口"),
        (name = "search", description = "搜索接口"),
        (name = "rss", description = "RSS订阅接口"),
    ),
    info(
        title = "Logos 博客系统 API",
        version = "1.0.0",
        description = "Logos 博客系统后端 API 接口文档",
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
        contact(
            name = "Logos 博客系统",
            url = "https://github.com/yourusername/logos-blog",
            email = "your.email@example.com"
        )
    )
)]
pub struct ApiDoc;

/// 注册 API 文档路由
pub fn swagger_routes() -> Router {
    let api_doc = ApiDoc::openapi();
    
    Router::new()
        .route("/api-docs/openapi.json", get(|| async move { axum::Json(api_doc) }))
        .route("/swagger-ui", get(serve_swagger_ui))
        .route("/swagger-ui/", get(serve_swagger_ui))
}

/// 提供 Swagger UI 页面
async fn serve_swagger_ui() -> Html<String> {
    Html(format!(r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Logos 博客系统 API 文档</title>
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css" />
    <style>
        body {{
            margin: 0;
            padding: 0;
        }}
        .swagger-ui .topbar {{ display: none; }}
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js"></script>
    <script>
        window.onload = function() {{
            const ui = SwaggerUIBundle({{
                url: "/api-docs/openapi.json",
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIBundle.SwaggerUIStandalonePreset
                ],
                layout: "BaseLayout",
                docExpansion: "list",
                defaultModelsExpandDepth: 1,
                defaultModelExpandDepth: 1,
            }});
            window.ui = ui;
        }};
    </script>
</body>
</html>"#))
}
