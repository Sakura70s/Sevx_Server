use crate::handlers::sv_handler::*;
use actix_web::web;

// Sv 路由
pub fn sv_routers(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/Sv")
        // 获取 Sv 路由
        .route("/All/", web::get().to(get_all_sv))
        // 根据 id 获取
        .route("/{id}", web::get().to(get_sv_for_id))
        // 根据名称查询
        .route("/Search/{name}", web::get().to(search_sv_for_name))
        // 添加 Sv 路由
        .route("/Add/", web::post().to(add_sv))
        // 修改 Sv 路由
        .route("/Update/", web::put().to(update_sv))
        // 删除 Sv 路由
        .route("/Delete/", web::delete().to(delete_sv))
    );
}