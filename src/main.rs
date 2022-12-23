use actix_web::{App, HttpServer};
mod routers;
mod db;
mod utils;
use db::connect;
use routers::check_server;

#[actix_web::main]
async fn main()->std::io::Result<()> {
    connect().await;
    HttpServer::new(|| {
        App::new()
            .service(check_server)
    })
    .bind(("0.0.0.0", 9090))?
    .run()
    .await
}
