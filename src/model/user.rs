use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Teacher,
    Student,
}

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub cpf: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    is_open: bool,
}
