use actix_web::{post, delete, HttpResponse, Responder, web, error::ResponseError, http::StatusCode};
use crate::db::user::User;
use crate::db::model::{create_user};
use mongodb::error::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct createReq{
    email: String,
    password: String,
}

#[post("/users/create")]
pub async fn create_user_api(data: web::Json<createReq>) -> impl Responder{
    let user = User::new(&data.email, &data.password);
    match create_user(&user).await {
        Ok(()) => HttpResponse::Ok().body("User create success!"),
        Err(v) => v.error_response(),
    }
}