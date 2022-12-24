use crate::utils::{rand_salt, hash};
use std::fmt;
use serde::{Deserialize, Serialize, Serializer, Deserializer,};
use serde::de::Error;

#[derive(Debug)]
pub enum UserStatus {
    Admin,
    Active,
    Pending,
    InActive,
}
impl Serialize for UserStatus{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        match *self{
            UserStatus::Admin => serializer.serialize_str("Admin"),
            UserStatus::Active => serializer.serialize_str("Active"),
            UserStatus::Pending => serializer.serialize_str("Pending"),
            UserStatus::InActive => serializer.serialize_str("InActive"),
        }
    }
}
impl<'de> Deserialize<'de> for UserStatus{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        let s: &str = Deserialize::deserialize(deserializer)?;
        match s {
            "Admin" => Ok(UserStatus::Admin),
            "Active" => Ok(UserStatus::Active),
            "Pending" => Ok(UserStatus::Pending),
            "InActive" => Ok(UserStatus::InActive),
            _ => Err(Error::custom("Invalid User Status during deserializing.")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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