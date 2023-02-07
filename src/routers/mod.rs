use actix_web::{get, HttpResponse, web};
use chrono::prelude::*;
use serde::Serialize;
pub mod users;

#[derive(Serialize)]
struct server_status {
    msg: String
} 

#[get("/checkserver")]
pub async fn check_server() -> web::Json<server_status>{
    let local: DateTime<Local> = Local::now();
    web::Json( server_status{
        msg: format!("Server is running. Server time is {}", local.format("%Y-%m-%d %H:%M:%S").to_string())
    })
}
