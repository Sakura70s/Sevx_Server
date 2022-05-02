use crate::{state::AppState, error::SEVXError};
use actix_web::{web, HttpResponse};
use crate::db_access::auth_db::*;
use crate::models::auth_model::*;

/**
 * 认证
 */
pub async fn get_login(
    app_state: web::Data<AppState>,
    auth: web::Json<Auth>,
) -> Result<HttpResponse, SEVXError> {
    get_login_db(&app_state.db, auth.try_into()?)
    .await
    .map(|message| HttpResponse::Ok().body(message))
}