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
    pub id: String,
    pub name: String,
    pub cpf: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    is_open: bool,
}

impl User {
    pub fn new(name: String, cpf: String, email: String, password: String, role: UserRole) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            cpf,
            email,
            password,
            role,
            is_open: false,
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn open_account(&mut self) {
        self.is_open = true;
    }
}
