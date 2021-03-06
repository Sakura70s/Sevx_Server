use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;
use chrono::Local;

/**
 * 自定义错误类型
 */
#[derive(Debug, Serialize)]
pub enum SEVXError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    AuthFailed(String),
    // InvalidTnput(String),
}

/**
 * 错误响应内容
 */
#[derive(Debug, Serialize)]
pub struct SEVXErrorResponse {
    error_message: String,
}

/**
 * 实现枚举的一些方法
 */
impl SEVXError {
    fn error_response(&self) -> String {
        match self {
            SEVXError::DBError(msg) => {
                let fmt = "%Y-%m-%d %H:%M:%S";
                let now = Local::now().format(fmt);
                println!("{}  Database Error: {:?}",now, msg);
                "Database Error".into()
            },
            SEVXError::ActixError(msg) => {
                let fmt = "%Y-%m-%d %H:%M:%S";
                let now = Local::now().format(fmt);
                println!("{}  Actix Error: {:?}",now, msg);
                "Actix Error".into()
            },
            SEVXError::NotFound(msg) => {
                let fmt = "%Y-%m-%d %H:%M:%S";
                let now = Local::now().format(fmt);
                println!("{}  Not Found: {:?}",now, msg);
                "Not Found".into()
            },
            SEVXError::AuthFailed(msg) => {
                let fmt = "%Y-%m-%d %H:%M:%S";
                let now = Local::now().format(fmt);
                println!("{}  Auth Failed: {:?}",now, msg);
                "Auth Failed".into()
            },
            // SEVXError::InvalidTnput(msg) => {
            //     println!("Invalid parameters received: {:?}", msg);
            //     msg.into()
            // }
        }
    }
}

/**
 * 转化为错误响应代码
 */
impl error::ResponseError for SEVXError {
    fn status_code(&self) -> StatusCode {
        match self {
            SEVXError::DBError(_msg) | SEVXError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            SEVXError::NotFound(_msg) => StatusCode::NOT_FOUND,
            SEVXError::AuthFailed(_msg) => StatusCode::UNAUTHORIZED,
            // SEVXError::InvalidTnput(_msg) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(SEVXErrorResponse {
            error_message: self.error_response(),
        })
    }
}

/**
 * 一堆不知道的实现
 */
impl fmt::Display for SEVXError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for SEVXError {
    fn from(err: actix_web::error::Error) -> Self {
        SEVXError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for SEVXError {
    fn from(err: SQLxError) -> Self {
        SEVXError::DBError(err.to_string())
    }
}