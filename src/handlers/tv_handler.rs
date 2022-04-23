use crate::{state::AppState, error::SEVXError};
use actix_web::{web, HttpResponse};
use crate::db_access::tv_db::*;
use crate::models::tv_model::*;

/**
 * 获取所有 Tv
 */
pub async fn get_all_tv(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, SEVXError> {
    get_all_tv_db(&app_state.db)
    .await
    .map(|tv| HttpResponse::Ok().json(tv))
}

/**
 * 根据 Id 获取具体 Tv
 */
pub async fn get_tv_for_id (
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
) -> Result<HttpResponse, SEVXError> {
    // 从 URL 获取 ID
    let id = i32::try_from(params.into_inner()).unwrap();
    // 查询数据库
    get_tv_for_id_db(&app_state.db, id)
    .await
    .map(|tv| HttpResponse::Ok().json(tv))
}

/**
 * Tv 名称 查询
 */
pub async fn search_tv_for_name (
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, SEVXError> {
    // 从 URL 获取 name
    let name = String::from(params.into_inner());
    //数据库查询
    search_tv_for_name_db(&app_state.db, name)
    .await
    .map(|tv| HttpResponse::Ok().json(tv))
}

/**
 * 添加 Tv
 */
pub async fn add_tv (
    new_tv: web::Json<AddTv>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SEVXError> {
    add_tv_db(&app_state.db, new_tv.try_into()?)
    .await.map(|tv| HttpResponse::Ok().json(tv))
}

/**
 * 更新 Tv
 */
pub async fn update_tv (
    app_state: web::Data<AppState>,
    new_tv: web::Json<UpdateTv>,
) -> Result<HttpResponse, SEVXError> {
    update_tv_db(&app_state.db, new_tv.into())
    .await
    .map(|tv| HttpResponse::Ok().json(tv))
}

/**
 * 删除 Tv
 */
pub async fn delete_tv (
    app_state: web::Data<AppState>,
    delete_tv: web::Json<DeleteTv>,
) -> Result<HttpResponse, SEVXError> {
    delete_tv_db(&app_state.db, delete_tv.into())
    .await
    .map(|tv| HttpResponse::Ok().body(tv))
}