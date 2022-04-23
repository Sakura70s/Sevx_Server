use crate::{state::AppState, error::SEVXError};
use actix_web::{web, HttpResponse};
use crate::db_access::animation_db::*;
use crate::models::animation_model::*;

/**
 * 获取所有 Animation
 */
pub async fn get_all_animation(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, SEVXError> {
    get_all_animation_db(&app_state.db)
    .await
    .map(|animation| HttpResponse::Ok().json(animation))
}

/**
 * 根据 Id 获取具体 Animation
 */
pub async fn get_animation_for_id (
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
) -> Result<HttpResponse, SEVXError> {
    // 从 URL 获取 ID
    let id = i32::try_from(params.into_inner()).unwrap();
    // 查询数据库
    get_animation_for_id_db(&app_state.db, id)
    .await
    .map(|animation| HttpResponse::Ok().json(animation))
}

/**
 * Animation 名称 查询
 */
pub async fn search_animation_for_name (
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, SEVXError> {
    // 从 URL 获取 name
    let name = String::from(params.into_inner());
    //数据库查询
    search_animation_for_name_db(&app_state.db, name)
    .await
    .map(|animation| HttpResponse::Ok().json(animation))
}

/**
 * 添加 Animation
 */
pub async fn add_animation (
    new_animation: web::Json<AddAnimation>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SEVXError> {
    add_animation_db(&app_state.db, new_animation.try_into()?)
    .await.map(|animation| HttpResponse::Ok().json(animation))
}

/**
 * 更新 Animation
 */
pub async fn update_animation (
    app_state: web::Data<AppState>,
    new_animation: web::Json<UpdateAnimation>,
) -> Result<HttpResponse, SEVXError> {
    update_animation_db(&app_state.db, new_animation.into())
    .await
    .map(|animation| HttpResponse::Ok().json(animation))
}

/**
 * 删除 Animation
 */
pub async fn delete_animation (
    app_state: web::Data<AppState>,
    delete_animation: web::Json<DeleteAnimation>,
) -> Result<HttpResponse, SEVXError> {
    delete_animation_db(&app_state.db, delete_animation.into())
    .await
    .map(|animation| HttpResponse::Ok().body(animation))
}