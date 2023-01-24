use actix_web::{
    http::{header::ContentType, StatusCode},
    post,
    web::{Data, Json},
    HttpResponse, Responder, ResponseError, Result,
};
use derive_more::Display;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{
    model::user::{User, UserRole},
    AppState,
};

#[derive(Debug, Display)]
pub enum AuthError {
    BadAuthRequest(&'static str),
    AuthConflict(&'static str),
    TokenParse,
}

impl ResponseError for AuthError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AuthError::BadAuthRequest(_) => StatusCode::BAD_REQUEST,
            AuthError::AuthConflict(_) => StatusCode::CONFLICT,
            AuthError::TokenParse => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let message = match self {
            AuthError::BadAuthRequest(message) | AuthError::AuthConflict(message) => {
                message.to_string()
            }

            _ => self.to_string(),
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(message)
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct SecurityUser {
    name: String,
    cpf: String,
    email: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct PayloadUser {
    id: i32,
    name: String,
    cpf: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    user: SecurityUser,
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginInput {
    email: String,
    password: String,
}

#[post("")]
pub async fn login(
    body: Json<LoginInput>,
    data: Data<AppState>,
) -> Result<Json<LoginResponse>, AuthError> {
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set!");

    let email = body.email.clone();

    let user = sqlx::query_as!(
        PayloadUser,
        "SELECT id, name, cpf, email FROM users WHERE email = ($1)",
        email,
    )
    .fetch_one(&data.db)
    .await;

    if let Err(_) = user {
        return Err(AuthError::BadAuthRequest("Incorrect email or password!"));
    }

    let user = user.unwrap();

    let token = encode(
        &Header::default(),
        &user,
        &EncodingKey::from_secret(secret_key.as_ref()),
    );

    if let Err(_) = token {
        return Err(AuthError::TokenParse);
    }

    Ok(Json(LoginResponse {
        user: SecurityUser {
            name: user.name,
            cpf: user.cpf,
            email: user.email,
        },
        token: token.unwrap(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUser {
    name: String,
    cpf: String,
    email: String,
    password: String,
    role: UserRole,
}

#[post("/register")]
pub async fn sign_in(
    body: Json<RegisterUser>,
    data: Data<AppState>,
) -> Result<impl Responder, AuthError> {
    let query = sqlx::query!(
        "INSERT INTO users (name, cpf, email, password, role) VALUES ($1, $2, $3, $4, $5)",
        body.name,
        body.cpf,
        body.email,
        body.password,
        body.role.clone() as UserRole
    )
    .execute(&data.db)
    .await;

    if let Err(query) = query {
        println!("{:?}", query.to_string());
        return Err(AuthError::AuthConflict("Email already registred!"));
    }

    Ok(HttpResponse::Created())
}
