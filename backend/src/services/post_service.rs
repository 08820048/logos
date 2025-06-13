use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait, Set, TransactionTrait,
};
use std::{collections::HashSet, sync::Arc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use crate::{
    entities::{
        post::{self, Entity as Post},
        post_tag::{self, Entity as PostTag},
        tag::{self, Entity as Tag},
    },
    utils::{
        error::{AppError, AppResult},
        markdown::extract_summary,
        slug::slugify,
    },
};

pub struct PostService {
    db: Arc<DatabaseConnection>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PostWithTags {
    /// 文章信息
    pub post: post::Model,
    /// 文章标签列表
    pub tags: Vec<tag::Model>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreatePostDto {
    /// 文章标题
    pub title: String,
    /// Markdown 格式的文章内容
    pub content_md: String,
    /// 文章标签列表
    pub tags: Vec<String>,
    /// 是否发布
    pub published: bool,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdatePostDto {
    /// 文章标题（可选）
    pub title: Option<String>,
    /// Markdown 格式的文章内容（可选）
    pub content_md: Option<String>,
    /// 文章标签列表（可选）
    pub tags: Option<Vec<String>>,
    /// 是否发布（可选）
    pub published: Option<bool>,
}

impl PostService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// 创建新文章
    pub async fn create_post(&self, dto: CreatePostDto) -> AppResult<PostWithTags> {
        let txn = self.db.begin().await?;

        // 生成 slug
        let slug = self.generate_unique_slug(&dto.title).await?;
        
        // 提取摘要
        let summary = extract_summary(&dto.content_md, 200);

        // 创建文章
        let post_model = post::ActiveModel {
            title: Set(dto.title),
            slug: Set(slug),
            content_md: Set(dto.content_md),
            summary: Set(summary),
            published: Set(dto.published),
            ..Default::default()
        };

        let post = post_model.insert(&txn).await?;

        // 处理标签
        let tags = self.handle_post_tags(&txn, post.id, &dto.tags).await?;

        txn.commit().await?;

        Ok(PostWithTags { post, tags })
    }

    /// 更新文章
    pub async fn update_post(&self, id: i32, dto: UpdatePostDto) -> AppResult<PostWithTags> {
        let txn = self.db.begin().await?;

        // 查找文章
        let post = Post::find_by_id(id)
            .one(&txn)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("文章 ID {} 不存在", id)))?;

        // 准备更新模型
        let mut post_model: post::ActiveModel = post.clone().into();

        // 如果标题更新了，重新生成 slug
        if let Some(title) = &dto.title {
            if title != &post.title {
                let slug = self.generate_unique_slug(title).await?;
                post_model.slug = Set(slug);
            }
            post_model.title = Set(title.clone());
        }

        // 如果内容更新了，重新生成摘要
        if let Some(content_md) = &dto.content_md {
            let summary = extract_summary(content_md, 200);
            
            post_model.content_md = Set(content_md.clone());
            post_model.summary = Set(summary);
        }

        // 更新发布状态
        if let Some(published) = dto.published {
            post_model.published = Set(published);
        }

        // 更新文章
        let updated_post = post_model.update(&txn).await?;

        // 如果提供了标签，更新标签
        let tags = if let Some(tags) = &dto.tags {
            self.handle_post_tags(&txn, updated_post.id, tags).await?
        } else {
            // 获取现有标签
            Tag::find()
                .join(sea_orm::JoinType::InnerJoin, tag::Relation::PostTag.def())
                .filter(post_tag::Column::PostId.eq(updated_post.id))
                .all(&txn)
                .await?
        };

        txn.commit().await?;

        Ok(PostWithTags {
            post: updated_post,
            tags,
        })
    }

    /// 删除文章
    pub async fn delete_post(&self, id: i32) -> AppResult<()> {
        // 查找文章
        let post = Post::find_by_id(id)
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("文章 ID {} 不存在", id)))?;

        // 删除文章（关联的标签和评论会通过外键级联删除）
        let post_model: post::ActiveModel = post.into();
        post_model.delete(&*self.db).await?;

        Ok(())
    }

    /// 获取文章详情
    pub async fn get_post(&self, id: i32) -> AppResult<PostWithTags> {
        // 查找文章
        let post = Post::find_by_id(id)
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("文章 ID {} 不存在", id)))?;

        // 获取文章的标签
        let tags = Tag::find()
            .join(sea_orm::JoinType::InnerJoin, tag::Relation::PostTag.def())
            .filter(post_tag::Column::PostId.eq(post.id))
            .all(&*self.db)
            .await?;

        Ok(PostWithTags { post, tags })
    }

    /// 根据 slug 获取文章
    pub async fn get_post_by_slug(&self, slug: &str) -> AppResult<PostWithTags> {
        // 查找文章
        let post = Post::find()
            .filter(post::Column::Slug.eq(slug))
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("文章 slug {} 不存在", slug)))?;

        // 获取文章的标签
        let tags = Tag::find()
            .join(sea_orm::JoinType::InnerJoin, tag::Relation::PostTag.def())
            .filter(post_tag::Column::PostId.eq(post.id))
            .all(&*self.db)
            .await?;

        Ok(PostWithTags { post, tags })
    }

    /// 分页获取文章列表
    pub async fn list_posts(
        &self,
        page: u64,
        limit: u64,
        published_only: bool,
    ) -> AppResult<(Vec<PostWithTags>, u64)> {
        // 构建查询
        let mut query = Post::find().order_by_desc(post::Column::CreatedAt);

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

    /// 处理文章标签关联
    async fn handle_post_tags<C>(
        &self,
        db: &C,
        post_id: i32,
        tag_names: &[String],
    ) -> AppResult<Vec<tag::Model>> 
    where
        C: sea_orm::ConnectionTrait,
    {
        // 删除现有的文章-标签关联
        PostTag::delete_many()
            .filter(post_tag::Column::PostId.eq(post_id))
            .exec(db)
            .await?;

        let mut tags = Vec::new();

        // 处理每个标签
        for tag_name in tag_names {
            if tag_name.trim().is_empty() {
                continue;
            }

            // 查找或创建标签
            let tag = self.find_or_create_tag(db, tag_name).await?;
            tags.push(tag.clone());

            // 创建文章-标签关联
            let post_tag = post_tag::ActiveModel {
                post_id: Set(post_id),
                tag_id: Set(tag.id),
            };
            post_tag.insert(db).await?;
        }

        Ok(tags)
    }

    /// 查找或创建标签
    async fn find_or_create_tag<C>(&self, db: &C, name: &str) -> AppResult<tag::Model> 
    where
        C: sea_orm::ConnectionTrait,
    {
        // 查找标签
        let tag = Tag::find()
            .filter(tag::Column::Name.eq(name))
            .one(db)
            .await?;

        // 如果标签存在，直接返回
        if let Some(tag) = tag {
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

        let tag = tag_model.insert(db).await?;
        Ok(tag)
    }

    /// 生成唯一的 slug
    async fn generate_unique_slug(&self, title: &str) -> AppResult<String> {
        let base_slug = slugify(title);
        
        // 查询所有以该 slug 开头的 slug
        let existing_slugs = Post::find()
            .filter(post::Column::Slug.like(&format!("{}%", base_slug)))
            .all(&*self.db)
            .await?
            .into_iter()
            .map(|p| p.slug)
            .collect::<HashSet<_>>();
        
        // 如果 slug 不存在，直接返回
        if !existing_slugs.contains(&base_slug) {
            return Ok(base_slug);
        }
        
        // 否则添加数字后缀
        let mut counter = 1;
        loop {
            let new_slug = format!("{}-{}", base_slug, counter);
            if !existing_slugs.contains(&new_slug) {
                return Ok(new_slug);
            }
            counter += 1;
        }
    }
}
