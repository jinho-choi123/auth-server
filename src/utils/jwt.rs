use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm::HS512, DecodingKey, EncodingKey, Validation, Header
};
use std::env;
use crate::{utils::errors::{AppErr, AppErrResponse, AppErrType}, db::model::get_stored_refresh_jwt};
use serde::{Deserialize, Serialize};
use crate::utils::str2int::parse2u64;
use super::str2int;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: u64, 
    iat: u64,
    username: String,
}

fn get_EncodingKey()->EncodingKey {
    let uni_salt = env::var("JWT_SALT").expect("JWT_SALT env variable not set");
    return EncodingKey::from_secret(uni_salt.as_bytes())
}
fn get_DecodingKey()->DecodingKey {
    let uni_salt = env::var("JWT_SALT").expect("JWT_SALT env variable not set");
    return DecodingKey::from_secret(uni_salt.as_bytes())
}

pub fn create_jwt(username: &String)->Result<(String, String), AppErr> {
    let access_jwt_lifetime: u64 = parse2u64(&env::var("ACCESS_JWT_LIFETIME").unwrap());
    let refresh_jwt_lifetime: u64 = parse2u64(&env::var("REFRESH_JWT_LIFETIME").unwrap());
    let header = Header::new(HS512);
    let access_payload = Claims {
        sub: "access".to_string(),
        exp: get_current_timestamp()+access_jwt_lifetime,
        iat: get_current_timestamp(),
        username: username.to_string(),
    };

    let refresh_payload = Claims {
        sub: "refresh".to_string(),
        exp: get_current_timestamp()+refresh_jwt_lifetime,
        iat: get_current_timestamp(),
        username: username.to_string(),
    };

    let access_jwt: String = match encode(&header, &access_payload, &get_EncodingKey()) {
        Ok(access_token) => access_token,
        Err(err) => return Err(AppErr::new(
            Some("Error occur while encoding jwt.".to_string()),
            Some(format!("{:?}", err)),
            AppErrType::JWT_Err,
        ))
    };

    let refresh_jwt: String= match encode(&header, &refresh_payload, &get_EncodingKey()) {
        Ok(refresh_token) => {
            //refresh token을 database에 저장한다.
            
            refresh_token
        },
        Err(err) => return Err(AppErr::new(
            Some("Error occur while encoding jwt.".to_string()),
            Some(format!("{:?}", err)),
            AppErrType::JWT_Err,
        ))
    };

    return Ok((access_jwt, refresh_jwt))
}

pub fn validate_access_jwt(jwt: &String)->Result<String, AppErr> {
    let validation = Validation::new(HS512);
    let validate_result = match decode_jwt(jwt) {
        Ok(claim) => {
            if claim.sub == String::from("access") {
                Ok(claim)
            } else {
                Err(AppErr::new(
                    Some(
                        "Token you provided is not Access token.".to_string()), 
                        Some("Token you provided is not Access token.".to_string()),
                        AppErrType::NotValidToken_Err,
                ))
            }
        },
        Err(err) => Err(err)
    }?;
    //check for expiration time
    println!("validate_result is {}", validate_result.exp);
    println!("current timestamp is {}", get_current_timestamp());
    if validate_result.exp < get_current_timestamp() {
        return Err(AppErr::new(
            Some("JWT Token has expired. Please refresh access token.".to_string()),
            Some("JWT Token has expired. Please refresh access token.".to_string()),
            AppErrType::JwtAccessExpired_ERR,
        ))
    }
    return Ok(validate_result.username)
}

pub async fn validate_refresh_jwt(jwt: &String)->Result<String, AppErr> {
    let validation = Validation::new(HS512);
    let validate_result = match decode_jwt(jwt) {
        Ok(claim) => {
            if claim.sub == String::from("refresh") {
                Ok(claim)
            } else {
                Err(AppErr::new(
                    Some(
                        "Provided Token is not Refresh token.".to_string()), 
                        Some("Provided Token is not Refresh token.".to_string()),
                        AppErrType::NotValidToken_Err,
                ))
            }
        },
        Err(err) => Err(err)
    }?;

    //check if refresh token is stored in database.
    let userEmail = validate_result.username;
    
    match get_stored_refresh_jwt(&userEmail).await {
        Ok(Some(refreshToken)) => {
            if refreshToken != *jwt {
                return Err(AppErr::new(
                    Some("JWT RefreshToken you provided is not in Database".to_string()),
                    Some("JWT RefreshToken you provided is not in Database".to_string()),
                    AppErrType::NotFound_Err,
                ))
            }
        },
        Ok(None) => return Err(
            AppErr::new(
                Some("JWT Refresh Token is not valid. It is not stored in dataabase.".to_string()),
            Some("JWT Refresh Token is not stored in DB.".to_string()),
            AppErrType::NotValidToken_Err,
            )
        ),
        Err(err) => return Err(err)
    }

    //check for expiration time
    if validate_result.exp < get_current_timestamp() {
        return Err(AppErr::new(
            Some("JWT Token has expired. Please refresh refresh token.".to_string()),
            Some("JWT Token has expired. Please refresh refresh token.".to_string()),
            AppErrType::JwtRefreshExpired_ERR,
        ))
    }
    return Ok(userEmail)
}

pub fn decode_jwt(jwt: &String)->Result<Claims, AppErr> {
    match decode::<Claims>(
        jwt,
        &get_DecodingKey(),
        &Validation::new(HS512),
    ) {
        Ok(c) => return Ok(c.claims),
        Err(err) => return Err(AppErr::new(
            Some("Error occur while decoding jwt.".to_string()),
            Some(format!("{:?}", err)),
            AppErrType::JWT_Err,
        ))
    }
}

pub fn refresh2access_jwt(refresh_token: &String) -> Result<String, AppErr> {
    let userName = match decode_jwt(refresh_token) {
        Ok(claims) => Ok(claims.username),
        Err(err) => Err(err),
    }?;
    let access_token = match create_jwt(&userName) {
        Ok((access_token, _)) => Ok(access_token),
        Err(err) => Err(err)
    }?;
    return Ok(access_token)
}

#[cfg(test)]
mod test {
    use crate::{utils::errors::AppErr, routers::users::create};

    use super::{create_jwt, validate_access_jwt, validate_refresh_jwt,decode_jwt};

    #[test]
    fn test_create_jwt()->Result<(), AppErr> {
        let token = create_jwt(&"mango_cookie".to_string())?;
        return Ok(());
    }

    #[test]
    fn test_validate_jwt()->Result<(), AppErr> {
        let (accessToken, refreshToken) = create_jwt(&"mingo_kookie".to_string())?;
        validate_access_jwt(&accessToken)?;
        return Ok(())
    }

    #[tokio::test]
    async fn test_decode_jwt()->Result<(), AppErr> {
        let (accessToken, refreshToken) = create_jwt(&"mingo_kookie".to_string())?;
        validate_access_jwt(&accessToken)?;
        validate_refresh_jwt(&refreshToken).await?;
        println!("{:?}", decode_jwt(&accessToken));
        return Ok(())
    }

}