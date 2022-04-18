use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
// 定义课程
pub struct Course {
    pub teacher_id: i32,
    pub id: Option<i32>,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}
// 课程的实现
impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            teacher_id: course.teacher_id,
            id: course.id,
            name: course.name.clone(),
            time: course.time,
        }
    }
}
