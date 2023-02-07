use actix_web::{post,HttpResponse, Responder, web, error::ResponseError, http::StatusCode};
use crate::db::model::{verify_dbuser, store_refresh_jwt};
use crate::utils::errors::AppErr;
use crate::utils::jwt::{create_jwt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct loginReq{
    email: String,
    password: String,
}

#[derive(Serialize)]
struct loginRes {
    accessToken: String,
    refreshToken: String,
}

#[post("/users/login")]
pub async fn login_user_api(data: web::Json<loginReq>) -> Result<web::Json<loginRes>, AppErr> {
    match verify_dbuser(&data.email, &data.password).await {
        Ok(()) => {
            //db에 User가 존재하므로, jwt token을 만든다.
            match create_jwt(&data.email) {
                Ok((accessToken, refreshToken)) => {
                    match store_refresh_jwt(&refreshToken, &data.email).await {
                    Ok(()) => {
                        Ok(
                            web::Json(
                                loginRes {
                                    accessToken: accessToken,
                                    refreshToken: refreshToken,
                                }
                            )
                        )
                        
                    },
                    Err(err) => Err(err),
                    }
                },
                Err(err) => Err(err),
            }
        },
        Err(err) => Err(err),
    }
}   