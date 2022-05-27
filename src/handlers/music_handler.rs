use crate::{state::AppState, error::SEVXError};
use actix_web::{web, HttpResponse};
use crate::db_access::music_db::*;
use crate::models::music_model::*;
use crate::models::auth_model::*;

/**
 * 获取所有 Music
 */
pub async fn get_all_music(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, SEVXError> {
    get_all_music_db(&app_state.db)
    .await
    .map(|music| HttpResponse::Ok().json(music))
}

/**
 * 根据 Id 获取具体 Music
 */
pub async fn get_music_for_id (
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
) -> Result<HttpResponse, SEVXError> {
    // 从 URL 获取 ID
    let id = i32::try_from(params.into_inner()).unwrap();
    // 查询数据库
    get_music_for_id_db(&app_state.db, id)
    .await
    .map(|music| HttpResponse::Ok().json(music))
}

/**
 * Music 名称 查询
 */
pub async fn search_music_for_name (
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, SEVXError> {
    // 从 URL 获取 name
    let name = String::from(params.into_inner());
    //数据库查询
    search_music_for_name_db(&app_state.db, name)
    .await
    .map(|music| HttpResponse::Ok().json(music))
}

/**
 * 添加 Music
 */
pub async fn add_music (
    new_music: web::Json<AddMusic>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SEVXError> {
    let auth = Auth{
        uname: new_music.uname.clone(),
        upassword: new_music.upassword.clone(),
    };
    add_music_db(&app_state.db, new_music.try_into()?, auth)
    .await.map(|music| HttpResponse::Ok().json(music))
}

/**
 * 更新 Music
 */
pub async fn update_music (
    app_state: web::Data<AppState>,
    new_music: web::Json<UpdateMusic>,
) -> Result<HttpResponse, SEVXError> {
    let auth = Auth{
        uname: new_music.uname.clone(),
        upassword: new_music.upassword.clone(),
    };
    update_music_db(&app_state.db, new_music.into(), auth)
    .await
    .map(|music| HttpResponse::Ok().json(music))
}

/**
 * 删除 Music
 */
pub async fn delete_music (
    app_state: web::Data<AppState>,
    delete_music: web::Json<DeleteMusic>,
) -> Result<HttpResponse, SEVXError> {
    delete_music_db(&app_state.db, delete_music.into())
    .await
    .map(|music| HttpResponse::Ok().body(music))
}