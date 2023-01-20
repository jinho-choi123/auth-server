use actix_web::{get,HttpResponse, Responder, web, error::ResponseError, http::StatusCode};
use crate::db::model::{find_dbuser, store_refresh_jwt};
use crate::utils::jwt::{create_jwt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct logoutReq{
    email: String,
}

#[get("/users/logout")]
pub async fn logout_user_api(data: web::Json<logoutReq>) -> impl Responder {
    match find_dbuser(&data.email).await {
        Ok(_) => {
            match store_refresh_jwt(&String::new(), &data.email).await {
                Ok(()) => {
                    HttpResponse::Ok().body("logout success!")
                },
                Err(err) => err.error_response(),
            }
        },
        Err(err) => err.error_response()
    }
}