use sea_orm_migration::prelude::*;

use super::m20250613_000002_create_posts_table::Posts;
use super::m20250613_000003_create_tags_table::Tags;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PostTags::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PostTags::PostId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PostTags::TagId)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(PostTags::PostId)
                            .col(PostTags::TagId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_tags_post_id")
                            .from(PostTags::Table, PostTags::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_tags_tag_id")
                            .from(PostTags::Table, PostTags::TagId)
                            .to(Tags::Table, Tags::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostTags::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum PostTags {
    Table,
    PostId,
    TagId,
}
