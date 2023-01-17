use actix_web::{get,HttpResponse, Responder, web, error::ResponseError, http::StatusCode};
use crate::db::model::{verify_dbuser, store_refresh_jwt};
use crate::utils::jwt::{create_jwt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct loginReq{
    email: String,
    password: String,
}

#[derive(Serialize)]
struct JWTResponse {
    accessToken: String,
    refreshToken: String,
}

#[get("/users/login")]
pub async fn login_user_api(data: web::Json<loginReq>) ->impl Responder {
    match verify_dbuser(&data.email, &data.password).await {
        Ok(()) => {
            //db에 User가 존재하므로, jwt token을 만든다.
            match create_jwt(&data.email) {
                Ok((accessToken, refreshToken)) => {
                    match store_refresh_jwt(&refreshToken, &data.email).await {
                    Ok(()) => {
                        let jwtResponse = JWTResponse {
                            accessToken: accessToken,
                            refreshToken: refreshToken,
                        };
                        HttpResponse::Ok().json(web::Json(jwtResponse))
                    },
                    Err(err) => err.error_response(),
                    }
                },
                Err(err) => err.error_response(),
            }
        },
        Err(v) => v.error_response(),
    }
}   