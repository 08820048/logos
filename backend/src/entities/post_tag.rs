use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::{post, tag};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "post_tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub post_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "post::Entity",
        from = "Column::PostId",
        to = "post::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Post,
    #[sea_orm(
        belongs_to = "tag::Entity",
        from = "Column::TagId",
        to = "tag::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Tag,
}

impl Related<post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
