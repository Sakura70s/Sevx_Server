use crate::models::course_model::*;
use sqlx::postgres::PgPool;
use crate::error::SEVXError;
use crate::log::print_log;

/**
 * 获取所有课程
 * 成功返回 Vec，出错返回 SEVXError
 */
pub async fn get_course_for_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<Vec<Course>, SEVXError> {
    
    // 查询当前老师
    let rows = sqlx::query_as!(
        Course,
        r#"
            Select * From course Where teacher_id = $1
        "#, teacher_id
    )
        .fetch_optional(pool)
        .await?;

    // 如果存在则返回集合
    match rows {
        Some(_rows) => {
            let courses: Vec<Course> = sqlx::query_as!(
                Course,
                r#"
                    Select * From course Where teacher_id = $1
                "#, teacher_id
            )
            .fetch_all(pool)
            .await?;

            print_log(format!("Get course of teacher:{}", teacher_id));
            Ok(courses)
        },
        _ => Err(SEVXError::NotFound(format!("Teacher:{} not found", teacher_id))),
    }
}

/**
 * 获取单独课程
 * 成功返回 Course，失败返回 SEVXError
 */
pub async fn get_course_detail_db(pool: &PgPool, teacher_id: i32, course_id: i32) -> Result<Course, SEVXError> {
    let row = sqlx::query_as!(
        Course,
        r#"
            Select *
            From course
            Where teacher_id = $1 and id = $2
        "#, teacher_id, course_id 
    )
        .fetch_optional(pool)
        .await?;

    match row {
        Some(row) => {
            // Pring Log
            print_log(format!("Get course detail when teacher:{} & course:{}", teacher_id, course_id));
            Ok(row)
        },

        _ => Err(SEVXError::NotFound(format!("The course:{} of the teacher:{} not found",course_id, teacher_id)))
    }

}

/**
 * 新增课程
 * Ok 返回 Course，否则返回 SEVXError
 */
pub async fn post_new_course_db(pool: &PgPool, new_course: CourseAdd) -> Result<Course, SEVXError> {
    let row = sqlx::query_as!(
        Course,
        r#"
            Insert into course (
                teacher_id,
                name,
                lang
            )
            Values (
                $1,
                $2,
                $3
            )
            Returning id, teacher_id, name, time, lang
        "#, new_course.teacher_id, new_course.name, new_course.lang
    )
        .fetch_one(pool)
        .await?;
    print_log(format!("Add course:{}", row.id));
    Ok(row)
}

/**
 * 删除课程
 * 成功返回字符串，失败返回 SEVXError
 */
pub async fn delete_course_db(pool: &PgPool, teacher_id: i32, id: i32) -> Result<String, SEVXError> {

    // 判断要删除课程是否存在, 存在继续，不存在返回错误信息
    let current_row = sqlx::query_as!(
        Course,
        "Select * From course where teacher_id = $1 and id = $2",
        teacher_id, id,
    ).fetch_optional(pool).await?;

    match current_row {
        Some(_current_row) => {
            let course_row = sqlx::query!(
                "Delete From course where teacher_id = $1 and id = $2",
                teacher_id, id,
            )
            .execute(pool)
            .await?;
            print_log(format!("Delete course:{}", id));
            Ok(format!("Delete {:?} record", course_row))
        },

        _ => Err(SEVXError::NotFound(format!("Course:{} of teacher:{} not found", id, teacher_id))),
    }
}

/**
 * 更新课程
 */
pub async fn update_course_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
    update_course: CourseUpdate,
) -> Result<Course, SEVXError> {

    // 获取当前课程
    let current_course_row = sqlx::query_as!(
        Course,
        "Select * from course where teacher_id = $1 and id = $2",
        teacher_id, id,
    )
    .fetch_one(pool)
    .await
    // 成功则继续，失败则返回
    .map_err(|_err| SEVXError::NotFound("Course not found".into()))?;

    let name: String = match update_course.name {
        Some(name) => name,
        _ => current_course_row.name,
    };

    let lang: String = match update_course.lang {
        Some(lang) => lang,
        _ => current_course_row.lang.unwrap_or_default(),
    };

    // 更改课程
    let course_row = sqlx::query_as!(
        Course,
        "Update course Set name = $1, lang = $2 where teacher_id = $3 and id = $4
        Returning id, teacher_id, name, time, lang",
        name, lang, teacher_id, id,
    )
    .fetch_one(pool)
    .await;

    match course_row {
        Ok(course_row) => {
            print_log(format!("Update course:{} of teacher:{}", id, teacher_id));
            Ok(course_row)
        },
        Err(_course_row) => Err(SEVXError::NotFound("Course not found".into())),
    }
}