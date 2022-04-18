use crate::models::course_model::Course;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;
use crate::error::SEVXError;

/**
 * 已实现错误处理
 * 获取一个老师下的所有课程！！！
 * 成功返回 Vec，出错返回 SEVXError
 */
pub async fn get_course_for_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<Vec<Course>, SEVXError> {
    let rows = sqlx::query!(
        r#"
            Select id, teacher_id, name, time
            From course
            Where teacher_id = $1
        "#, teacher_id
    )
        .fetch_all(pool)
        .await?;

    /*
     * 通过集合长度判断是否出错
     * 为 0 则是没有查到数据
     * 其他情况则正常
     */
    let courses: Vec<Course> = rows.iter()
        .map(|item| Course {
            id: Some(item.id),
            teacher_id: item.teacher_id,
            name: item.name.clone(),
            time: Some(NaiveDateTime::from(item.time)),
        })
        .collect();
    
    match courses.len() {
        0 => Err(SEVXError::NotFound("Course not found for teacher".into())),
        _ => Ok(courses),
    }
}

/**
 * 已实现错误处理
 * 获取一个老师下的单独课程
 * match 到 Ok 返回 Course，否则返回 SEVXError
 */
pub async fn get_course_detail_db(pool: &PgPool, teacher_id: i32, course_id: i32) -> Result<Course, SEVXError> {
    let row = sqlx::query!(
        r#"
            Select id, teacher_id, name, time
            From course
            Where teacher_id = $1 and id = $2
        "#, teacher_id, course_id 
    )
        .fetch_one(pool)
        .await;

    match row {
        Ok(row) => Ok(
            Course {
                id: Some(row.id),
                teacher_id: row.teacher_id,
                name: row.name.clone(),
                time: Some(NaiveDateTime::from(row.time)),
            }
        ),

        Err(_row) => Err(SEVXError::NotFound("Course Id not found".to_string()))
    }

}

/**
 * 已实现错误处理
 * 新增课程
 * match 到 Ok 返回 Course，否则返回 SEVXError
 */
pub async fn new_course_db(pool: &PgPool, new_course: Course) -> Result<Course, SEVXError> {
    let row = sqlx::query!(
        r#"
            Insert into course (
                teacher_id,
                name
            )
            Values (
                $1,
                $2
            )
            Returning id, teacher_id, name, time
        "#, new_course.teacher_id, new_course.name
    )
        .fetch_one(pool)
        .await?;

    Ok(
        Course {
            id: Some(row.id),
            teacher_id: row.teacher_id,
            name: row.name.clone(),
            time: Some(NaiveDateTime::from(row.time))
        }
    )
}