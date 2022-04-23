use crate::{state::AppState, error::SEVXError};
use actix_web::{web, HttpResponse};
use crate::db_access::sv_db::*;
use crate::models::sv_model::*;

/**
 * 获取所有 Sv
 */
pub async fn get_all_sv(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, SEVXError> {
    get_all_sv_db(&app_state.db)
    .await
    .map(|sv| HttpResponse::Ok().json(sv))
}

/**
 * 根据 Id 获取具体 Sv
 */
pub async fn get_sv_for_id (
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
) -> Result<HttpResponse, SEVXError> {
    // 从 URL 获取 ID
    let id = i32::try_from(params.into_inner()).unwrap();
    // 查询数据库
    get_sv_for_id_db(&app_state.db, id)
    .await
    .map(|sv| HttpResponse::Ok().json(sv))
}

/**
 * Sv 名称 查询
 */
pub async fn search_sv_for_name (
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, SEVXError> {
    // 从 URL 获取 name
    let name = String::from(params.into_inner());
    //数据库查询
    search_sv_for_name_db(&app_state.db, name)
    .await
    .map(|sv| HttpResponse::Ok().json(sv))
}

/**
 * 添加 Sv
 */
pub async fn add_sv (
    new_sv: web::Json<AddSv>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SEVXError> {
    add_sv_db(&app_state.db, new_sv.try_into()?)
    .await.map(|sv| HttpResponse::Ok().json(sv))
}

/**
 * 更新 Sv
 */
pub async fn update_sv (
    app_state: web::Data<AppState>,
    new_sv: web::Json<UpdateSv>,
) -> Result<HttpResponse, SEVXError> {
    update_sv_db(&app_state.db, new_sv.into())
    .await
    .map(|sv| HttpResponse::Ok().json(sv))
}

/**
 * 删除 Sv
 */
pub async fn delete_sv (
    app_state: web::Data<AppState>,
    delete_sv: web::Json<DeleteSv>,
) -> Result<HttpResponse, SEVXError> {
    delete_sv_db(&app_state.db, delete_sv.into())
    .await
    .map(|sv| HttpResponse::Ok().body(sv))
}