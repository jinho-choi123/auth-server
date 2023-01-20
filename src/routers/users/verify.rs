use actix_web::{get,HttpResponse, Responder,HttpRequest, web, error::ResponseError, http::StatusCode};
use crate::db::model::{verify_dbuser, store_refresh_jwt};
use crate::utils::jwt::{validate_access_jwt};
use crate::utils::errors::{AppErr, AppErrType};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct VerifiedUser_Response {
    username: String,
}


#[get("/jwt/verify")]
pub async fn verify_user_api(req: HttpRequest) -> impl Responder {
    match req.headers().get("Authorization") {
        Some(accessToken) => {
            let jwt = accessToken.to_str().unwrap().replace("Bearer ", "");
            
            match validate_access_jwt(&jwt) {
                Ok(username) => {
                    let username_response = VerifiedUser_Response {
                        username: username
                    };
                    HttpResponse::Ok().json(web::Json(username_response))
                },
                Err(err) => err.error_response(),
            }
        },
        None => AppErr::new(None, None, AppErrType::NoAuthHeader_Err).error_response()
    }
}