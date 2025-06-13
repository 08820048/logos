use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{
    entities::{
        comment::{self, Entity as Comment},
        post::{self, Entity as Post},
    },
    utils::error::{AppError, AppResult},
};

pub struct CommentService {
    db: Arc<DatabaseConnection>,
}

#[derive(Debug, Clone, ToSchema)]
pub struct CreateCommentDto {
    pub post_id: i32,
    pub nickname: String,
    pub email: String,
    pub content: String,
}

#[derive(Debug, Clone, ToSchema)]
pub struct UpdateCommentStatusDto {
    pub status: String,
}

impl CommentService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// 创建新评论
    pub async fn create_comment(&self, dto: CreateCommentDto) -> AppResult<comment::Model> {
        // 检查文章是否存在
        let post = Post::find_by_id(dto.post_id)
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("文章 ID {} 不存在", dto.post_id)))?;

        // 检查文章是否已发布
        if !post.published {
            return Err(AppError::BadRequest("无法评论未发布的文章".to_string()));
        }

        // 创建评论
        let comment_model = comment::ActiveModel {
            post_id: Set(dto.post_id),
            nickname: Set(dto.nickname),
            email: Set(dto.email),
            content: Set(dto.content),
            status: Set("pending".to_string()), // 默认为待审核状态
            ..Default::default()
        };

        let comment = comment_model.insert(&*self.db).await?;
        Ok(comment)
    }

    /// 获取文章的评论列表
    pub async fn list_comments(
        &self,
        post_id: i32,
        page: u64,
        limit: u64,
        include_pending: bool,
    ) -> AppResult<(Vec<comment::Model>, u64)> {
        // 检查文章是否存在
        let _post = Post::find_by_id(post_id)
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("文章 ID {} 不存在", post_id)))?;

        // 构建查询
        let mut query = Comment::find()
            .filter(comment::Column::PostId.eq(post_id))
            .order_by_desc(comment::Column::CreatedAt);

        // 如果不包括待审核的评论
        if !include_pending {
            query = query.filter(comment::Column::Status.eq("approved"));
        }

        // 获取总数
        let total = query.clone().count(&*self.db).await?;

        // 分页查询
        let comments = query
            .paginate(&*self.db, limit)
            .fetch_page(page - 1)
            .await?;

        Ok((comments, total))
    }

    /// 获取所有评论（用于管理后台）
    pub async fn list_all_comments(
        &self,
        page: u64,
        limit: u64,
        status: Option<&str>,
    ) -> AppResult<(Vec<comment::Model>, u64)> {
        // 构建查询
        let mut query = Comment::find().order_by_desc(comment::Column::CreatedAt);

        // 如果指定了状态
        if let Some(status) = status {
            query = query.filter(comment::Column::Status.eq(status));
        }

        // 获取总数
        let total = query.clone().count(&*self.db).await?;

        // 分页查询
        let comments = query
            .paginate(&*self.db, limit)
            .fetch_page(page - 1)
            .await?;

        Ok((comments, total))
    }

    /// 更新评论状态
    pub async fn update_comment_status(&self, id: i32, status: &str) -> AppResult<comment::Model> {
        // 检查状态是否有效
        if !["pending", "approved", "rejected"].contains(&status) {
            return Err(AppError::BadRequest(format!("无效的评论状态: {}", status)));
        }

        // 查找评论
        let comment = Comment::find_by_id(id)
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("评论 ID {} 不存在", id)))?;

        // 更新状态
        let mut comment_model: comment::ActiveModel = comment.into();
        comment_model.status = Set(status.to_string());
        let updated_comment = comment_model.update(&*self.db).await?;

        Ok(updated_comment)
    }

    /// 删除评论
    pub async fn delete_comment(&self, id: i32) -> AppResult<()> {
        // 查找评论
        let comment = Comment::find_by_id(id)
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("评论 ID {} 不存在", id)))?;

        // 删除评论
        let comment_model: comment::ActiveModel = comment.into();
        comment_model.delete(&*self.db).await?;

        Ok(())
    }
}
