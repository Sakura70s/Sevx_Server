use crate::handlers::tv_handler::*;
use actix_web::web;

// Tv 路由
pub fn tv_routers(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/Tv")
        // 获取 Tv 路由
        .route("/All/", web::get().to(get_all_tv))
        // 根据 id 获取
        .route("/{id}", web::get().to(get_tv_for_id))
        // 根据名称查询
        .route("/Search/{name}", web::get().to(search_tv_for_name))
        // 添加 Tv 路由
        .route("/Add/", web::post().to(add_tv))
        // 修改 Tv 路由
        .route("/Update/", web::put().to(update_tv))
        // 删除 Tv 路由
        .route("/Delete/", web::delete().to(delete_tv))
    );
}