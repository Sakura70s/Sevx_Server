use crate::handlers::comic_handler::*;
use actix_web::web;

// Comic 路由
pub fn comic_routers(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/Comic")
        // 获取 Comic 路由
        .route("/All/", web::get().to(get_all_comic))
        // 根据 id 获取
        .route("/{id}", web::get().to(get_comic_for_id))
        // 根据名称查询
        .route("/Search/{name}", web::get().to(search_comic_for_name))
        // 添加 Comic 路由
        .route("/Add/", web::post().to(add_comic))
        // 修改 Comic 路由
        .route("/Update/", web::put().to(update_comic))
        // 删除 Comic 路由
        .route("/Delete/", web::delete().to(delete_comic))
    );
}