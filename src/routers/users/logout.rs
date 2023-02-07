use actix_web::{get,HttpResponse, Responder, web, error::ResponseError, http::StatusCode};
use crate::db::model::{find_dbuser, clear_refresh_jwt};
use crate::utils::errors::AppErr;
use crate::utils::jwt::{create_jwt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct logoutReq{
    email: String,
}

#[derive(Serialize)]
struct logoutRes {
    msg: String,
    email: String,
}

#[get("/users/logout")]
pub async fn logout_user_api(data: web::Json<logoutReq>) -> Result<web::Json<logoutRes>, AppErr> {
    let userEmail = data.email.clone();
    match find_dbuser(&userEmail).await {
        Ok(_) => {
            match clear_refresh_jwt(&String::new()).await {
                Ok(()) => {
                    Ok(
                        web::Json(
                            logoutRes {
                                msg: "logout success.".to_string(),
                                email: userEmail,
                            }
                        )
                    )
                },
                Err(err) => Err(err),
            }
        },
        Err(err) => Err(err),
    }
}