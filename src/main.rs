use actix_web::{App, HttpServer};
mod routers;
mod db;
mod utils;
use db::{connect, init_db};
use routers::check_server;

#[actix_web::main]
async fn main()->std::io::Result<()> {
    init_db(&connect().await.unwrap_or_else(|_|{panic!("Error occur while initializing DB")})).await;
    HttpServer::new(|| {
        App::new()
            .service(check_server)
    })
    .bind(("0.0.0.0", 9090))?
    .run()
    .await
}
