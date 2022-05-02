use actix_web::web;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use crate::error::SEVXError;
use std::convert::TryFrom;

/**
 * Music 结构体
 */
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Music {
    pub id: i32,                    // <- id
    pub music_name: String,         // <- 名称
    pub music_year: NaiveDate,      // <- 年份
    pub logo: String,               // <- 专辑封面
    pub artist: String,             // <- 艺术家
    pub album: String,              // <- 专辑
    pub lyrics: String,             // <- 作词
    pub written: String,            // <- 作曲
    pub localflag: bool,            // <- 本地 Flag
    pub localurl: Option<String>,   // <- 本地 URL      Null
    pub remoteflag: bool,           // <- 远程 Flag 
    pub remoteurl: Option<String>,  // <- 远程 URL      Null
    pub container: String,          // <- 容器格式
    pub lyrictype: String,          // <- 歌词类型
    pub updatetime: NaiveDate,      // <- 更新时间
    pub remark: Option<String>,     // <- 备注          Null
}


/**
 * Music 添加结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct AddMusic {
    // pub id: i32,                    // <- id
    pub music_name: String,         // <- 名称
    pub music_year: NaiveDate,      // <- 年份
    pub logo: String,               // <- 专辑封面
    pub artist: String,             // <- 艺术家
    pub album: String,              // <- 专辑
    pub lyrics: String,             // <- 作词
    pub written: String,            // <- 作曲
    pub localflag: bool,            // <- 本地 Flag
    pub localurl: Option<String>,   // <- 本地 URL      Null
    pub remoteflag: bool,           // <- 远程 Flag 
    pub remoteurl: Option<String>,  // <- 远程 URL      Null
    pub container: String,          // <- 容器格式
    pub lyrictype: String,          // <- 歌词类型
    // pub updatetime: NaiveDate,      // <- 更新时间
    pub remark: Option<String>,     // <- 备注          Null
}
/**
 * Music 添加 实现
 */
impl TryFrom<web::Json<AddMusic>> for AddMusic {
    type Error = SEVXError;
    fn try_from(add_music: web::Json<AddMusic>) -> Result<Self, Self::Error> {
        Ok(AddMusic {
            music_name: add_music.music_name.clone(),
            music_year: add_music.music_year,
            logo: add_music.logo.clone(),
            artist: add_music.artist.clone(),
            album: add_music.album.clone(),
            lyrics: add_music.lyrics.clone(),
            written: add_music.written.clone(),
            localflag: add_music.localflag,
            localurl: add_music.localurl.clone(),
            remoteflag: add_music.remoteflag,
            remoteurl: add_music.remoteurl.clone(),
            container: add_music.container.clone(),
            lyrictype: add_music.lyrictype.clone(),
            remark: add_music.remark.clone(),
        })
    }
}


/**
 * Music 更新 结构体
 */
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateMusic {
    pub id: i32,                            // <- id
    pub music_name: Option<String>,         // <- 名称
    pub music_year: Option<NaiveDate>,      // <- 年份
    pub logo: Option<String>,               // <- 专辑封面
    pub artist: Option<String>,             // <- 作者
    pub album: Option<String>,              // <- 专辑
    pub lyrics: Option<String>,             // <- 作词
    pub written: Option<String>,            // <- 作曲
    pub localflag: Option<bool>,            // <- 本地 Flag
    pub localurl: Option<String>,           // <- 本地 URL      Null
    pub remoteflag: Option<bool>,           // <- 远程 Flag 
    pub remoteurl: Option<String>,          // <- 远程 URL      Null
    pub container: Option<String>,          // <- 容器格式
    pub lyrictype: Option<String>,          // <- 歌词类型
    pub remark: Option<String>,             // <- 备注          Null
}
/**
 * 实现-Music 更新-From
 */
impl From<web::Json<UpdateMusic>> for UpdateMusic {
    fn from(music: web::Json<UpdateMusic>) -> Self {
        UpdateMusic { 
            id: music.id,
            music_name: music.music_name.clone(),
            music_year: music.music_year,
            logo: music.logo.clone(),
            artist: music.artist.clone(),
            album: music.album.clone(),
            lyrics: music.lyrics.clone(),
            written: music.written.clone(),
            localflag: music.localflag,
            localurl: music.localurl.clone(),
            remoteflag: music.remoteflag,
            remoteurl: music.remoteurl.clone(),
            container: music.container.clone(),
            lyrictype: music.lyrictype.clone(),
            remark: music.remark.clone(),
        }
    }
}


/**
 * Music 删除结构体
 * 仅当口令验证正确的时候才执行删除
 */
#[derive(Deserialize, Debug, Clone)]
pub struct DeleteMusic {
    pub id: i32,                // <- Music id
    pub name: String,
    pub password: String,       // <- 口令
}
/**
 * Music 删除 实现
 */
impl From<web::Json<DeleteMusic>> for DeleteMusic {
    fn from(music: web::Json<DeleteMusic>) -> Self {
        DeleteMusic {
            id: music.id,
            name: music.name.clone(),
            password: music.password.clone(),
        }
    }
}
