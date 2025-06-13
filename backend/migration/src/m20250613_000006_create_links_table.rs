use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Links::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Links::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Links::Name).string().not_null())
                    .col(ColumnDef::new(Links::Url).string().not_null())
                    .col(ColumnDef::new(Links::Description).string())
                    .col(
                        ColumnDef::new(Links::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Links::UpdatedAt)
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
            .drop_table(Table::drop().table(Links::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Links {
    Table,
    Id,
    Name,
    Url,
    Description,
    CreatedAt,
    UpdatedAt,
}
