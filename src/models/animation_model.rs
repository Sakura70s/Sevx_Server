// use actix_web::web;
use chrono::NaiveDate;
use serde::{Serialize};
// use crate::error::SEVXError;

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
/**
 * Animation 结构体
 */
pub struct Animation {
    pub id: i32,                    // <- id
    pub series_flag: bool,          // <- 系列 Flag  
    pub series_id: i16,     // <- 系列ID        Null
    pub animation_name: String,     // <- 名称
    pub animation_year: NaiveDate,  // <- 年份
    pub director: String,           // <- 监督
    pub screen_writer: String,      // <- 系列构成
    pub make: String,               // <- 动画制作
    pub logo: String,               // <- 剧照
    pub amount: i16,                // <- 集数
    pub local_flag: bool,           // <- 本地 Flag
    pub local_url: Option<String>,  // <- 本地 URL      Null
    pub remote_flag: bool,          // <- 远程 Flag 
    pub remote_url: Option<String>, // <- 远程 URL      Null
    pub container: String,          // <- 容器格式
    pub codev: String,              // <- 本地视频编码格式
    pub codea: String,              // <- 本地音频编码格式
    pub sub_type: String,           // <- 字幕类型
    pub sub_team: Option<String>,   // <- 字幕组        Null
    pub last_watch: NaiveDate,  // <- 最后观看时间  Null
    pub update_time: NaiveDate,     // <- 更新时间
    pub remark: Option<String>,     // <- 备注          Null
}