use crate::handlers::auth_handler::*;
use actix_web::web;

/**
 * 认证路由
 */
pub fn auth_routers(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/Auth")
        // 认证
        .route("/login/", web::post().to(get_login))
    );
}