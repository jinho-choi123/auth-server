use actix_web::{post,HttpResponse, Responder,HttpRequest, web, error::ResponseError, http::StatusCode};
use crate::db::model::{verify_dbuser, store_refresh_jwt};
use crate::utils::jwt::{validate_refresh_jwt, refresh2access_jwt};
use crate::utils::errors::{AppErr, AppErrType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct refreshReq {
    refresh_token: String,
}

#[derive(Serialize)]
struct refreshRes {
    accessToken: String,
    msg: String,
}

#[post("/jwt/refresh")]
pub async fn refresh2access_api(data: web::Json<refreshReq>)-> Result<web::Json<refreshRes>, AppErr>{
    match validate_refresh_jwt(&data.refresh_token).await {
        Ok(email) => {
            match refresh2access_jwt(&data.refresh_token) {
                Ok(access_token) => 
                    match store_refresh_jwt(&String::new(), &email).await {
                        Ok(()) => Ok(
                                    web::Json(
                                        refreshRes {
                                            accessToken: access_token,
                                            msg: "refreshing token success.".to_string(),
                                        }
                                    )
                                ),
                        Err(err) => Err(err),
                    },
                Err(err) => Err(err),
            }
        
        },
        Err(err) => Err(err),
    }
}