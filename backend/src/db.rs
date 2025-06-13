use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::Arc;
use tokio::sync::OnceCell;

static DB_CONN: OnceCell<Arc<DatabaseConnection>> = OnceCell::const_new();

/// 初始化数据库连接
pub async fn init_db(database_url: &str) -> Result<(), DbErr> {
    let conn = Database::connect(database_url).await?;
    DB_CONN.get_or_init(|| async move { Arc::new(conn) }).await;
    Ok(())
}

/// 获取数据库连接
pub async fn get_conn() -> Arc<DatabaseConnection> {
    DB_CONN
        .get_or_init(|| async {
            let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let conn = Database::connect(&database_url)
                .await
                .expect("Failed to connect to database");
            Arc::new(conn)
        })
        .await
        .clone()
}
