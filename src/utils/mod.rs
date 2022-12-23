use rand::{thread_rng, Rng};
use std::env;

pub fn rand_salt()->String {
    let mut rng = thread_rng();
    let salt_size = rng.gen_range(30..50);
    let salt: String = (0..salt_size).map(|_| format!("{:x}", rng.gen_range(0..256))).collect();
    return salt;
}

pub fn salt_padding(init_salt: &String){
    let uni_hash1 = env::var("DB_UNI_SALT1").expect("DB_UNI_SALT1 env variable not set");
    let uni_hash2 = env::var("DB_UNI_SALT2").expect("DB_UNI_SALT2 env variable not set");

    init_salt.push_str(&uni_hash1);
    init_salt.insert_str(0, &uni_hash2);
}