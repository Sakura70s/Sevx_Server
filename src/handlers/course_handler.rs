use crate::{state::AppState, error::SEVXError,};
use actix_web::{web, HttpResponse};
use crate::db_access::course_db::{new_course_db, get_course_for_teacher_db, get_course_detail_db};
use crate::models::course_model::*;

/**
 * 添加课程处理器
 */

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SEVXError> {
    new_course_db(&app_state.db, new_course.into())
        .await.map(|course| HttpResponse::Ok().json(course))
}

/**
 * 已有错误处理
 * 获取指定老师课程
 * 将返回类型更改为了 Result
 */
pub async fn get_course_for_teacher(
    params: web::Path<usize>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SEVXError> {
    // 仅有一个参数用 into_inner() 方法
    let teacher_id = i32::try_from(params.into_inner()).unwrap();

    // 查询数据库
    get_course_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

/**
 * 已有错误处理
 * 获取指定老师指定课程
 */
pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> Result<HttpResponse, SEVXError> {
    // 多个参数正常使用
    let teacher_id = i32::try_from(params.0).unwrap();
    let course_id = i32::try_from(params.1).unwrap();

    // 查询数据库
    get_course_detail_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
    
}