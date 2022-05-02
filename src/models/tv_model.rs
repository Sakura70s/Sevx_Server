use actix_web::web;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use crate::error::SEVXError;
use std::convert::TryFrom;

/**
 * Tv 结构体
 */
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Tv {
    pub id: i32,                    // <- id
    pub seriesflag: bool,           // <- 系列 Flag  
    pub seriesid: i16,              // <- 系列ID
    pub tv_name: String,            // <- 名称
    pub tv_year: NaiveDate,         // <- 年份
    pub director: String,           // <- 导演
    pub screenwriter: String,       // <- 编剧
    pub make: String,               // <- 出品方
    pub logo: String,               // <- 剧照
    pub amount: i16,                // <- 集数
    pub localflag: bool,            // <- 本地 Flag
    pub localurl: Option<String>,   // <- 本地 URL      Null
    pub remoteflag: bool,           // <- 远程 Flag 
    pub remoteurl: Option<String>,  // <- 远程 URL      Null
    pub container: String,          // <- 容器格式
    pub codev: String,              // <- 本地视频编码格式
    pub codea: String,              // <- 本地音频编码格式
    pub subtype: String,            // <- 字幕类型
    pub subteam: Option<String>,    // <- 字幕组        Null
    pub lastwatch: NaiveDate,       // <- 最后观看时间
    pub updatetime: NaiveDate,      // <- 更新时间
    pub remark: Option<String>,     // <- 备注          Null
}


/**
 * Tv 添加结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct AddTv {
    // pub id: i32,                 // <- id
    pub seriesflag: bool,           // <- 系列 Flag  
    pub seriesid: i16,              // <- 系列ID
    pub tv_name: String,            // <- 名称
    pub tv_year: NaiveDate,         // <- 年份
    pub director: String,           // <- 监督
    pub screenwriter: String,       // <- 系列构成
    pub make: String,               // <- 动画制作
    pub logo: String,               // <- 剧照
    pub amount: i16,                // <- 集数
    pub localflag: bool,            // <- 本地 Flag
    pub localurl: Option<String>,   // <- 本地 URL      Null
    pub remoteflag: bool,           // <- 远程 Flag 
    pub remoteurl: Option<String>,  // <- 远程 URL      Null
    pub container: String,          // <- 容器格式
    pub codev: String,              // <- 本地视频编码格式
    pub codea: String,              // <- 本地音频编码格式
    pub subtype: String,            // <- 字幕类型
    pub subteam: Option<String>,    // <- 字幕组        Null
    pub lastwatch: NaiveDate,       // <- 最后观看时间
    // pub update_time: NaiveDate,  // <- 更新时间
    pub remark: Option<String>,     // <- 备注          Null
}
/**
 * Tv 添加 实现
 */
impl TryFrom<web::Json<AddTv>> for AddTv {
    type Error = SEVXError;
    fn try_from(add_tv: web::Json<AddTv>) -> Result<Self, Self::Error> {
        Ok(AddTv {
            // id: add_tv.id,
            seriesflag: add_tv.seriesflag,
            seriesid: add_tv.seriesid,
            tv_name: add_tv.tv_name.clone(),
            tv_year: add_tv.tv_year,
            director: add_tv.director.clone(),
            screenwriter: add_tv.screenwriter.clone(),
            make: add_tv.make.clone(),
            logo: add_tv.logo.clone(),
            amount: add_tv.amount,
            localflag: add_tv.localflag,
            localurl: add_tv.localurl.clone(),
            remoteflag: add_tv.remoteflag,
            remoteurl: add_tv.remoteurl.clone(),
            container: add_tv.container.clone(),
            codev: add_tv.codev.clone(),
            codea: add_tv.codea.clone(),
            subtype: add_tv.subtype.clone(),
            subteam: add_tv.subteam.clone(),
            lastwatch: add_tv.lastwatch,
            // update_time: NaiveDate::from(add_tv.updatetime),
            remark: add_tv.remark.clone(),
        })
    }
}


/**
 * Tv 更新 结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTv {
    pub id: i32,                            // <- id
    pub seriesflag: Option<bool>,           // <- 系列 Flag  
    pub seriesid: Option<i16>,              // <- 系列ID
    pub tv_name: Option<String>,            // <- 名称
    pub tv_year: Option<NaiveDate>,         // <- 年份
    pub director: Option<String>,           // <- 导演
    pub screenwriter: Option<String>,       // <- 编剧
    pub make: Option<String>,               // <- 出品方
    pub logo: Option<String>,               // <- 剧照
    pub amount: Option<i16>,                // <- 集数
    pub localflag: Option<bool>,            // <- 本地 Flag
    pub localurl: Option<String>,           // <- 本地 URL      Null
    pub remoteflag: Option<bool>,           // <- 远程 Flag 
    pub remoteurl: Option<String>,          // <- 远程 URL      Null
    pub container: Option<String>,          // <- 容器格式
    pub codev: Option<String>,              // <- 本地视频编码格式
    pub codea: Option<String>,              // <- 本地音频编码格式
    pub subtype: Option<String>,            // <- 字幕类型
    pub subteam: Option<String>,            // <- 字幕组        Null
    pub lastwatch: Option<NaiveDate>,       // <- 最后观看时间
    // pub update_time: NaiveDate,          // <- 更新时间
    pub remark: Option<String>,             // <- 备注          Null
}
/**
 * 实现-Tv 更新-From
 */
impl From<web::Json<UpdateTv>> for UpdateTv {
    fn from(tv: web::Json<UpdateTv>) -> Self {
        UpdateTv { 
            id: tv.id,
            seriesflag: tv.seriesflag.clone(),
            seriesid: tv.seriesid,
            tv_name: tv.tv_name.clone(),
            tv_year: tv.tv_year,
            director: tv.director.clone(),
            screenwriter: tv.screenwriter.clone(),
            make: tv.make.clone(),
            logo: tv.logo.clone(),
            amount: tv.amount,
            localflag: tv.localflag,
            localurl: tv.localurl.clone(),
            remoteflag: tv.remoteflag,
            remoteurl: tv.remoteurl.clone(),
            container: tv.container.clone(),
            codev: tv.codev.clone(),
            codea: tv.codea.clone(),
            subtype: tv.subtype.clone(),
            subteam: tv.subteam.clone(),
            lastwatch: tv.lastwatch,
            remark: tv.remark.clone(),
        }
    }
}


/**
 * Tv 删除结构体
 * 仅当口令验证正确的时候才执行删除
 */
#[derive(Deserialize, Debug, Clone)]
pub struct DeleteTv {
    pub id: i32,                // <- Tv id
    pub name: String,
    pub password: String,       // <- 口令
}
/**
 * Tv 删除 实现
 */
impl From<web::Json<DeleteTv>> for DeleteTv {
    fn from(tv: web::Json<DeleteTv>) -> Self {
        DeleteTv {
            id: tv.id,
            name: tv.name.clone(),
            password: tv.password.clone(),
        }
    }
}
