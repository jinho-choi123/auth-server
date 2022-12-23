use mongodb::{Client, options::ClientOptions};
use std::env;

pub async fn connect()->Result<Client, mongodb::error::Error>{
    let mongo_url = env::var("MONGO_URL").expect("$MONGO_URL doesnt exist!");
    let client_options = ClientOptions::parse(mongo_url).await?;

    let client = Client::with_options(client_options)?;
    println!("############Connected to MONGODB###############");
    println!("Printing DB List");
    for db_name in client.list_database_names(None, None).await?{
        println!("{}", db_name);
    }
    return Ok(client)
}
