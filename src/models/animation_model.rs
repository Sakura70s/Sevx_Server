use actix_web::web;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use crate::error::SEVXError;
use std::convert::TryFrom;

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
/**
 * Animation 结构体
 */
pub struct Animation {
    pub id: i32,                    // <- id
    pub seriesflag: bool,          // <- 系列 Flag  
    pub seriesid: i16,             // <- 系列ID
    pub animation_name: String,     // <- 名称
    pub animation_year: NaiveDate,  // <- 年份
    pub director: String,           // <- 监督
    pub screenwriter: String,      // <- 系列构成
    pub make: String,               // <- 动画制作
    pub logo: String,               // <- 剧照
    pub amount: i16,                // <- 集数
    pub localflag: bool,           // <- 本地 Flag
    pub localurl: Option<String>,  // <- 本地 URL      Null
    pub remoteflag: bool,          // <- 远程 Flag 
    pub remoteurl: Option<String>, // <- 远程 URL      Null
    pub container: String,          // <- 容器格式
    pub codev: String,              // <- 本地视频编码格式
    pub codea: String,              // <- 本地音频编码格式
    pub subtype: String,           // <- 字幕类型
    pub subteam: Option<String>,   // <- 字幕组        Null
    pub lastwatch: NaiveDate,      // <- 最后观看时间
    pub updatetime: NaiveDate,     // <- 更新时间
    pub remark: Option<String>,     // <- 备注          Null
}

/**
 * 动漫添加结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct AddAnimation {
    // pub id: i32,                 // <- id
    pub seriesflag: bool,          // <- 系列 Flag  
    pub seriesid: i16,             // <- 系列ID
    pub animation_name: String,     // <- 名称
    pub animation_year: NaiveDate,  // <- 年份
    pub director: String,           // <- 监督
    pub screenwriter: String,      // <- 系列构成
    pub make: String,               // <- 动画制作
    pub logo: String,               // <- 剧照
    pub amount: i16,                // <- 集数
    pub localflag: bool,           // <- 本地 Flag
    pub localurl: Option<String>,  // <- 本地 URL      Null
    pub remoteflag: bool,          // <- 远程 Flag 
    pub remoteurl: Option<String>, // <- 远程 URL      Null
    pub container: String,          // <- 容器格式
    pub codev: String,              // <- 本地视频编码格式
    pub codea: String,              // <- 本地音频编码格式
    pub subtype: String,           // <- 字幕类型
    pub subteam: Option<String>,   // <- 字幕组        Null
    pub lastwatch: NaiveDate,      // <- 最后观看时间
    // pub update_time: NaiveDate,  // <- 更新时间
    pub remark: Option<String>,     // <- 备注          Null
}

/**
 * 动漫添加 实现
 */
impl TryFrom<web::Json<AddAnimation>> for AddAnimation {
    type Error = SEVXError;
    fn try_from(add_animation: web::Json<AddAnimation>) -> Result<Self, Self::Error> {
        Ok(AddAnimation {
            // id: add_animation.id,
            seriesflag: add_animation.seriesflag,
            seriesid: add_animation.seriesid,
            animation_name: add_animation.animation_name.clone(),
            animation_year: add_animation.animation_year,
            director: add_animation.director.clone(),
            screenwriter: add_animation.screenwriter.clone(),
            make: add_animation.make.clone(),
            logo: add_animation.logo.clone(),
            amount: add_animation.amount,
            localflag: add_animation.localflag,
            localurl: add_animation.localurl.clone(),
            remoteflag: add_animation.remoteflag,
            remoteurl: add_animation.remoteurl.clone(),
            container: add_animation.container.clone(),
            codev: add_animation.codev.clone(),
            codea: add_animation.codea.clone(),
            subtype: add_animation.subtype.clone(),
            subteam: add_animation.subteam.clone(),
            lastwatch: add_animation.lastwatch,
            // update_time: NaiveDate::from(add_animation.updatetime),
            remark: add_animation.remark.clone(),
        })
    }
}
