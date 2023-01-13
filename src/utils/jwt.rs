use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm::HS512, DecodingKey, EncodingKey, Validation, Header
};
use std::env;
use crate::utils::errors::{AppErr, AppErrResponse, AppErrType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: u64, 
    iat: u64,
    username: String,
}

fn get_EncodingKey()->EncodingKey {
    let uni_salt = env::var("DB_UNI_SALT1").expect("DB_UNI_SALT1 env variable not set");
    return EncodingKey::from_secret(uni_salt.as_bytes())
}
fn get_DecodingKey()->DecodingKey {
    let uni_salt = env::var("DB_UNI_SALT1").expect("DB_UNI_SALT1 env variable not set");
    return DecodingKey::from_secret(uni_salt.as_bytes())
}

pub fn create_jwt(username: &String)->Result<String, AppErr> {
    let fivemin: u64 = 1000*60*5;
    let header = Header::new(HS512);
    let payload = Claims {
        sub: "auth".to_string(),
        exp: get_current_timestamp()+fivemin,
        iat: get_current_timestamp(),
        username: username.to_string(),
    };

    let jwt = match encode(&header, &payload, &get_EncodingKey()) {
        Ok(t) => Ok(t),
        Err(err) => return Err(AppErr::new(
            Some("Error occur while encoding jwt.".to_string()),
            Some(format!("{:?}", err)),
            AppErrType::JWT_Err,
        ))
    };

    return jwt
}

pub fn validate_jwt(jwt: &String)->Result<(), AppErr> {
    let validation = Validation::new(HS512);
    let validate_result = match decode::<Claims> (
        jwt,
        &get_DecodingKey(),
        &validation,
    ) {
        Ok(c) => Ok(c),
        Err(err) => return Err(AppErr::new(
            Some("Error occur while decoding jwt.".to_string()),
            Some(format!("{:?}", err)),
            AppErrType::JWT_Err,
        ))
    }?;

    //check for expiration time
    if validate_result.claims.exp < get_current_timestamp() {
        return Err(AppErr::new(
            Some("JWT Token has expired. Please use another token.".to_string()),
            Some("JWT Token has expired. Please use another token.".to_string()),
            AppErrType::JWT_Err,
        ))
    }

    return Ok(())

}

#[cfg(test)]
mod test {
    use crate::{utils::errors::AppErr, routers::users::create};

    use super::{create_jwt, validate_jwt};

    #[test]
    fn test_create_jwt()->Result<(), AppErr> {
        let token = create_jwt(&"mango_cookie".to_string())?;
        return Ok(());
    }

    #[test]
    fn test_validate_jwt()->Result<(), AppErr> {
        let token = create_jwt(&"mingo_kookie".to_string())?;
        validate_jwt(&token)?;
        return Ok(())
    }

}