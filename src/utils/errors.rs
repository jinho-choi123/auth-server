use serde::Serialize;
use actix_web::{ResponseError, http::StatusCode, HttpResponse};
use std::fmt;

#[derive(Debug)]
pub enum AppErrType {
    DB_Err,
    NotFound_Err,
    Verification_Err,
    JWT_Err
}

#[derive(Debug)]
pub struct AppErr {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub errorType: AppErrType,
}
impl fmt::Display for AppErr{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AppErr {
    pub fn new(message: Option<String>, cause: Option<String>, errorType: AppErrType) -> AppErr{
        return AppErr { message: message, cause: cause, errorType: errorType }
    }
    pub fn message(&self) -> String {
        match self{
            AppErr{
                message: Some(message), 
                cause: _, 
                errorType: _
            } => message.clone(),
            AppErr{
                message: None, 
                cause: _, 
                errorType: AppErrType::DB_Err,
            } => "Error occur while DB operations.".to_string(),
            AppErr { 
                message: None, 
                cause: _, 
                errorType: AppErrType::NotFound_Err 
            } => "The request item was not found.".to_string(),
            AppErr {
                message: None,
                cause: _,
                errorType: AppErrType::Verification_Err,
            } => "Verification error. Please check if your account has enough privilege.".to_string(),
            AppErr {
                message: None,
                cause: _,
                errorType: AppErrType::JWT_Err,
            } => "Error occur while processing with JWT. Please contact service team.".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct AppErrResponse {
    pub error: String,
}

impl ResponseError for AppErr {
    fn status_code(&self) -> StatusCode {
        match self.errorType{
            AppErrType::DB_Err => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrType::NotFound_Err => StatusCode::NOT_FOUND,
            AppErrType::Verification_Err => StatusCode::BAD_REQUEST,
            AppErrType::JWT_Err => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .json(AppErrResponse {
                error: self.message(),
            })
    }
}

