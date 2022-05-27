use crate::{state::AppState, error::SEVXError};
use actix_web::{web, HttpResponse};
use crate::db_access::comic_db::*;
use crate::models::comic_model::*;
use crate::models::auth_model::*;

/**
 * 获取所有 Comic
 */
pub async fn get_all_comic(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, SEVXError> {
    get_all_comic_db(&app_state.db)
    .await
    .map(|comic| HttpResponse::Ok().json(comic))
}

/**
 * 根据 Id 获取具体 Comic
 */
pub async fn get_comic_for_id (
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
) -> Result<HttpResponse, SEVXError> {
    // 从 URL 获取 ID
    let id = i32::try_from(params.into_inner()).unwrap();
    // 查询数据库
    get_comic_for_id_db(&app_state.db, id)
    .await
    .map(|comic| HttpResponse::Ok().json(comic))
}

/**
 * Comic 名称 查询
 */
pub async fn search_comic_for_name (
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, SEVXError> {
    // 从 URL 获取 name
    let name = String::from(params.into_inner());
    //数据库查询
    search_comic_for_name_db(&app_state.db, name)
    .await
    .map(|comic| HttpResponse::Ok().json(comic))
}

/**
 * 添加 Comic
 */
pub async fn add_comic (
    new_comic: web::Json<AddComic>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SEVXError> {
    let auth = Auth{
        uname: new_comic.uname.clone(),
        upassword: new_comic.upassword.clone(),
    };
    add_comic_db(&app_state.db, new_comic.try_into()?, auth)
    .await.map(|comic| HttpResponse::Ok().json(comic))
}

/**
 * 更新 Comic
 */
pub async fn update_comic (
    app_state: web::Data<AppState>,
    new_comic: web::Json<UpdateComic>,
) -> Result<HttpResponse, SEVXError> {
    let auth = Auth{
        uname: new_comic.uname.clone(),
        upassword: new_comic.upassword.clone(),
    };
    update_comic_db(&app_state.db, new_comic.into(), auth)
    .await
    .map(|comic| HttpResponse::Ok().json(comic))
}

/**
 * 删除 Comic
 */
pub async fn delete_comic (
    app_state: web::Data<AppState>,
    delete_comic: web::Json<DeleteComic>,
) -> Result<HttpResponse, SEVXError> {
    delete_comic_db(&app_state.db, delete_comic.into())
    .await
    .map(|comic| HttpResponse::Ok().body(comic))
}