use futures::executor::block_on;
use super::user::{User, UserStatus};
use super::{connect, init_db};
use mongodb::{Collection, error::Error, error::ErrorKind};
use mongodb::bson::doc;

//create user in database
pub async fn create_user(user: &User)->Result<(), Error> {
    let db_client = connect().await?;
    let users: Collection<User> = db_client.database("auth").collection::<User>("users");
    users.insert_one(user, None).await?;
    return Ok(());
}

//find user in database by email
pub async fn find_user(email: &String)->Result<Option<User>, Error>{
    let db_client = connect().await?;
    let users = db_client.database("auth").collection::<User>("users");
    let filter = doc!{"email": email};
    let result = users.find_one(filter, None).await?;
    return Ok(result);
}

//delete user in database by email
pub async fn delete_user(email: &String)->Result<(), Error>{
    let db_client = connect().await?;
    let users = db_client.database("auth").collection::<User>("users");
    let filter = doc!{"email": email};
    users.delete_one(filter, None).await?;
    return Ok(())
}


#[derive(Debug, PartialEq, Eq)]
enum VerificationStatus {
    Success,
    Fail(String),
}
//verify if user exists in database
pub async fn verify_user(email: &String, password: &String)->Result<VerificationStatus, Error>{
    let user_info = find_user(email).await?;
    match user_info {
        None => Ok(VerificationStatus::Fail(String::from("User is not registered."))),
        Some(v) => {
            match User::verify(email, password, &v) {
                Ok(()) => return Ok(VerificationStatus::Success),
                Err(_) => return Ok(VerificationStatus::Fail(String::from("User password doesn\'t match."))),
            }
        }
    }
}

#[cfg(test)]
mod test{
    use super::create_user;
    use super::User;
    use mongodb::error::Error;
    use super::{verify_user, VerificationStatus};

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

    #[tokio::test]
    async fn test_verify_user1() {
        let email = String::from("testverify@testverify.com");
        let password = String::from("passwordforverifyinguser");
        let user = User::new(&email, &password);
        create_user(&user).await;
        assert_eq!(verify_user(&email, &password).await.unwrap(), VerificationStatus::Success);
    }


}