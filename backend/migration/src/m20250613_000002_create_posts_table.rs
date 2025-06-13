use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Posts::Title).string().not_null())
                    .col(ColumnDef::new(Posts::Slug).string().not_null().unique_key())
                    .col(ColumnDef::new(Posts::ContentMd).text().not_null())
                    .col(ColumnDef::new(Posts::Summary).string().not_null())
                    .col(ColumnDef::new(Posts::Published).boolean().not_null().default(false))
                    .col(
                        ColumnDef::new(Posts::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Posts::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .extra("ON UPDATE CURRENT_TIMESTAMP".to_string()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Posts {
    Table,
    Id,
    Title,
    Slug,
    ContentMd,
    Summary,
    Published,
    CreatedAt,
    UpdatedAt,
}
