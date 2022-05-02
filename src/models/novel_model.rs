use actix_web::web;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use crate::error::SEVXError;
use std::convert::TryFrom;

/**
 * Novel 结构体
 */
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Novel {
    pub id: i32,                    // <- id
    pub seriesflag: bool,
    pub seriesid: i16,
    pub novel_name: String,            // <- 名称
    pub novel_year: NaiveDate,         // <- 年份
    pub novel_status: String,            // <- 短片类型
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
 * Novel 添加结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct AddNovel {
    // pub id: i32,                    // <- id
    pub seriesflag: bool,
    pub seriesid: i16,
    pub novel_name: String,            // <- 名称
    pub novel_year: NaiveDate,         // <- 年份
    pub novel_status: String,            // <- 短片类型
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
 * Novel 添加 实现
 */
impl TryFrom<web::Json<AddNovel>> for AddNovel {
    type  Error = SEVXError;
    fn try_from(add_novel: web::Json<AddNovel>) -> Result<Self, Self::Error> {
        Ok(AddNovel {
            seriesflag: add_novel.seriesflag,
            seriesid: add_novel.seriesid,
            novel_name: add_novel.novel_name.clone(),
            novel_year: add_novel.novel_year,
            novel_status: add_novel.novel_status.clone(),
            logo: add_novel.logo.clone(),
            author: add_novel.author.clone(),
            localflag: add_novel.localflag,
            localurl: add_novel.localurl.clone(),
            remoteflag: add_novel.remoteflag,
            remoteurl: add_novel.remoteurl.clone(),
            container: add_novel.container.clone(),
            remark: add_novel.remark.clone(),
        })
    }
}


/**
 * Novel 更新 结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateNovel {
    pub id: i32,                            // <- id
    pub seriesflag: Option<bool>,
    pub seriesid: Option<i16>,
    pub novel_name: Option<String>,            // <- 名称
    pub novel_year: Option<NaiveDate>,         // <- 年份
    pub novel_status: Option<String>,            // <- 类型
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
 * 实现-Novel 更新-From
 */
impl From<web::Json<UpdateNovel>> for UpdateNovel {
    fn from(novel: web::Json<UpdateNovel>) -> Self {
        UpdateNovel { 
            id: novel.id,
            seriesflag: novel.seriesflag,
            seriesid: novel.seriesid,
            novel_name: novel.novel_name.clone(),
            novel_year: novel.novel_year,
            novel_status: novel.novel_status.clone(),
            logo: novel.logo.clone(),
            author: novel.author.clone(),
            localflag: novel.localflag,
            localurl: novel.localurl.clone(),
            remoteflag: novel.remoteflag,
            remoteurl: novel.remoteurl.clone(),
            container: novel.container.clone(),
            remark: novel.remark.clone(),
        }
    }
}


/**
 * Novel 删除结构体
 * 仅当口令验证正确的时候才执行删除
 */
#[derive(Deserialize, Debug, Clone)]
pub struct DeleteNovel {
    pub id: i32,                // <- Novel id
    pub name: String,
    pub password: String,       // <- 口令
}
/**
 * Novel 删除 实现
 */
impl From<web::Json<DeleteNovel>> for DeleteNovel {
    fn from(novel: web::Json<DeleteNovel>) -> Self {
        DeleteNovel {
            id: novel.id,
            name: novel.name.clone(),
            password: novel.password.clone(),
        }
    }
}
