use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct School {
    pub id: i32,
    pub name: String,
    pub domain: String,
}
