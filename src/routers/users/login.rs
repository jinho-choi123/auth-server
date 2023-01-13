use actix_web::{get,HttpResponse, Responder, web, error::ResponseError, http::StatusCode};
use crate::db::model::{verify_dbuser};
use serde::Deserialize;

#[derive(Deserialize)]
struct loginReq{
    email: String,
    password: String,
}

#[get("/users/login")]
pub async fn login_user_api(data: web::Json<loginReq>) -> impl Responder {
    match verify_dbuser(&data.email, &data.password).await {
        Ok(jwt_token) => HttpResponse::Ok().body(jwt_token),
        Err(v) => v.error_response(),
    }
}   