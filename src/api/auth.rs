use actix_web::{get, post, web::Json, ResponseError, HttpResponse, http::{header::{ContentType, ContentEncoding}, StatusCode}, Result, body::MessageBody};
use serde::{Serialize, Deserialize};
use serde_json::json;
use derive_more::{Display};

use crate::model::user::UserRole;

#[derive(Debug, Display)]
pub enum AuthError {
    BadAuthRequest(&'static str),
    AuthConflict(&'static str)
}

impl ResponseError for AuthError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
           AuthError::BadAuthRequest(_) => StatusCode::BAD_REQUEST,
           AuthError::AuthConflict(_) => StatusCode::CONFLICT
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let message = match self {
            AuthError::BadAuthRequest(message) |
            AuthError::AuthConflict(message) => message.to_string(),

            _ => self.to_string()
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(message)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SecurityUser {
    name: String,
    cpf: String,
    email: String,
    role: UserRole
}

#[derive(Serialize, Deserialize)]
pub struct LoginInput {
    email: String,
    password: String
}

#[post("")]
pub async fn login(body: Json<LoginInput>) -> Result<Json<SecurityUser>, AuthError> {
    Err(AuthError::BadAuthRequest("Incorrect email or password!"))
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUser {
    name: String,
    cpf: String,
    email: String,
    password: String,
    role: UserRole
}

#[post("/register")]
pub async fn sign_in(body: Json<RegisterUser>) -> Result<Json<SecurityUser>, AuthError> {
    Err(AuthError::AuthConflict("Email already registred!"))
}
