use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod routers;
use routers::check_server;

#[actix_web::main]
async fn main()->std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(check_server)
    })
    .bind(("0.0.0.0", 9090))?
    .run()
    .await
}
