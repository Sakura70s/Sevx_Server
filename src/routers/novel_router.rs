use crate::handlers::novel_handler::*;
use actix_web::web;

// Novel 路由
pub fn novel_routers(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/Novel")
        // 获取 Novel 路由
        .route("/All/", web::get().to(get_all_novel))
        // 根据 id 获取
        .route("/{id}", web::get().to(get_novel_for_id))
        // 根据名称查询
        .route("/Search/{name}", web::get().to(search_novel_for_name))
        // 添加 Novel 路由
        .route("/Add/", web::post().to(add_novel))
        // 修改 Novel 路由
        .route("/Update/", web::put().to(update_novel))
        // 删除 Novel 路由
        .route("/Delete/", web::delete().to(delete_novel))
    );
}