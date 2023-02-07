use actix_web::{post, delete, HttpResponse, Responder, web, error::ResponseError, http::StatusCode};
use crate::db::user::User;
use crate::db::model::{delete_dbuser};
use crate::utils::errors::AppErr;
use serde::{Serialize};

#[derive(Serialize)]
struct deleteRes{
    msg: String,
    email: String,
}

#[delete("/users/delete/{user_email}")]
pub async fn delete_user_api(path: web::Path<String>) -> Result<web::Json<deleteRes>, AppErr> {
    let user_email = path.into_inner();
    match delete_dbuser(&user_email).await {
    Ok(()) => Ok(web::Json(
        deleteRes {
            msg: "User deletion success.".to_string(),
            email: user_email,
        }
    )),
    Err(v) => Err(v),
    }
}