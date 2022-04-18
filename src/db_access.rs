use crate::models::*;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;
use crate::error::SEVXError;

/**
 * 已实现自定义错误类型
 * 获取一个老师下的所有课程
 * 更改为 await后加？，同时将函数返回值改为 Result
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
 * 获取一个老师下的单独课程
 */
pub async fn get_course_detail_db(pool: &PgPool, teacher_id: i32, course_id: i32) -> Course {
    let row = sqlx::query!(
        r#"
            Select id, teacher_id, name, time
            From course
            Where teacher_id = $1 and id = $2
        "#, teacher_id, course_id 
    )
        .fetch_one(pool)
        .await
        .unwrap();

    Course {
        id: Some(row.id),
        teacher_id: row.teacher_id,
        name: row.name.clone(),
        time: Some(NaiveDateTime::from(row.time))
    }

}

/**
 * 新增课程
 */
pub async fn new_course_db(pool: &PgPool, new_course: Course) -> Course {
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
        .await
        .unwrap();

        Course {
            id: Some(row.id),
            teacher_id: row.teacher_id,
            name: row.name.clone(),
            time: Some(NaiveDateTime::from(row.time))
        }
}