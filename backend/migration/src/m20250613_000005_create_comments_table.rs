use sea_orm_migration::prelude::*;

use super::m20250613_000002_create_posts_table::Posts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comments::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Comments::PostId).integer().not_null())
                    .col(ColumnDef::new(Comments::Nickname).string().not_null())
                    .col(ColumnDef::new(Comments::Email).string().not_null())
                    .col(ColumnDef::new(Comments::Content).text().not_null())
                    .col(ColumnDef::new(Comments::Status).string().not_null().default("pending"))
                    .col(
                        ColumnDef::new(Comments::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comments_post_id")
                            .from(Comments::Table, Comments::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Comments {
    Table,
    Id,
    PostId,
    Nickname,
    Email,
    Content,
    Status,
    CreatedAt,
}
