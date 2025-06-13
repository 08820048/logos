use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
};
use std::sync::Arc;

use crate::{
    entities::{
        post::{self, Entity as Post},
        post_tag::{self, Entity as PostTag},
        tag::{self, Entity as Tag},
    },
    services::post_service::PostWithTags,
    utils::{
        error::{AppError, AppResult},
        slug::slugify,
    },
};

pub struct TagService {
    db: Arc<DatabaseConnection>,
}

impl TagService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// 创建新标签
    pub async fn create_tag(&self, name: &str) -> AppResult<tag::Model> {
        // 检查标签是否已存在
        let existing_tag = Tag::find()
            .filter(tag::Column::Name.eq(name))
            .one(&*self.db)
            .await?;

        if let Some(tag) = existing_tag {
            return Ok(tag);
        }

        // 生成 slug
        let slug = slugify(name);

        // 创建新标签
        let tag_model = tag::ActiveModel {
            name: Set(name.to_string()),
            slug: Set(slug),
            ..Default::default()
        };

        let tag = tag_model.insert(&*self.db).await?;
        Ok(tag)
    }

    /// 获取所有标签
    pub async fn list_tags(&self) -> AppResult<Vec<tag::Model>> {
        let tags = Tag::find()
            .order_by_asc(tag::Column::Name)
            .all(&*self.db)
            .await?;

        Ok(tags)
    }

    /// 获取标签详情
    pub async fn get_tag(&self, id: i32) -> AppResult<tag::Model> {
        let tag = Tag::find_by_id(id)
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("标签 ID {} 不存在", id)))?;

        Ok(tag)
    }

    /// 根据 slug 获取标签
    pub async fn get_tag_by_slug(&self, slug: &str) -> AppResult<tag::Model> {
        let tag = Tag::find()
            .filter(tag::Column::Slug.eq(slug))
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("标签 slug {} 不存在", slug)))?;

        Ok(tag)
    }

    /// 删除标签
    pub async fn delete_tag(&self, id: i32) -> AppResult<()> {
        // 查找标签
        let tag = Tag::find_by_id(id)
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("标签 ID {} 不存在", id)))?;

        // 删除标签（关联的文章-标签关系会通过外键级联删除）
        let tag_model: tag::ActiveModel = tag.into();
        tag_model.delete(&*self.db).await?;

        Ok(())
    }

    /// 获取标签下的文章
    pub async fn get_tag_posts(
        &self,
        tag_id: i32,
        page: u64,
        limit: u64,
        published_only: bool,
    ) -> AppResult<(Vec<PostWithTags>, u64)> {
        // 查找标签
        let _tag = Tag::find_by_id(tag_id)
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("标签 ID {} 不存在", tag_id)))?;

        // 查找标签下的文章 ID
        let post_ids = PostTag::find()
            .filter(post_tag::Column::TagId.eq(tag_id))
            .all(&*self.db)
            .await?
            .into_iter()
            .map(|pt| pt.post_id)
            .collect::<Vec<_>>();

        if post_ids.is_empty() {
            return Ok((Vec::new(), 0));
        }

        // 构建查询
        let mut query = Post::find()
            .filter(post::Column::Id.is_in(post_ids.clone()))
            .order_by_desc(post::Column::CreatedAt);

        // 如果只查询已发布的文章
        if published_only {
            query = query.filter(post::Column::Published.eq(true));
        }

        // 获取总数
        let total = query.clone().count(&*self.db).await?;

        // 分页查询
        let posts = query
            .paginate(&*self.db, limit)
            .fetch_page(page - 1)
            .await?;

        // 获取每篇文章的标签
        let mut posts_with_tags = Vec::new();
        for post in posts {
            let tags = Tag::find()
                .join(sea_orm::JoinType::InnerJoin, tag::Relation::PostTag.def())
                .filter(post_tag::Column::PostId.eq(post.id))
                .all(&*self.db)
                .await?;

            posts_with_tags.push(PostWithTags { post, tags });
        }

        Ok((posts_with_tags, total))
    }
}
