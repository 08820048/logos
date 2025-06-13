use sea_orm::entity::prelude::*;
use sea_orm::RelationTrait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{comment, post_tag, tag};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub slug: String,
    #[sea_orm(column_type = "Text")]
    pub content_md: String,
    pub summary: String,
    pub published: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Comment,
    PostTag,
    Tag,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Comment => Entity::has_many(comment::Entity).into(),
            Self::PostTag => Entity::has_many(post_tag::Entity).into(),
            Self::Tag => Entity::has_many(tag::Entity).into(),
        }
    }
}

impl Related<comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}

impl Related<tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}

impl Related<post_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostTag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
