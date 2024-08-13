use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
pub async fn start_connection() -> Pool<Postgres> {
    let postgres_environment = std::env::var("DATABASE_URL").expect("DATABASE_URL  must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_environment)
        .await
        .expect("");

    let check_migrate = sqlx::migrate!("./")
        .run(&pool)
        .await
        .expect(" failed run migration");
    match check_migrate {
        Ok(_) => println!("MIgrations sucessfully"),
        Err(e) => println!("Error running migration"),
    }
    pool
}
