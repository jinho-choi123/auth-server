use actix_web::{get,HttpResponse, Responder,HttpRequest, web, error::ResponseError, http::StatusCode};
use crate::db::model::{verify_dbuser, store_refresh_jwt};
use crate::utils::jwt::{validate_access_jwt};
use crate::utils::errors::{AppErr, AppErrType};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct verifyRes {
    msg: String,
    email: String,
}


#[get("/jwt/verify")]
pub async fn verify_user_api(req: HttpRequest) -> Result<web::Json<verifyRes>, AppErr> {
    match req.headers().get("Authorization") {
        Some(accessToken) => {
            let jwt = accessToken.to_str().unwrap().replace("Bearer ", "");
            
            match validate_access_jwt(&jwt) {
                Ok(email) => {
                    Ok(
                        web::Json(
                            verifyRes {
                                email: email,
                                msg: "verification success. User is authorized.".to_string(),
                            }
                        )
                    )
                },
                Err(err) => Err(err),
            }
        },
        None => Err(AppErr::new(None, None, AppErrType::NoAuthHeader_Err))
    }
}