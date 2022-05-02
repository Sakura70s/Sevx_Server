use actix_web::web;
use serde::{Deserialize, Serialize};
use crate::error::SEVXError;
use std::convert::TryFrom;

/**
 * 用户结构体
 */
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i32,                // <- ID
    pub uname: String,          // <- 用户
    pub upassword: String,      // <- 密码
}


/**
 * 认证结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct Auth {
    pub uname: String,          // <- 用户
    pub upassword: String,      // <- 密码
}
// 实现
impl TryFrom<web::Json<Auth>> for Auth {
    type Error = SEVXError;
    fn try_from(auth: web::Json<Auth>) -> Result<Self, Self::Error> {
        Ok(Auth {
            uname: auth.uname.clone(),
            upassword: auth.upassword.clone(),
        })
    }
}