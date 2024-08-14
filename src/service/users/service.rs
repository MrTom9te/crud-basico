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
        Ok(users) => {
            println!("All user is required");
            HttpResponse::Ok().json(
                users
                    .iter()
                    .map(|user| Allusers {
                        id: user.id,
                        name: user.name.clone(),
                        email: user.email.clone(),
                        password: user.password.clone(),
                    })
                    .collect::<Vec<Allusers>>(),
            )
        }
        Err(_) => {
            HttpResponse::InternalServerError().body("Error Trying to get all users from database.")
        }
    }
}

#[put("/users/{i32}")] //Atualiza um user
async fn update_users(
    app_state: web::Data<AppState>,
    user: web::Json<UpdateUser>,
    id: web::Path<i32>,
) -> impl Responder {
    let hashed = hash(&user.password, DEFAULT_COST).expect("failed to hash password");
    if !(hashed != user.password) {
        return HttpResponse::InternalServerError().body("Error Trying to hash password");
    }

    let result = sqlx::query!(
        "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4",
        user.name,
        user.email,
        hashed,
        id.into_inner()
    )
    .fetch_one(&app_state.postgres_client)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User update"),
        Err(e) => {
            println!("Error {}", e);
            HttpResponse::InternalServerError().body("Error")
        }
    }
}

#[post("/users")] // Criação de usuario no Banco de Dados
async fn create_user(
    app_state: web::Data<AppState>,
    user: web::Json<RegisterUser>,
) -> impl Responder {
    let hashed = hash(&user.password, DEFAULT_COST).expect("Erro hashed");
    if !(hashed != user.password) {
        return HttpResponse::InternalServerError().body("Error Trying to hash password");
    }

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
        Ok(result_query) => HttpResponse::Ok().json(Allusers {
            name: result_query.name,
            email: result_query.email,
            password: result_query.password,
            id: result_query.id,
        }),
        Err(_) => HttpResponse::InternalServerError().body("Error Trying to create user."),
    }
}
#[delete("/users/{id}")]
async fn delete_user(app_state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1 RETURNING id, name , email, password",
        id.into_inner()
    )
    .fetch_one(&app_state.postgres_client)
    .await;

    match result {
        Ok(result_query) => HttpResponse::Ok().json(Allusers {
            id: result_query.id,
            name: result_query.name,
            email: result_query.email,
            password: result_query.password,
        }),
        Err(e) => {
            println!("User not found : {} ", e);
            HttpResponse::InternalServerError().body("Erro trying to delete user")
        }
    }
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users)
        .service(create_user)
        .service(update_users);
}
