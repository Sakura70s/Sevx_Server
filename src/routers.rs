use crate::handlers::*;
use actix_web::web;

// 创建路由，处理 /health的请求
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_checker_handler));
}

// 添加课程路由
pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::scope("/course")
        // 添加课程
        .route("/", web::post().to(new_course))
        // 根据 user id 获取所有课程
        .route("/{user_id}", web::get().to(get_course_for_teacher))
        // 根据 course id 获取详细课程
        .route("/{user_id}/{course_id}", web::get().to(get_course_detail))
    );
}