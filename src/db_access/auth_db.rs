use crate::models::auth_model::*;
use sqlx::postgres::PgPool;
use crate::error::SEVXError;
use crate::log::print_log;
/**
 * 查询记录
 */
pub async fn get_auth_db(
    pool: &PgPool,
    uname: String,
    upassword: String,
) -> Result<String, SEVXError> {
    let row = sqlx::query_as!(
        User,
        "Select * from Auth Where uname = $1 And upassword = $2",
        uname, upassword
    )
    .fetch_optional(pool)
    .await?;
    match row {
        // 存在
        Some(_row) => Ok(format!("Auth OK")),
        // 不存在返回认证错误
        _ => Err(SEVXError::AuthFailed("Auth Failed!".into()))
    }
}
pub async fn get_login_db(
    pool: &PgPool,
    auth: Auth,
) -> Result<String, SEVXError> {
    let row = sqlx::query_as!(
        User,
        "Select * from Auth Where uname = $1 And upassword = $2",
        auth.uname, auth.upassword
    )
    .fetch_optional(pool)
    .await?;
    match row {
        // 存在
        Some(_row) => {
            print_log(format!("Auth Ok of name:[{}]", auth.uname));
            Ok(format!("Auth OK"))
        },
        // 不存在返回认证错误
        _ => Err(SEVXError::AuthFailed(format!("Auth Failed of name:[{}]", auth.uname)))
    }
}