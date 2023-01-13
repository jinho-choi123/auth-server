use futures::executor::block_on;
use super::user::{User, UserStatus};
use super::{connect};
use mongodb::{Collection};
use crate::utils::errors::{AppErr, AppErrResponse, AppErrType};
use mongodb::bson::doc;

//create user in database
pub async fn create_dbuser(user: &User)->Result<(), AppErr> {
    let db_client = connect()
        .await?;
    let users: Collection<User> = db_client.database("auth").collection::<User>("users");

    //before inserting user into database, search if there are duplicate email
    match find_dbuser(&user.email).await? {
        Some(_) => return Err(AppErr::new(
            Some("Duplicate email found. Please check your email".to_string()),
            Some("duplicate key error in db".to_string()),
            AppErrType::DB_Err,
        )),
        None => (),
    }

    users
        .insert_one(user, None)
        .await
        .map_err(|err| {
            AppErr::new(
            Some("Error occur while creating user".to_string()),
            Some(format!("{:?}", err)),
            AppErrType::DB_Err,
        )})?;
    return Ok(());
}

//create user in database for test purpose
#[cfg(test)]
pub async fn test_create_user(user: &User)->Result<(), AppErr> {
    let db_client = connect()
        .await?;
    let users: Collection<User> = db_client.database("test").collection::<User>("users");

    //before inserting user into database, search if there are duplicate email
    match test_find_user(&user.email).await? {
        Some(_) => return Err(AppErr::new(
            Some("Duplicate email found. Please check your email".to_string()),
            Some("duplicate key error in db".to_string()),
            AppErrType::DB_Err,
        )),
        None => (),
    }

    users
        .insert_one(user, None)
        .await
        .map_err(|err| {
            AppErr::new(
            Some("Error occur while creating user".to_string()),
            Some(format!("{:?}", err)),
            AppErrType::DB_Err,
        )})?;
    return Ok(());
}

//find user in database by email
pub async fn find_dbuser(email: &String)->Result<Option<User>, AppErr>{
    let db_client = connect()
        .await?;
    let users = db_client.database("auth").collection::<User>("users");
    let filter = doc!{"email": email};
    let result = users
        .find_one(filter, None)
        .await
        .map_err(|err| AppErr::new(
            Some("Error occur while searching user".to_string()),
            Some(format!("{:?}", err)),
            AppErrType::DB_Err,
        ))?;
    return Ok(result);
}

//find user in database by email for test purpose
#[cfg(test)]
pub async fn test_find_user(email: &String)->Result<Option<User>, AppErr>{
    let db_client = connect()
        .await?;
    let users = db_client.database("test").collection::<User>("users");
    let filter = doc!{"email": email};
    let result = users
        .find_one(filter, None)
        .await
        .map_err(|err| AppErr::new(
            Some("Error occur while searching user".to_string()),
            Some(format!("{:?}", err)),
            AppErrType::DB_Err,
        ))?;
    return Ok(result);
}

//delete user in database by email
pub async fn delete_dbuser(email: &String)->Result<(), AppErr>{
    let db_client = connect()
        .await?;
    let users = db_client.database("auth").collection::<User>("users");

    //before deleting user from database, check if user exists in it.
    match find_dbuser(email).await? {
        Some(_) => (),
        None => return Err(AppErr::new(
            Some("User not found. Please check your email".to_string()),
            Some("Error occur while deleting user. User does not exist in the DB".to_string()),
            AppErrType::NotFound_Err,
        )),
    }
    let filter = doc!{"email": email};
    users.delete_one(filter, None)
    .await
    .map_err(|err| AppErr::new(
        Some("Error occur while connecting to DB".to_string()), 
        Some(format!("{:?}", err)), 
        AppErrType::DB_Err))?;
    return Ok(())
}


//verify if user exists in database
//then return jwt token if Ok
pub async fn verify_dbuser(email: &String, password: &String)->Result<String, AppErr>{
    let user_info = find_dbuser(email).await?;
    match user_info {
        Some(user) => {
            match User::verify(email, password, &user) {
                Ok(()) => return Ok("Verification Success. JWT Token blah blah".to_string()),
                Err(err) => return Err(AppErr::new(
                    Some("Invalid password. Please check your password again.".to_string()),
                    Some(format!("{:?}", err)),
                    AppErrType::Verification_Err,
                ))
            }
        },
        None => return Err(AppErr::new(
            Some("User does not exist. Please check user email.".to_string()),
            None,
            AppErrType::NotFound_Err,
        ))
    }
}

#[cfg(test)]
mod test{
    use super::{test_create_user};
    use crate::utils::errors::{AppErr, AppErrResponse, AppErrType};
    use crate::db::{init_test_db, connect, user::User};

    #[tokio::test]
    async fn test_create_user1()->Result<(), AppErr> {
        let db_client = connect()
            .await
            .map_err(|err| AppErr::new(
                Some("Error occur while connecting to DB".to_string()),
                Some(format!("{:?}", err)),
                AppErrType::DB_Err,
            ))?;
        let users = db_client.database("test").collection::<User>("users");
        let email = String::from("testemail@testemail.email");
        let password = String::from("testpasswordTestpwd");
        let user = User::new(&email, &password);
        test_create_user(&user).await?;
        return Ok(())
    }

    #[tokio::test]
    #[should_panic(expected="Duplicate email")]
    async fn test_create_user2() {
        let db_client = connect()
            .await
            .map_err(|err| AppErr::new(
                Some("Error occur while connecting to DB".to_string()),
                Some(format!("{:?}", err)),
                AppErrType::DB_Err,
            )).unwrap_or_else(|err| panic!("{:?}", err));
        let users = db_client.database("test").collection::<User>("users");
        let email = String::from("123testemail@testemail.email");
        let password = String::from("123testpasswordTestpwd");
        let user = User::new(&email, &password);
        test_create_user(&user).await.unwrap_or_else(|err| panic!("{:?}", err));
        test_create_user(&user).await.unwrap_or_else(|err| panic!("{:?}", err));

    }



    

}