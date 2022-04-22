use crate::handlers::animation_handler::*;
use actix_web::web;

// Animation 路由
pub fn animation_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/Animation")
        // 获取动漫路由
        .route("/All", web::get().to(get_all_animation))
        // 根据 id 获取
        .route("/{id}", web::get().to(get_animation_for_id))
        // 根据名称查询
        .route("/Search/{name}", web::get().to(search_animation_for_name))
        // 添加动漫路由
        .route("/Add", web::post().to(add_animation))
        // 修改动漫路由
        .route("/Update/", web::put().to(update_animation))
        // 删除动漫路由
        .route("/Delete", web::delete().to(delete_animation))
    );
}