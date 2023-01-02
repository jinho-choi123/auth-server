use actix_web::{post, delete, HttpResponse, Responder, web, error::ResponseError, http::StatusCode};
use crate::db::user::User;
use crate::db::model::delete_user;

#[delete("/users/delete/{user_email}")]
pub async fn delete_user_api(path: web::Path<String>) -> impl Responder{
    let user_email = path.into_inner();
    match delete_user(&user_email).await {
    Ok(()) => HttpResponse::Ok().body(format!("User with email {} delete success!", user_email)),
    Err(v) => v.error_response(),
    }
}