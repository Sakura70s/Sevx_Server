use crate::models::music_model::*;
use sqlx::postgres::PgPool;
use crate::error::SEVXError;
use chrono::{NaiveDate, Local};
use crate::log::print_log;


/**
 * 获取所有 Music 列表
 * 成功应返回 Vec<Music>，错误应返回SEVXError
 */
pub async fn get_all_music_db(pool: &PgPool) -> Result<Vec<Music>, SEVXError>{
    let rows = sqlx::query!(
        "Select * from Music Order By id ASC"
    )
    .fetch_all(pool)
    .await?;

    let music_vec: Vec<Music> = rows.iter()
    .map(|item| Music{
        id: item.id,
        music_name: item.music_name.clone(),
        music_year: item.music_year,
        logo: item.logo.clone(),
        artist: item.artist.clone(),
        album: item.album.clone(),
        lyrics: item.lyrics.clone(),
        written: item.written.clone(),
        localflag: item.localflag,
        localurl: item.localurl.clone(),
        remoteflag: item.remoteflag,
        remoteurl: item.remoteurl.clone(),
        container: item.container.clone(),
        lyrictype: item.lyrictype.clone(),
        updatetime: NaiveDate::from(item.updatetime),
        remark: item.remark.clone(),
    }).collect();

    match music_vec.len() {
        0 => Err(SEVXError::NotFound("Not found any Music".into())),
        _ => {
            print_log("Get Music list".to_string());
            Ok(music_vec)
        },
    }
}


/**
 * 根据单一 ID 获取具体 Music
 * 成功返回 Music，失败还不清楚
 */
pub async fn get_music_for_id_db (
    pool: &PgPool,
    id: i32,
) -> Result<Music, SEVXError> {
    let row = sqlx::query_as!(
        Music,
        "Select * from Music where id = $1", id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        // Success
        Some(row) => {
            print_log(format!("Get Music of id:{}, name:[{}]", id, row.music_name));
            Ok(row)
        }
        // Error
        _ => Err(SEVXError::NotFound(format!("Animaton of id = {} is not found!", id)))
    }
}


/**
 * 根据名称查询
 */
pub async fn search_music_for_name_db (
    pool: &PgPool,
    name: String,
) -> Result<Vec<Music>, SEVXError> {
    let rows = sqlx::query!("Select * from Music where music_name = $1 Order By id ASC", name)
    .fetch_all(pool)
    .await?;

    let music_vec: Vec<Music> = rows.iter()
    .map(|item| Music{
        id: item.id,
        music_name: item.music_name.clone(),
        music_year: item.music_year,
        logo: item.logo.clone(),
        artist: item.artist.clone(),
        album: item.album.clone(),
        lyrics: item.lyrics.clone(),
        written: item.written.clone(),
        localflag: item.localflag,
        localurl: item.localurl.clone(),
        remoteflag: item.remoteflag,
        remoteurl: item.remoteurl.clone(),
        container: item.container.clone(),
        lyrictype: item.lyrictype.clone(),
        updatetime: NaiveDate::from(item.updatetime),
        remark: item.remark.clone(),
    }).collect();

    match music_vec.len() {
        0 => Err(SEVXError::NotFound(format!("Not found any Music of name:[{}]", name))),
        _ => {
            print_log(format!("Get Music list of name:[{}]", name));
            Ok(music_vec)
        },
    }
}


/**
 * 添加 Music
 */
pub async fn add_music_db (
    pool: &PgPool,
    add_music: AddMusic,
) -> Result<Music, SEVXError> {
    let row = sqlx::query_as!(
        Music,
        "Insert into Music (
            Music_name,
            Music_year,
            logo,
            artist,
            album,
            lyrics,
            written,
            localFlag,
            localUrl,
            remoteFlag,
            remoteUrl,
            container,
            lyricType,
            remark
        ) Values (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14
        ) Returning
        id,
        music_name,
        music_year,
        logo,
        artist,
        album,
        lyrics,
        written,
        localflag,
        localurl,
        remoteflag,
        remoteurl,
        container,
        lyrictype,
        updatetime,
        remark",
        add_music.music_name,
        add_music.music_year,
        add_music.logo,
        add_music.artist,
        add_music.album,
        add_music.lyrics,
        add_music.written,
        add_music.localflag,
        add_music.localurl,
        add_music.remoteflag,
        add_music.remoteurl,
        add_music.container,
        add_music.lyrictype,
        add_music.remark
    )
    .fetch_one(pool)
    .await?;

    // 成功之后打印 Log， 返回新增加的
    print_log(format!("Add Music of name:[{}]", add_music.music_name));
    Ok(row)
}


/**
 * 更新 Music
 */
pub async fn update_music_db (
    pool: &PgPool,
    update_music: UpdateMusic,
) -> Result<Music, SEVXError> {

    // 判断要更新课程是否存在
    let current_music = sqlx::query_as!(
        Music,
        "Select * from Music where id = $1", update_music.id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| SEVXError::NotFound(format!("Music of id:{} is not found", update_music.id)))?;

    // 存在则继续

    // 配置将要更新的变量
    let music_name: String = match update_music.music_name {
        Some(music_name) => music_name,
        _ => current_music.music_name
    };
    let music_year: NaiveDate = match update_music.music_year {
        Some(music_year) => music_year,
        _ => current_music.music_year,
    };
    let logo: String = match update_music.logo {
        Some(logo) => logo,
        _ => current_music.logo,
    };
    let artist: String = match update_music.artist {
        Some(artist) => artist,
        _ => current_music.artist,
    };
    let album: String = match update_music.album {
        Some(album) => album,
        _ => current_music.album
    };
    let lyrics: String = match update_music.lyrics {
        Some(lyrics) => lyrics,
        _ => current_music.lyrics,
    };
    let written: String = match update_music.written {
        Some(written) => written,
        _ => current_music.written,
    };
    let localflag: bool = match update_music.localflag {
        Some(localflag) => localflag,
        _ => current_music.localflag,
    };
    let localurl: String = match update_music.localurl {
        Some(localurl) => localurl,
        _ => current_music.localurl.unwrap_or_default(),
    };
    let remoteflag: bool = match update_music.remoteflag {
        Some(remoteflag) => remoteflag,
        _ => current_music.remoteflag,
    };
    let remoteurl: String = match update_music.remoteurl {
        Some(remoteurl) => remoteurl,
        _ => current_music.remoteurl.unwrap_or_default(),
    };
    let container: String = match update_music.container {
        Some(container) => container,
        _ => current_music.container,
    };
    let lyrictype: String = match update_music.lyrictype {
        Some(lyrictype) => lyrictype,
        _ => current_music.lyrictype,
    };
    let updatetime: NaiveDate = {
        let fmt = "%Y-%m-%d";
        let now = format!("{}", Local::now().format(fmt));
        NaiveDate::parse_from_str(&now, "%Y-%m-%d").unwrap()
    };
    let remark: String = match update_music.remark {
        Some(remark) => remark,
        _ => current_music.remark.unwrap_or_default(),
    };

    // 修改
    let music_row = sqlx::query_as!(
        Music,
        "
        Update Music 
        set
        Music_name = $1,
        Music_year = $2,
        logo = $3,
        artist = $4,
        album = $5,
        lyrics = $6,
        written = $7,
        localFlag = $8,
        localUrl = $9,
        remoteFlag = $10,
        remoteUrl = $11,
        container = $12,
        lyricType = $13,
        updatetime = $14,
        remark = $15
        Where id = $16
            Returning
            id,
            music_name,
            music_year,
            logo,
            artist,
            album,
            lyrics,
            written,
            localflag,
            localurl,
            remoteflag,
            remoteurl,
            container,
            lyrictype,
            updatetime,
            remark
        ",
                music_name,
                music_year, 
                logo,
                artist,
                album,
                lyrics,
                written,
                localflag,
                localurl,
                remoteflag,
                remoteurl,
                container,
                lyrictype,
                updatetime,
                remark,
                update_music.id,
    )
    .fetch_one(pool)
    .await;

    // 判断是否修改成功
    match music_row {
        Ok(music_row) => {
            print_log(format!("Update Music of id:{}", update_music.id));
            Ok(music_row)
        },
        Err(_music_row) => Err(SEVXError::DBError("Update Failed".into())),
    }
}


/**
 * 删除 Music DB
 */
pub async fn delete_music_db (
    pool: &PgPool,
    delete_music: DeleteMusic,
) -> Result<String, SEVXError> {

    // 首先判断口令是否正确
    if delete_music.password.eq("114514") {
        
        // 判断当前 Music 是否存在
        let current_row = sqlx::query_as!(
            Music,
            "Select * From Music where id = $1",
            delete_music.id
        )
        .fetch_optional(pool).await?;
        match current_row {

            // 存在则执行删除
            Some(_current_row) => {
                let _row = sqlx::query!(
                    "Delete From music where id = $1",
                    delete_music.id
                )
                .execute(pool)
                .await?;
                print_log(format!("Delete Music of id:{}", delete_music.id));
                Ok(format!("Delete Music of id:{}", delete_music.id))
            },

            // 不存在返回错误
            _ => Err(SEVXError::NotFound(format!("Music of id:{} is not found", delete_music.id)))
        }
    
    // 口令不正确提示认证错误
    } else {
        Err(SEVXError::AuthFailed("Password Error".into()))
    }
}
