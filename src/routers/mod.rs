use actix_web::{get, HttpResponse, Responder};
use chrono::prelude::*;
pub mod users;

#[get("/checkserver")]
pub async fn check_server() -> impl Responder{
    let local: DateTime<Local> = Local::now();
    HttpResponse::Ok().body(format!("Server is Running. ServerTime is {}", local.format("%Y-%m-%d %H:%M:%S").to_string()))
}

