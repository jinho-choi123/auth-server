use mongodb::{Client, options::ClientOptions};
use std::env;
mod user;

pub async fn connect()->Result<Client, mongodb::error::Error>{
    let mongo_url = env::var("MONGO_URL").expect("$MONGO_URL doesnt exist!");
    let client_options = ClientOptions::parse(mongo_url).await?;

    let client = Client::with_options(client_options)?;
    println!("############Connected to MONGODB###############");
    return Ok(client)
}
