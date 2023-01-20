use std::env;

use actix_web::{App, HttpServer};
mod routers;
mod db;
mod utils;
use db::{connect, init_db, init_test_db};
use routers::check_server;
use routers::users::{refreshtoken::refresh2access_api, verify::verify_user_api,logout::logout_user_api,login::login_user_api, create::create_user_api, delete::delete_user_api};
use utils::str2int::parse2u16;

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
            .service(logout_user_api)
            .service(verify_user_api)
            .service(refresh2access_api)
    })
    .bind(("0.0.0.0", parse2u16(&env::var("PORT").unwrap())))?
    .run()
    .await
}
