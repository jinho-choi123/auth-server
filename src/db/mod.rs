use mongodb::{Client, options::ClientOptions, error::Error};
use std::env;
mod user;
mod model;

pub async fn connect()->Result<Client, Error>{
    let mongo_url = env::var("MONGO_URL").expect("$MONGO_URL doesnt exist!");
    let client_options = ClientOptions::parse(mongo_url).await?;

    let client = Client::with_options(client_options)?;
    println!("############Connected to MONGODB###############");
    return Ok(client)
}

#[cfg(test)]

mod test{
    use std::env;
    use mongodb::{Client, options::ClientOptions};
    use futures::executor::block_on;
    use mongodb::error::Error;
    #[tokio::test]
    async fn test_db_connection()->Result<(), Error>{
        let mongo_url = env::var("MONGO_URL").expect("$MONGO_URL doesnt exist!");
        let client_options = block_on(ClientOptions::parse(mongo_url))?;
        let client = Client::with_options(client_options)?;
        return Ok(());
    }
}