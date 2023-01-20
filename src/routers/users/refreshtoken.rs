use actix_web::{post,HttpResponse, Responder,HttpRequest, web, error::ResponseError, http::StatusCode};
use crate::db::model::{verify_dbuser, store_refresh_jwt};
use crate::utils::jwt::{validate_refresh_jwt, refresh2access_jwt};
use crate::utils::errors::{AppErr, AppErrType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct refreshReq {
    refresh_token: String,
}

#[post("/jwt/refresh")]
pub async fn refresh2access_api(data: web::Json<refreshReq>)-> impl Responder {
    match validate_refresh_jwt(&data.refresh_token) {
        Ok(()) => {
            match refresh2access_jwt(&data.refresh_token) {
                Ok(access_token) => HttpResponse::Ok().body(access_token),
                Err(err) => err.error_response()
            }
        
        },
        Err(err) => err.error_response(),
    }
}