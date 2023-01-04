use actix_web::{App, HttpServer};
mod routers;
mod db;
mod utils;
use db::{connect, init_db, init_test_db};
use routers::check_server;
use routers::users::{login::login_user_api, create::create_user_api, delete::delete_user_api};

#[actix_web::main]
async fn main()->std::io::Result<()> {
    init_db(&connect().await.unwrap_or_else(|_|{panic!("Error occur while initializing DB")})).await;

    //development purpose only!! 
    init_test_db(&connect().await.unwrap_or_else(|_|{panic!("Error occur while initializing DB")})).await;

    HttpServer::new(|| {
        App::new()
            .service(check_server)
            .service(create_user_api)
            .service(delete_user_api)
            .service(login_user_api)
    })
    .bind(("0.0.0.0", 9090))?
    .run()
    .await
}
