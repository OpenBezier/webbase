use sea_orm::DatabaseConnection;
use sea_orm::{ConnectOptions, Database};
use std::sync::OnceLock;
use tracing::info;

static MYSQL: OnceLock<DatabaseConnection> = OnceLock::<DatabaseConnection>::new();

pub fn init_mysql(dburl: String) -> &'static DatabaseConnection {
    MYSQL.get_or_init(|| {
        let mut opt = ConnectOptions::new(dburl.to_owned());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(std::time::Duration::from_secs(8))
            .idle_timeout(std::time::Duration::from_secs(8))
            .max_lifetime(std::time::Duration::from_secs(8))
            .sqlx_logging(false);
        let conn = futures::executor::block_on(async { Database::connect(opt).await.unwrap() });
        info!("connect to mysql successfully");
        conn
    })
}

pub fn get_mysql() -> &'static DatabaseConnection {
    MYSQL.get().unwrap()
}
