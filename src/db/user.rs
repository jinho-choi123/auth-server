use crate::utils::{rand_salt, hash};
use std::fmt;
pub enum UserStatus {
    Admin,
    Active,
    Pending,
    InActive,
}

pub struct User {
    email: String,
    status: UserStatus,
    password: String,
    hashSalt: String,
}

impl User {
    pub fn create_user(email: &String, password: &String)->User{
        let init_salt = rand_salt();

        let hashed_password = hash(&init_salt, password);
        User { email: email.to_string(), status: UserStatus::Pending, password: hashed_password, hashSalt: init_salt}
    }

    pub fn verify_user(email: & String, password: & String, user_info: & User)->Result<(), &'static str>{
        if *email != user_info.email {
            return Err("User Email doesn\'t match.");
        }
        let password_hash = hash(&user_info.hashSalt, password);
        if password_hash != user_info.password {
            return Err("Password doesn\'t match. ");
        }
        return Ok(());

    }
}

#[cfg(not(debug_assertions))]
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(email: {}, status: {}, password: {}, hashSalt: {})", self.email, self.status, self.password, self.hashSalt)
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_user_create(){
        let email = String::from("jinho1234@1234.1234");
        let password = String::from("hello world!!!");
        let user_info = User::create_user(&email, &password);
        assert_eq!(user_info.email, email);
    }

    #[test]
    fn test_user_create2(){
        let email = String::from("jinho1234@1234.1234");
        let password = String::from("hello world!!!");
        let user_info = User::create_user(&email, &password);
        let password_hash = hash(&user_info.hashSalt, &password);
        assert_eq!(password_hash, user_info.password);
    }

    #[test]
    fn test_user_verify()->Result<(), &'static str>{
        let email = String::from("jinho1234@1234.1234");
        let password = String::from("hello world!!!");
        let user_info = User::create_user(&email, &password);
        let result = User::verify_user(&email, &password, &user_info);
        return result;
    }
}