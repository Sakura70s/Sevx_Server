use crate::handlers::course_handler::*;
use actix_web::web;

// 课程路由
pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/course")
        // 添加课程
        .route("/add", web::post().to(post_new_course))
        // 获取所有课程
        .route("/{teacher_id}", web::get().to(get_course_for_teacher))
        // 获取单独课程
        .route("/{teacher_id}/{course_id}", web::get().to(get_course_detail))
        // 删除课程
        .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
        // 修改课程
        .route("/{teacher_id}/{course_id}", web::put().to(update_course))
    );
}