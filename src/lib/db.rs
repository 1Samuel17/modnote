// database operations
use sea_orm::{ConnectOptions, DatabaseConnection, DbErr};
use std::time::Duration;

pub fn set_db_options() -> ConnectOptions {
    let mut opt = ConnectOptions::new("sqlite://db.sqlite?mode=rwc");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false) 
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("default_schema");
    opt
}

pub async fn check(db: DatabaseConnection) {
    assert!(db.ping().await.is_ok());
    let _ = db.clone().close().await;
    assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire(_))));
}