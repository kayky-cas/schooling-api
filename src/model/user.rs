use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Teacher,
    Student
}

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub cpf: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    is_open: bool
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
            is_open: false
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn open_account(&mut self) {
        self.is_open = true;
    }
}
