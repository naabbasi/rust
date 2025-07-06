pub mod sea_orm_example {
    use std::time::Duration;
    use sea_orm::{ConnectOptions, Database};

    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }

    #[tokio::main]
    pub async fn hellworld() {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sea_orm_example::add(2, 2);
        assert_eq!(result, 4);
    }
}
