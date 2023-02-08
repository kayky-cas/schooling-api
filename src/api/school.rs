use actix_web::{
    get,
    web::{Data, Json},
};
use sqlx::query_as;

use crate::AppState;

#[get("")]
async fn fetch_schools(data: Data<AppState>) -> Json<String> {
    let schools = query_as!(School, "SELECT * FROM school")
        .fetch_all(&data.db)
        .await
        .unwrap();
    Json(serde_json::to_string(&schools).unwrap())
}

#[derive(serde::Serialize)]
pub struct School {
    pub id: i32,
    pub name: String,
    pub address: String,
}
