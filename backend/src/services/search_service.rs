use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, QueryOrder,
};
use std::sync::Arc;

use crate::{
    entities::post::{self, Entity as Post},
    services::post_service::PostWithTags,
    utils::error::AppResult,
};

pub struct SearchService {
    db: Arc<DatabaseConnection>,
}

impl SearchService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// 搜索文章
    pub async fn search_posts(&self, query: &str, published_only: bool) -> AppResult<Vec<PostWithTags>> {
        // 如果查询为空，返回空结果
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        // 构建查询条件
        let mut condition = Condition::any()
            .add(post::Column::Title.contains(query))
            .add(post::Column::ContentMd.contains(query));

        // 如果只查询已发布的文章
        if published_only {
            condition = condition.add(post::Column::Published.eq(true));
        }

        // 执行查询
        let posts = Post::find()
            .filter(condition)
            .order_by_desc(post::Column::CreatedAt)
            .all(&*self.db)
            .await?;

        // 获取每篇文章的标签
        let mut posts_with_tags = Vec::new();
        for post in posts {
            let tags = post
                .find_related(crate::entities::tag::Entity)
                .all(&*self.db)
                .await?;

            posts_with_tags.push(PostWithTags { post, tags });
        }

        Ok(posts_with_tags)
    }
}
