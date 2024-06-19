use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use once_cell::sync::Lazy;
use std::env;
use dotenvy::dotenv;

pub static POOL: Lazy<PgPool> = Lazy::new(|| {
    dotenv().ok();
    let database_url = env::var("DB_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(10)
        .connect_lazy(&database_url)
        .expect("Failed to create pool")
});

pub trait   Data_process {
    fn read(&self, conn: &sqlx::PgPool, q:&str)  -> Result<Vec<&self>, Box<dyn Error>>;
}