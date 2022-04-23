use crate::{state::AppState, error::SEVXError};
use actix_web::{web, HttpResponse};
use crate::db_access::film_db::*;
use crate::models::film_model::*;

/**
 * 获取所有 Film
 */
pub async fn get_all_film(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, SEVXError> {
    get_all_film_db(&app_state.db)
    .await
    .map(|film| HttpResponse::Ok().json(film))
}

/**
 * 根据 Id 获取具体 Film
 */
pub async fn get_film_for_id (
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
) -> Result<HttpResponse, SEVXError> {
    // 从 URL 获取 ID
    let id = i32::try_from(params.into_inner()).unwrap();
    // 查询数据库
    get_film_for_id_db(&app_state.db, id)
    .await
    .map(|film| HttpResponse::Ok().json(film))
}

/**
 * Film 名称 查询
 */
pub async fn search_film_for_name (
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, SEVXError> {
    // 从 URL 获取 name
    let name = String::from(params.into_inner());
    //数据库查询
    search_film_for_name_db(&app_state.db, name)
    .await
    .map(|film| HttpResponse::Ok().json(film))
}

/**
 * 添加 Film
 */
pub async fn add_film (
    new_film: web::Json<AddFilm>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SEVXError> {
    add_film_db(&app_state.db, new_film.try_into()?)
    .await.map(|film| HttpResponse::Ok().json(film))
}

/**
 * 更新 Film
 */
pub async fn update_film (
    app_state: web::Data<AppState>,
    new_film: web::Json<UpdateFilm>,
) -> Result<HttpResponse, SEVXError> {
    update_film_db(&app_state.db, new_film.into())
    .await
    .map(|film| HttpResponse::Ok().json(film))
}

/**
 * 删除 Film
 */
pub async fn delete_film (
    app_state: web::Data<AppState>,
    delete_film: web::Json<DeleteFilm>,
) -> Result<HttpResponse, SEVXError> {
    delete_film_db(&app_state.db, delete_film.into())
    .await
    .map(|film| HttpResponse::Ok().body(film))
}