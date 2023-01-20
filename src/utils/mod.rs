use rand::{thread_rng, Rng};
use sha2::{Sha256, Digest};
use std::env;
pub mod errors;
pub mod jwt;
pub mod custom_settings;

pub fn rand_salt()->String {
    let mut rng = thread_rng();
    let salt_size = rng.gen_range(30..50);
    let salt: String = (0..salt_size).map(|_| format!("{:x}", rng.gen_range(0..256))).collect();
    return salt;
}

fn salt_padding(init_salt: &mut String)->&String{
    let uni_hash1 = env::var("DB_UNI_SALT1").expect("DB_UNI_SALT1 env variable not set");
    let uni_hash2 = env::var("DB_UNI_SALT2").expect("DB_UNI_SALT2 env variable not set");
    init_salt.push_str(&uni_hash1);
    init_salt.insert_str(0, &uni_hash2);
    return init_salt;
}

pub fn hash(init_salt: &String, password: &String)->String{
    let mut hasher = Sha256::default();
    let mut hash_salt = init_salt.clone();
    salt_padding(&mut hash_salt);
    hasher.update(hash_salt);
    hasher.update(password);
    let hashed_password = format!("{:x}", hasher.finalize());
    return hashed_password;
}

#[cfg(test)]
mod test{
    use crate::utils::{rand_salt, salt_padding};
    #[test]
    fn test_salt_padding1(){
        let mut init_salt = rand_salt();
        salt_padding(&mut init_salt);
    }
}