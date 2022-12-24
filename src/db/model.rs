use futures::executor::block_on;
use super::user::{User, UserStatus};
use super::{connect, init_db};
use mongodb::{Collection, error::Error, error::ErrorKind};
use mongodb::bson::doc;

pub async fn create_user(user: &User)->Result<(), Error> {
    let db_client = connect().await?;
    let users: Collection<User> = db_client.database("auth").collection::<User>("users");
    users.insert_one(user, None).await?;
    return Ok(());
}

pub async fn find_user(email: &String)->Result<Option<User>, Error>{
    let db_client = connect().await?;
    let users = db_client.database("auth").collection::<User>("users");
    let filter = doc!{"email": email};
    let result = users.find_one(filter, None).await?;
    return Ok(result);
}

pub async fn delete_user(email: &String)->Result<(), Error>{
    let db_client = connect().await?;
    let users = db_client.database("auth").collection::<User>("users");
    let filter = doc!{"email": email};
    users.delete_one(filter, None).await?;
    return Ok(())
}

pub async fn verify_user(email: &String, password: &String)->Result<(), Error>{
    return Ok(())
}

#[cfg(test)]
mod test{
    use super::create_user;
    use super::User;
    use mongodb::error::Error;

    #[tokio::test]
    async fn test_create_user()->Result<(), Error>{
        let email = String::from("testtest@test.com");
        let password = String::from("testtesttest");
        create_user(&User::new(&email, &password)).await?;
        return Ok(())
    }

    //before running this testcase, delete all test datas in database.
    #[tokio::test]
    #[should_panic(expected="duplicate")]
    async fn test_create_duplicate_user(){
        let email = String::from("testduplicate@testduplicate.com");
        let password = String::from("passwordforduplicateuser");
        let user = User::new(&email, &password);
        create_user(&user).await.unwrap_or_else(|_|{panic!("Error occur when adding user first time")});
        create_user(&user).await.unwrap_or_else(|_|{panic!("Error occur when adding duplicate user")});

    }


}