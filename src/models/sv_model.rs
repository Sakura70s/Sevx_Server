use actix_web::web;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use crate::error::SEVXError;
use std::convert::TryFrom;

/**
 * Sv 结构体
 */
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Sv {
    pub id: i32,                    // <- id
    pub sv_name: String,            // <- 名称
    pub sv_year: NaiveDate,         // <- 年份
    pub sv_type: String,            // <- 短片类型
    pub logo: String,               // <- 剧照
    pub author: String,             // <- 作者
    pub localflag: bool,            // <- 本地 Flag
    pub localurl: Option<String>,   // <- 本地 URL      Null
    pub remoteflag: bool,           // <- 远程 Flag 
    pub remoteurl: Option<String>,  // <- 远程 URL      Null
    pub container: String,          // <- 容器格式
    pub codev: String,              // <- 本地视频编码格式
    pub codea: String,              // <- 本地音频编码格式
    pub updatetime: NaiveDate,      // <- 更新时间
    pub remark: Option<String>,     // <- 备注          Null
}


/**
 * Sv 添加结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct AddSv {
    pub sv_name: String,            // <- 名称
    pub sv_year: NaiveDate,         // <- 年份
    pub sv_type: String,            // <- 短片类型
    pub logo: String,               // <- 剧照
    pub author: String,             // <- 作者
    pub localflag: bool,            // <- 本地 Flag
    pub localurl: Option<String>,   // <- 本地 URL      Null
    pub remoteflag: bool,           // <- 远程 Flag 
    pub remoteurl: Option<String>,  // <- 远程 URL      Null
    pub container: String,          // <- 容器格式
    pub codev: String,              // <- 本地视频编码格式
    pub codea: String,              // <- 本地音频编码格式
    pub remark: Option<String>,     // <- 备注          Null
}
/**
 * Sv 添加 实现
 */
impl TryFrom<web::Json<AddSv>> for AddSv {
    type Error = SEVXError;
    fn try_from(add_sv: web::Json<AddSv>) -> Result<Self, Self::Error> {
        Ok(AddSv {
            sv_name: add_sv.sv_name.clone(),
            sv_year: add_sv.sv_year,
            sv_type: add_sv.sv_type.clone(),
            logo: add_sv.logo.clone(),
            author: add_sv.author.clone(),
            localflag: add_sv.localflag,
            localurl: add_sv.localurl.clone(),
            remoteflag: add_sv.remoteflag,
            remoteurl: add_sv.remoteurl.clone(),
            container: add_sv.container.clone(),
            codev: add_sv.codev.clone(),
            codea: add_sv.codea.clone(),
            remark: add_sv.remark.clone(),
        })
    }
}


/**
 * Sv 更新 结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateSv {
    pub id: i32,                            // <- id
    pub sv_name: Option<String>,            // <- 名称
    pub sv_year: Option<NaiveDate>,         // <- 年份
    pub sv_type: Option<String>,            // <- 类型
    pub logo: Option<String>,               // <- 剧照
    pub author: Option<String>,             // <- 作者
    pub localflag: Option<bool>,            // <- 本地 Flag
    pub localurl: Option<String>,           // <- 本地 URL      Null
    pub remoteflag: Option<bool>,           // <- 远程 Flag 
    pub remoteurl: Option<String>,          // <- 远程 URL      Null
    pub container: Option<String>,          // <- 容器格式
    pub codev: Option<String>,              // <- 本地视频编码格式
    pub codea: Option<String>,              // <- 本地音频编码格式
    pub remark: Option<String>,             // <- 备注          Null
}
/**
 * 实现-Sv 更新-From
 */
impl From<web::Json<UpdateSv>> for UpdateSv {
    fn from(sv: web::Json<UpdateSv>) -> Self {
        UpdateSv { 
            id: sv.id,
            sv_name: sv.sv_name.clone(),
            sv_year: sv.sv_year,
            sv_type: sv.sv_type.clone(),
            logo: sv.logo.clone(),
            author: sv.author.clone(),
            localflag: sv.localflag,
            localurl: sv.localurl.clone(),
            remoteflag: sv.remoteflag,
            remoteurl: sv.remoteurl.clone(),
            container: sv.container.clone(),
            codev: sv.codev.clone(),
            codea: sv.codea.clone(),
            remark: sv.remark.clone(),
        }
    }
}


/**
 * Sv 删除结构体
 * 仅当口令验证正确的时候才执行删除
 */
#[derive(Deserialize, Debug, Clone)]
pub struct DeleteSv {
    pub id: i32,                // <- Sv id
    pub name: String,
    pub password: String,       // <- 口令
}
/**
 * Sv 删除 实现
 */
impl From<web::Json<DeleteSv>> for DeleteSv {
    fn from(sv: web::Json<DeleteSv>) -> Self {
        DeleteSv {
            id: sv.id,
            name: sv.name.clone(),
            password: sv.password.clone(),
        }
    }
}
