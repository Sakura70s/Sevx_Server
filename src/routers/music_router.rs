use crate::handlers::music_handler::*;
use actix_web::web;

// Music 路由
pub fn music_routers(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/Music")
        // 获取 Music 路由
        .route("/All/", web::get().to(get_all_music))
        // 根据 id 获取
        .route("/{id}", web::get().to(get_music_for_id))
        // 根据名称查询
        .route("/Search/{name}", web::get().to(search_music_for_name))
        // 添加 Music 路由
        .route("/Add/", web::post().to(add_music))
        // 修改 Music 路由
        .route("/Update/", web::put().to(update_music))
        // 删除 Music 路由
        .route("/Delete/", web::delete().to(delete_music))
    );
}