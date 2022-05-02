use actix_web::web;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use crate::error::SEVXError;
use std::convert::TryFrom;

/**
 * Film 结构体
 */
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Film {
    pub id: i32,                    // <- id
    pub seriesflag: bool,           // <- 系列 Flag  
    pub seriesid: i16,              // <- 系列ID
    pub film_name: String,          // <- 名称
    pub film_year: NaiveDate,       // <- 年份
    pub director: String,           // <- 监督
    pub screenwriter: String,       // <- 导演
    pub make: String,               // <- 出品方
    pub logo: String,               // <- 剧照
    // pub amount: i16,             // <- 集数
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
 * Film 添加结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct AddFilm {
    // pub id: i32,                 // <- id
    pub seriesflag: bool,           // <- 系列 Flag  
    pub seriesid: i16,              // <- 系列ID
    pub film_name: String,          // <- 名称
    pub film_year: NaiveDate,       // <- 年份
    pub director: String,           // <- 监督
    pub screenwriter: String,       // <- 导演
    pub make: String,               // <- 出品方
    pub logo: String,               // <- 剧照
    // pub amount: i16,             // <- 集数
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
 * Film 添加 实现
 */
impl TryFrom<web::Json<AddFilm>> for AddFilm {
    type Error = SEVXError;
    fn try_from(add_film: web::Json<AddFilm>) -> Result<Self, Self::Error> {
        Ok(AddFilm {
            // id: add_Film.id,
            seriesflag: add_film.seriesflag,
            seriesid: add_film.seriesid,
            film_name: add_film.film_name.clone(),
            film_year: add_film.film_year,
            director: add_film.director.clone(),
            screenwriter: add_film.screenwriter.clone(),
            make: add_film.make.clone(),
            logo: add_film.logo.clone(),
            localflag: add_film.localflag,
            localurl: add_film.localurl.clone(),
            remoteflag: add_film.remoteflag,
            remoteurl: add_film.remoteurl.clone(),
            container: add_film.container.clone(),
            codev: add_film.codev.clone(),
            codea: add_film.codea.clone(),
            subtype: add_film.subtype.clone(),
            subteam: add_film.subteam.clone(),
            lastwatch: add_film.lastwatch,
            // update_time: NaiveDate::from(add_Film.updatetime),
            remark: add_film.remark.clone(),
        })
    }
}


/**
 * Film 更新 结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateFilm {
    pub id: i32,                            // <- id
    pub seriesflag: Option<bool>,           // <- 系列 Flag  
    pub seriesid: Option<i16>,              // <- 系列ID
    pub film_name: Option<String>,          // <- 名称
    pub film_year: Option<NaiveDate>,       // <- 年份
    pub director: Option<String>,           // <- 监督
    pub screenwriter: Option<String>,       // <- 导演
    pub make: Option<String>,               // <- 出品方
    pub logo: Option<String>,               // <- 剧照
    // pub amount: Option<i16>,             // <- 集数
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
 * 实现-Film 更新-From
 */
impl From<web::Json<UpdateFilm>> for UpdateFilm {
    fn from(film: web::Json<UpdateFilm>) -> Self {
        UpdateFilm { 
            id: film.id,
            seriesflag: film.seriesflag.clone(),
            seriesid: film.seriesid,
            film_name: film.film_name.clone(),
            film_year: film.film_year,
            director: film.director.clone(),
            screenwriter: film.screenwriter.clone(),
            make: film.make.clone(),
            logo: film.logo.clone(),
            localflag: film.localflag,
            localurl: film.localurl.clone(),
            remoteflag: film.remoteflag,
            remoteurl: film.remoteurl.clone(),
            container: film.container.clone(),
            codev: film.codev.clone(),
            codea: film.codea.clone(),
            subtype: film.subtype.clone(),
            subteam: film.subteam.clone(),
            lastwatch: film.lastwatch,
            remark: film.remark.clone(),
        }
    }
}


/**
 * Film 删除结构体
 * 仅当口令验证正确的时候才执行删除
 */
#[derive(Deserialize, Debug, Clone)]
pub struct DeleteFilm {
    pub id: i32,                // <- Film id
    pub name: String,
    pub password: String,       // <- 口令
}
/**
 * Film 删除 实现
 */
impl From<web::Json<DeleteFilm>> for DeleteFilm {
    fn from(film: web::Json<DeleteFilm>) -> Self {
        DeleteFilm {
            id: film.id,
            name: film.name.clone(),
            password: film.password.clone(),
        }
    }
}
