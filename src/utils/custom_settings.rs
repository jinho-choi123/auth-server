use std::env;
use std::str;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


fn generate_random_hash()->String {
    let salt: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(256)
        .map(char::from)
        .collect();
    return salt
}

pub fn init_auth_options() {
    match env::var("DB_PWD_SALT1") {
        Err(_) => env::set_var("DB_PWD_SALT1", generate_random_hash()),
        Ok(_) => (), 
    }

    match env::var("DB_PWD_SALT2") {
        Err(_) => env::set_var("DB_PWD_SALT1", generate_random_hash()),
        Ok(_) => (), 
    }

    match env::var("MONGO_URL") {
        Ok(_) => (),
        Err(_) => panic!("MONGODB_URL is required. Please set MONGO_URL environment variable.")
    }

    match env::var("PORT") {
        Ok(_) => (),
        Err(_) => env::set_var("PORT", "9090"),
    }

    match env::var("JWT_SALT") {
        Ok(_) => (),
        Err(_) => env::set_var("JWT_SALT", generate_random_hash()),
    }

    match env::var("ACCESS_JWT_LIFETIME") {
        Ok(_) => (),
        Err(_) => env::set_var("ACCESS_JWT_LIFETIME", "3600"),
    }

    match env::var("REFRESH_JWT_LIFETIME") {
        Ok(_) => (),
        Err(_) => env::set_var("REFRESH_JWT_LIFETIME", "86400"),
    }


}

#[cfg(test)]
mod test {
    use super::generate_random_hash;
    #[test]
    fn test_hash_generator() {
        generate_random_hash();
        generate_random_hash();
    }
}