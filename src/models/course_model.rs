use actix_web::web;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::error::SEVXError;
use std::convert::TryFrom;

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
/**
 * 课程获取结构体
 */
pub struct Course {
    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: NaiveDate,
    pub lang: Option<String>,
}
/**
 * 课程添加结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct CourseAdd {
    pub teacher_id: i32,
    pub name: String,
    pub lang: Option<String>,
}

/**
 * 课程更新结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct CourseUpdate {
    pub name: Option<String>,
    pub lang: Option<String>,
}

// /**
//  * 课程添加实现1
//  */
// impl From<web::Json<Course_add>> for Course_add {
//     fn from(course: web::Json<Course_add>) -> Self {
//         Course_add {
//             teacher_id: course.teacher_id,
//             name: course.name.clone(),
//             lang: course.lang.clone(),
//         }
//     }
// }

/**
 * 课程添加实现2
 */
impl TryFrom<web::Json<CourseAdd>> for CourseAdd {
    type Error = SEVXError;

    fn try_from(course: web::Json<CourseAdd>)
        -> Result<Self, Self::Error> {
            Ok(CourseAdd {
                teacher_id: course.teacher_id,
                name: course.name.clone(),
                lang: course.lang.clone(),
            })
        }
}

/**
 * 课程更新实现
 */
impl From<web::Json<CourseUpdate>> for CourseUpdate {
    fn from(course: web::Json<CourseUpdate>) -> Self {
        CourseUpdate {
            name: course.name.clone(),
            lang: course.lang.clone(),
        }
    }
}