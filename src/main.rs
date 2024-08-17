use actix_files::Files;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};

mod service {
    pub mod users;
}
mod database {
    pub mod postgres_connection;
}
#[derive(Clone)]
pub struct AppState {
    postgres_client: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let _pool = database::postgres_connection::start_connection().await;
    HttpServer::new(move || {
        App::new()
            .service(
                Files::new("/", "./static")
                    .index_file("index.html")
                    .show_files_listing(),
            )
            .app_data(actix_web::web::Data::new(AppState {
                postgres_client: _pool.clone(),
            }))
            .configure(service::users::service::user_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
