use actix_web::web;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use crate::error::SEVXError;
use std::convert::TryFrom;

/**
 * Comic 结构体
 */
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Comic {
    pub id: i32,                    // <- id
    pub seriesflag: bool,
    pub seriesid: i16,
    pub comic_name: String,            // <- 名称
    pub comic_year: NaiveDate,         // <- 年份
    pub comic_status: String,            // <- 短片类型
    pub logo: String,               // <- 剧照
    pub author: String,             // <- 作者
    pub localflag: bool,            // <- 本地 Flag
    pub localurl: Option<String>,   // <- 本地 URL      Null
    pub remoteflag: bool,           // <- 远程 Flag 
    pub remoteurl: Option<String>,  // <- 远程 URL      Null
    pub container: String,          // <- 容器格式
    pub updatetime: NaiveDate,      // <- 更新时间
    pub remark: Option<String>,     // <- 备注          Null
}


/**
 * Comic 添加结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct AddComic {
    // pub id: i32,                    // <- id
    pub seriesflag: bool,
    pub seriesid: i16,
    pub comic_name: String,            // <- 名称
    pub comic_year: NaiveDate,         // <- 年份
    pub comic_status: String,            // <- 短片类型
    pub logo: String,               // <- 剧照
    pub author: String,             // <- 作者
    pub localflag: bool,            // <- 本地 Flag
    pub localurl: Option<String>,   // <- 本地 URL      Null
    pub remoteflag: bool,           // <- 远程 Flag 
    pub remoteurl: Option<String>,  // <- 远程 URL      Null
    pub container: String,          // <- 容器格式
    // pub updatetime: NaiveDate,      // <- 更新时间
    pub remark: Option<String>,     // <- 备注          Null
}
/**
 * Comic 添加 实现
 */
impl TryFrom<web::Json<AddComic>> for AddComic {
    type  Error = SEVXError;
    fn try_from(add_comic: web::Json<AddComic>) -> Result<Self, Self::Error> {
        Ok(AddComic {
            seriesflag: add_comic.seriesflag,
            seriesid: add_comic.seriesid,
            comic_name: add_comic.comic_name.clone(),
            comic_year: add_comic.comic_year,
            comic_status: add_comic.comic_status.clone(),
            logo: add_comic.logo.clone(),
            author: add_comic.author.clone(),
            localflag: add_comic.localflag,
            localurl: add_comic.localurl.clone(),
            remoteflag: add_comic.remoteflag,
            remoteurl: add_comic.remoteurl.clone(),
            container: add_comic.container.clone(),
            remark: add_comic.remark.clone(),
        })
    }
}


/**
 * Comic 更新 结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateComic {
    pub id: i32,                            // <- id
    pub seriesflag: Option<bool>,
    pub seriesid: Option<i16>,
    pub comic_name: Option<String>,            // <- 名称
    pub comic_year: Option<NaiveDate>,         // <- 年份
    pub comic_status: Option<String>,            // <- 类型
    pub logo: Option<String>,               // <- 剧照
    pub author: Option<String>,             // <- 作者
    pub localflag: Option<bool>,            // <- 本地 Flag
    pub localurl: Option<String>,           // <- 本地 URL      Null
    pub remoteflag: Option<bool>,           // <- 远程 Flag 
    pub remoteurl: Option<String>,          // <- 远程 URL      Null
    pub container: Option<String>,          // <- 容器格式
    pub remark: Option<String>,             // <- 备注          Null
}
/**
 * 实现-Comic 更新-From
 */
impl From<web::Json<UpdateComic>> for UpdateComic {
    fn from(comic: web::Json<UpdateComic>) -> Self {
        UpdateComic { 
            id: comic.id,
            seriesflag: comic.seriesflag,
            seriesid: comic.seriesid,
            comic_name: comic.comic_name.clone(),
            comic_year: comic.comic_year,
            comic_status: comic.comic_status.clone(),
            logo: comic.logo.clone(),
            author: comic.author.clone(),
            localflag: comic.localflag,
            localurl: comic.localurl.clone(),
            remoteflag: comic.remoteflag,
            remoteurl: comic.remoteurl.clone(),
            container: comic.container.clone(),
            remark: comic.remark.clone(),
        }
    }
}


/**
 * Comic 删除结构体
 * 仅当口令验证正确的时候才执行删除
 */
#[derive(Deserialize, Debug, Clone)]
pub struct DeleteComic {
    pub id: i32,                // <- Comic id
    pub name: String,
    pub password: String,       // <- 口令
}
/**
 * Comic 删除 实现
 */
impl From<web::Json<DeleteComic>> for DeleteComic {
    fn from(comic: web::Json<DeleteComic>) -> Self {
        DeleteComic {
            id: comic.id,
            name: comic.name.clone(),
            password: comic.password.clone(),
        }
    }
}
