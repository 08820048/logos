use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::sync::Arc;

use crate::{
    entities::user::{self, ActiveModel, Entity as User},
    utils::{
        auth::create_token,
        error::{AppError, AppResult},
        password::{hash_password, verify_password},
    },
};

pub struct UserService {
    db: Arc<DatabaseConnection>,
}

impl UserService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// 创建新用户
    pub async fn create_user(&self, username: &str, password: &str) -> AppResult<user::Model> {
        // 检查用户名是否已存在
        let existing_user = User::find()
            .filter(user::Column::Username.eq(username))
            .one(&*self.db)
            .await?;

        if existing_user.is_some() {
            return Err(AppError::ValidationError("用户名已存在".to_string()));
        }

        // 对密码进行哈希处理
        let password_hash = hash_password(password)?;

        // 创建新用户
        let user = ActiveModel {
            username: Set(username.to_string()),
            password_hash: Set(password_hash),
            ..Default::default()
        };

        let user = user.insert(&*self.db).await?;
        Ok(user)
    }

    /// 用户登录
    pub async fn login(&self, username: &str, password: &str) -> AppResult<String> {
        // 查找用户
        let user = User::find()
            .filter(user::Column::Username.eq(username))
            .one(&*self.db)
            .await?
            .ok_or_else(|| AppError::AuthError("用户名或密码不正确".to_string()))?;

        // 验证密码
        let is_valid = verify_password(password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::AuthError("用户名或密码不正确".to_string()));
        }

        // 生成 JWT 令牌
        let token = create_token(&user.username)?;
        Ok(token)
    }

    /// 根据用户名查找用户
    pub async fn find_by_username(&self, username: &str) -> AppResult<Option<user::Model>> {
        let user = User::find()
            .filter(user::Column::Username.eq(username))
            .one(&*self.db)
            .await?;

        Ok(user)
    }

    /// 修改用户密码
    pub async fn change_password(&self, username: &str, old_password: &str, new_password: &str) -> AppResult<()> {
        // 查找用户
        let user = self.find_by_username(username).await?
            .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

        // 验证旧密码
        let is_valid = verify_password(old_password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::AuthError("旧密码不正确".to_string()));
        }

        // 对新密码进行哈希处理
        let password_hash = hash_password(new_password)?;

        // 更新密码
        let mut user_model: ActiveModel = user.into();
        user_model.password_hash = Set(password_hash);
        user_model.update(&*self.db).await?;

        Ok(())
    }
}
