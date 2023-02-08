use actix_web::{post, delete, HttpResponse, Responder, web, error::ResponseError, http::StatusCode};
use crate::db::user::User;
use crate::db::model::{create_dbuser};
use crate::utils::errors::AppErr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct createReq{
    email: String,
    password: String,
}

#[derive(Serialize)]
struct createRes{
    msg: String,
    email: String,
}

#[post("/users/create")]
pub async fn create_user_api(data: web::Json<createReq>) -> Result<web::Json<createRes>, AppErr>{
    let user = User::new(&data.email, &data.password);
    match create_dbuser(&user).await {
        Ok(()) => Ok(
            web::Json(
                createRes {
                    msg: "create user success".to_string(),
                    email: user.email,
                })
        ),
        Err(v) => Err(v),
    }
}