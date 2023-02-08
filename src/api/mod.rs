mod auth;
mod school;

use actix_web::{web, Scope};

use self::{
    auth::{login, sign_in},
    school::{create_school, fetch_all_schools},
};

pub fn auth() -> Scope {
    web::scope("/auth").service(login).service(sign_in)
}

pub fn school() -> Scope {
    web::scope("/school")
        .service(fetch_all_schools)
        .service(create_school)
}
