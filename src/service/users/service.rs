use super::models::{Allusers, RegisterUser, UpdateUser};
use crate::AppState;
use actix_web::*;
use bcrypt::{hash, verify, DEFAULT_COST};

#[get("/users")]
async fn get_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query!("SELECT * FROM users")
        .fetch_all(&app_state.postgres_client)
        .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(
            users
                .iter()
                .map(|user| Allusers {
                    id: user.id,
                    name: user.name.clone(),
                    email: user.email.clone(),
                    password: user.password.clone(),
                })
                .collect::<Vec<Allusers>>(),
        ),
        Err(_) => {
            HttpResponse::InternalServerError().body("Error Trying to get all users from database.")
        }
    }
}

#[post("/users")]
async fn create_user(
    app_state: web::Data<AppState>,
    user: web::Json<RegisterUser>,
) -> impl Responder {
    let hashed = hash(&user.password, DEFAULT_COST).expect("Erro hashed");

    let missing_fields = [
        ("name", &user.name),
        ("email", &user.email),
        ("password", &user.password),
    ]
    .iter()
    .filter(|(_, value)| value.is_empty())
    .map(|(field, _)| *field)
    .collect::<Vec<&str>>();

    if !missing_fields.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!(
            {
                "message":"Missing required fields",
                "Fields":missing_fields
            }
        ));
    }

    let result = sqlx::query!(
        "INSERT INTO users (name, email, password ) VALUES ($1,$2,$3) RETURNING * ",
        user.name,
        user.email,
        hashed
    )
    .fetch_one(&app_state.postgres_client)
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(RegisterUser {
            name: user.name,
            email: user.email,
            password: user.password,
        }),
        Err(_) => HttpResponse::InternalServerError().body("Error Trying to create user."),
    }
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users).service(create_user);
}
