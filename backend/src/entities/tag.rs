use sea_orm::entity::prelude::*;
use sea_orm::RelationTrait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{post, post_tag};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(unique)]
    pub slug: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    PostTag,
    Post,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::PostTag => Entity::has_many(post_tag::Entity).into(),
            Self::Post => Entity::has_many(post::Entity).into(),
        }
    }
}

impl Related<post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<post_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostTag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
