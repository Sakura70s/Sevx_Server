use crate::{state::AppState, error::SEVXError};
use actix_web::{web, HttpResponse};
use crate::db_access::novel_db::*;
use crate::models::novel_model::*;

/**
 * 获取所有 Novel
 */
pub async fn get_all_novel(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, SEVXError> {
    get_all_novel_db(&app_state.db)
    .await
    .map(|novel| HttpResponse::Ok().json(novel))
}

/**
 * 根据 Id 获取具体 Novel
 */
pub async fn get_novel_for_id (
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
) -> Result<HttpResponse, SEVXError> {
    // 从 URL 获取 ID
    let id = i32::try_from(params.into_inner()).unwrap();
    // 查询数据库
    get_novel_for_id_db(&app_state.db, id)
    .await
    .map(|novel| HttpResponse::Ok().json(novel))
}

/**
 * Novel 名称 查询
 */
pub async fn search_novel_for_name (
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, SEVXError> {
    // 从 URL 获取 name
    let name = String::from(params.into_inner());
    //数据库查询
    search_novel_for_name_db(&app_state.db, name)
    .await
    .map(|novel| HttpResponse::Ok().json(novel))
}

/**
 * 添加 Novel
 */
pub async fn add_novel (
    new_novel: web::Json<AddNovel>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SEVXError> {
    add_novel_db(&app_state.db, new_novel.try_into()?)
    .await.map(|novel| HttpResponse::Ok().json(novel))
}

/**
 * 更新 Novel
 */
pub async fn update_novel (
    app_state: web::Data<AppState>,
    new_novel: web::Json<UpdateNovel>,
) -> Result<HttpResponse, SEVXError> {
    update_novel_db(&app_state.db, new_novel.into())
    .await
    .map(|novel| HttpResponse::Ok().json(novel))
}

/**
 * 删除 Novel
 */
pub async fn delete_novel (
    app_state: web::Data<AppState>,
    delete_novel: web::Json<DeleteNovel>,
) -> Result<HttpResponse, SEVXError> {
    delete_novel_db(&app_state.db, delete_novel.into())
    .await
    .map(|novel| HttpResponse::Ok().body(novel))
}