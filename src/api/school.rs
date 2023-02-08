use crate::{model::school::school, appstate};
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    post,
    web::{Data, Json},
    HttpResponse, Responder, ResponseError,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgDatabaseError, query_as};
use std::ops::Deref;
use strum::Display;

#[derive(Debug, Display)]
pub enum SchoolError {
    BadAuthRequest(String),
    AuthConflict(String),
    DatabaseError,
}

impl ResponseError for SchoolError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            SchoolError::BadAuthRequest(_) => StatusCode::BAD_REQUEST,
            SchoolError::AuthConflict(_) => StatusCode::CONFLICT,
            SchoolError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let message = match self {
            SchoolError::BadAuthRequest(message) | SchoolError::AuthConflict(message) => {
                message.to_string()
            }

            _ => self.to_string(),
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(message)
    }
}

#[get("")]
async fn fetch_all_schools(data: Data<AppState>) -> Result<Json<Vec<School>>, SchoolError> {
    let schools = query_as!(School, "SELECT * FROM schools")
        .fetch_all(&data.db)
        .await;

    if let Err(_) = schools {
        return Err(SchoolError::DatabaseError);
    }

    let schools = schools.unwrap();

    Ok(Json(schools))
}

#[derive(Serialize, Deserialize)]
struct CreateSchool {
    name: String,
    domain: String,
}

#[post("")]
async fn create_school(
    body: Json<CreateSchool>,
    data: Data<AppState>,
    ) -> Result<impl Responder, SchoolError> {
    let query = query_as!(
        School,
        "INSERT INTO schools (name, domain) VALUES ($1, $2)",
        body.name,
        body.domain
        )
        .execute(&data.db)
        .await;

    if let Err(query) = query {
        if let Some(query) = query.as_database_error() {
            let code = query.code().unwrap();

            return Err(match code.to_string().deref() {
                "23505" => SchoolError::AuthConflict(
                    query
                    .downcast_ref::<PgDatabaseError>()
                    .detail()
                    .unwrap()
                    .to_string(),
                    ),
                _ => SchoolError::BadAuthRequest(query.message().to_string()),
            });
        }

        return Err(SchoolError::DatabaseError);
    }

    Ok(HttpResponse::Created())
        >>>>>>> c60b83bb7d21f4c19e89f90cc74db97570211b14
}
