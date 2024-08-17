use std::env::var;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn start_connection() -> Pool<Postgres> {
    let postgres_environment = var("DATABASE_URL").expect("DATABASE must be set ");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_environment)
        .await
        .expect("FAILED TO CONNECT POSTGRES");

    let check_migration = sqlx::migrate!("./src/database/postgres_connection/migration")
        .run(&pool)
        .await;

    match check_migration {
        Ok(_) => println!("Migration succefully"),
        Err(e) => println!("Error runnig migration: {:?}", e),
    }
    pool
}
