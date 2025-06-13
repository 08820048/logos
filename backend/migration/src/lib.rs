pub use sea_orm_migration::prelude::*;

mod m20250613_000001_create_users_table;
mod m20250613_000002_create_posts_table;
mod m20250613_000003_create_tags_table;
mod m20250613_000004_create_post_tags_table;
mod m20250613_000005_create_comments_table;
mod m20250613_000006_create_links_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250613_000001_create_users_table::Migration),
            Box::new(m20250613_000002_create_posts_table::Migration),
            Box::new(m20250613_000003_create_tags_table::Migration),
            Box::new(m20250613_000004_create_post_tags_table::Migration),
            Box::new(m20250613_000005_create_comments_table::Migration),
            Box::new(m20250613_000006_create_links_table::Migration),
        ]
    }
}
