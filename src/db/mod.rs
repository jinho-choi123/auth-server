use mongodb::{Client, options::ClientOptions, error::Error, options::IndexOptions, IndexModel, bson::doc};
use std::env;
use user::User;
mod user;
mod model;

pub async fn connect()->Result<Client, Error>{
    let mongo_url = env::var("MONGO_URL").expect("$MONGO_URL doesnt exist!");
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client = Client::with_options(client_options)?;
    println!("############Connected to MONGODB###############");
    return Ok(client)
}

pub async fn init_db(client: &Client){
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc!{"email": 1})
        .options(options)
        .build();

    client
        .database("auth")
        .collection::<User>("users")
        .create_index(model, None)
        .await
        .expect("Error occur while initializing database");
}

#[cfg(test)]
mod test{
    use super::{connect, init_db};    
    use mongodb::error::Error;
    #[tokio::test]
    async fn test_db_connection()->Result<(), Error>{
        let db_client = connect().await?;
        init_db(&db_client).await;
        return Ok(());
    }
}