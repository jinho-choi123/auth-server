use super::*;
use futures::executor::block_on;

pub fn create_user(email: &String, password: &String){
    let db_client = block_on(connect());
}