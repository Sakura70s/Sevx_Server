use crate::{state::AppState, error::SEVXError};
use actix_web::{web, HttpResponse};
use crate::db_access::animation_db::*;
use crate::models::animation_model::*;

/**
 * 获取所有动画
 */
pub async fn get_all_animation(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, SEVXError> {
    get_all_animation_db(&app_state.db)
    .await
    .map(|animation| HttpResponse::Ok().json(animation))
}

/**
 * 添加动画
 */
pub async fn add_animation(
    new_animation: web::Json<AddAnimation>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SEVXError> {
    add_animation_db(&app_state.db, new_animation.try_into()?)
    .await.map(|animation| HttpResponse::Ok().json(animation))
}