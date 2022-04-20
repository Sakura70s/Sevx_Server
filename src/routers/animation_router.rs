use crate::handlers::animation_handler::*;
use actix_web::web;

// Animation 路由
pub fn animation_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/Animation")
        // 获取动漫路由
        .route("/All", web::get().to(get_all_animation))
        // 添加动漫路由
        .route("/Add", web::post().to(add_animation))
    );
}