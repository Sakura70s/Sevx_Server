use crate::handlers::film_handler::*;
use actix_web::web;

// Film 路由
pub fn film_routers(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/Film")
        // 获取 Film 路由
        .route("/All/", web::get().to(get_all_film))
        // 根据 id 获取
        .route("/{id}", web::get().to(get_film_for_id))
        // 根据名称查询
        .route("/Search/{name}", web::get().to(search_film_for_name))
        // 添加 Film 路由
        .route("/Add/", web::post().to(add_film))
        // 修改 Film 路由
        .route("/Update/", web::put().to(update_film))
        // 删除 Film 路由
        .route("/Delete/", web::delete().to(delete_film))
    );
}