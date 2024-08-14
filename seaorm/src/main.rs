use sea_orm::{ConnectOptions, Database};
use std::time::Duration;
use seaorm::add;

#[tokio::main]
async fn main() {
    //let db: DatabaseConnection = Database::connect("postgres://username:password@host/database?currentSchema=my_schema").await?;

    println!("{}", add(1, 2));

    let mut opt = ConnectOptions::new("postgres://username:password@host/database?currentSchema=my_schema");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await;
    if db.is_ok() {
        let database_connection = db.unwrap();
        database_connection.close().await.ok();
    }
}