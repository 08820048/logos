use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, Set,
};
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{
    entities::link::{self, Entity as Link},
    utils::error::{AppError, AppResult},
};

pub struct LinkService {
    db: Arc<DatabaseConnection>,
}

#[derive(Debug, Clone, ToSchema)]
pub struct CreateLinkDto {
    pub name: String,
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, ToSchema)]
pub struct UpdateLinkDto {
    pub name: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
}

impl LinkService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// 创建新友情链接
    pub async fn create_link(&self, dto: CreateLinkDto) -> AppResult<link::Model> {
        // 创建友情链接
        let link_model = link::ActiveModel {
            name: Set(dto.name),
            url: Set(dto.url),
            description: Set(dto.description),
            ..Default::default()
        };

        let link = link_model.insert(&*self.db).await?;
        Ok(link)
    }

    /// 获取所有友情链接
    pub async fn list_links(&self, page: u64, limit: u64) -> AppResult<(Vec<link::Model>, u64)> {
        // 构建查询
        let query = Link::find().order_by_asc(link::Column::Name);

        // 获取总数
        let total = query.clone().count(&*self.db).await?;

        // 分页查询
        let links = query
            .paginate(&*self.db, limit)
            .fetch_page(page - 1)
            .await?;

        Ok((links, total))
    }

    /// 获取友情链接详情
    pub async fn get_link(&self, id: i32) -> AppResult<link::Model> {
        let link = Link::find_by_id(id)
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("友情链接 ID {} 不存在", id)))?;

        Ok(link)
    }

    /// 更新友情链接
    pub async fn update_link(&self, id: i32, dto: UpdateLinkDto) -> AppResult<link::Model> {
        // 查找友情链接
        let link = Link::find_by_id(id)
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("友情链接 ID {} 不存在", id)))?;

        // 准备更新模型
        let mut link_model: link::ActiveModel = link.into();

        // 更新字段
        if let Some(name) = dto.name {
            link_model.name = Set(name);
        }

        if let Some(url) = dto.url {
            link_model.url = Set(url);
        }

        link_model.description = Set(dto.description);

        // 更新友情链接
        let updated_link = link_model.update(&*self.db).await?;

        Ok(updated_link)
    }

    /// 删除友情链接
    pub async fn delete_link(&self, id: i32) -> AppResult<()> {
        // 查找友情链接
        let link = Link::find_by_id(id)
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("友情链接 ID {} 不存在", id)))?;

        // 删除友情链接
        let link_model: link::ActiveModel = link.into();
        link_model.delete(&*self.db).await?;

        Ok(())
    }
}
