use crate::models::comic_model::*;
use sqlx::postgres::PgPool;
use crate::error::SEVXError;
use chrono::{NaiveDate, Local};
use crate::log::print_log;


/**
 * 获取所有 Comic 列表
 * 成功应返回 Vec<Comic>，错误应返回SEVXError
 */
pub async fn get_all_comic_db(pool: &PgPool) -> Result<Vec<Comic>, SEVXError>{
    let rows = sqlx::query!(
        "Select * from Comic Order By id ASC"
    )
    .fetch_all(pool)
    .await?;

    let comic_vec: Vec<Comic> = rows.iter()
    .map(|item| Comic{
        id: item.id,
        seriesflag: item.seriesflag,
        seriesid: item.seriesid,
        comic_name: item.comic_name.clone(),
        comic_year: item.comic_year,
        comic_status: item.comic_status.clone(),
        logo: item.logo.clone(),
        author: item.author.clone(),
        localflag: item.localflag,
        localurl: item.localurl.clone(),
        remoteflag: item.remoteflag,
        remoteurl: item.remoteurl.clone(),
        container: item.container.clone(),
        updatetime: NaiveDate::from(item.updatetime),
        remark: item.remark.clone(),
    }).collect();

    match comic_vec.len() {
        0 => Err(SEVXError::NotFound("Not found any Comic".into())),
        _ => {
            print_log("Get Comic list".to_string());
            Ok(comic_vec)
        },
    }
}


/**
 * 根据单一 ID 获取具体 Comic
 * 成功返回 Comic，失败还不清楚
 */
pub async fn get_comic_for_id_db (
    pool: &PgPool,
    id: i32,
) -> Result<Comic, SEVXError> {
    let row = sqlx::query_as!(
        Comic,
        "Select * from Comic where id = $1", id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        // Success
        Some(row) => {
            print_log(format!("Get Comic of id:{}, name:[{}]", id, row.comic_name));
            Ok(row)
        }
        // Error
        _ => Err(SEVXError::NotFound(format!("Animaton of id = {} is not found!", id)))
    }
}


/**
 * 根据名称查询
 */
pub async fn search_comic_for_name_db (
    pool: &PgPool,
    name: String,
) -> Result<Vec<Comic>, SEVXError> {
    let rows = sqlx::query!("Select * from Comic where comic_name = $1 Order By id ASC", name)
    .fetch_all(pool)
    .await?;

    let comic_vec: Vec<Comic> = rows.iter()
    .map(|item| Comic{
        id: item.id,
        seriesflag: item.seriesflag,
        seriesid: item.seriesid,
        comic_name: item.comic_name.clone(),
        comic_year: item.comic_year,
        comic_status: item.comic_status.clone(),
        logo: item.logo.clone(),
        author: item.author.clone(),
        localflag: item.localflag,
        localurl: item.localurl.clone(),
        remoteflag: item.remoteflag,
        remoteurl: item.remoteurl.clone(),
        container: item.container.clone(),
        updatetime: NaiveDate::from(item.updatetime),
        remark: item.remark.clone(),
    }).collect();

    match comic_vec.len() {
        0 => Err(SEVXError::NotFound(format!("Not found any Comic of name:[{}]", name))),
        _ => {
            print_log(format!("Get Comic list of name:[{}]", name));
            Ok(comic_vec)
        },
    }
}


/**
 * 添加 Comic
 */
pub async fn add_comic_db (
    pool: &PgPool,
    add_comic: AddComic,
) -> Result<Comic, SEVXError> {
    let row = sqlx::query_as!(
        Comic,
        "Insert into Comic (
            seriesflag,
            seriesid,
            Comic_name,
            Comic_year,
            Comic_status,
            logo,
            author,
            localFlag,
            localUrl,
            remoteFlag,
            remoteUrl,
            container,
            remark
        ) Values (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13
        ) Returning
        id,
        seriesflag,
        seriesid,
        comic_name,
        comic_year,
        comic_status,
        logo,
        author,
        localflag,
        localurl,
        remoteflag,
        remoteurl,
        container,
        updatetime,
        remark",
        add_comic.seriesflag,
        add_comic.seriesid,
        add_comic.comic_name,
        add_comic.comic_year,
        add_comic.comic_status,
        add_comic.logo,
        add_comic.author,
        add_comic.localflag,
        add_comic.localurl,
        add_comic.remoteflag,
        add_comic.remoteurl,
        add_comic.container,
        add_comic.remark
    )
    .fetch_one(pool)
    .await?;

    // 成功之后打印 Log， 返回新增加的
    print_log(format!("Add Comic of name:[{}]", add_comic.comic_name));
    Ok(row)
}


/**
 * 更新 Comic
 */
pub async fn update_comic_db (
    pool: &PgPool,
    update_comic: UpdateComic,
) -> Result<Comic, SEVXError> {

    // 判断要更新课程是否存在
    let current_comic = sqlx::query_as!(
        Comic,
        "Select * from Comic where id = $1", update_comic.id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| SEVXError::NotFound(format!("Comic of id:{} is not found", update_comic.id)))?;

    // 存在则继续

    // 配置将要更新的变量
    let seriesflag: bool = match update_comic.seriesflag {
        Some(seriesflag) => seriesflag,
        _ => current_comic.seriesflag,
    };
    let seriesid: i16 = match update_comic.seriesid {
        Some(seriesid) => seriesid,
        _ => current_comic.seriesid,
    };
    let comic_name: String = match update_comic.comic_name {
        Some(comic_name) => comic_name,
        _ => current_comic.comic_name
    };
    let comic_year: NaiveDate = match update_comic.comic_year {
        Some(comic_year) => comic_year,
        _ => current_comic.comic_year,
    };
    let comic_status: String = match update_comic.comic_status {
        Some(comic_status) => comic_status,
        _ => current_comic.comic_status,
    };
    let logo: String = match update_comic.logo {
        Some(logo) => logo,
        _ => current_comic.logo,
    };
    let author: String = match update_comic.author {
        Some(author) => author,
        _ => current_comic.author,
    };
    let localflag: bool = match update_comic.localflag {
        Some(localflag) => localflag,
        _ => current_comic.localflag,
    };
    let localurl: String = match update_comic.localurl {
        Some(localurl) => localurl,
        _ => current_comic.localurl.unwrap_or_default(),
    };
    let remoteflag: bool = match update_comic.remoteflag {
        Some(remoteflag) => remoteflag,
        _ => current_comic.remoteflag,
    };
    let remoteurl: String = match update_comic.remoteurl {
        Some(remoteurl) => remoteurl,
        _ => current_comic.remoteurl.unwrap_or_default(),
    };
    let container: String = match update_comic.container {
        Some(container) => container,
        _ => current_comic.container,
    };
    let updatetime: NaiveDate = {
        let fmt = "%Y-%m-%d";
        let now = format!("{}", Local::now().format(fmt));
        NaiveDate::parse_from_str(&now, "%Y-%m-%d").unwrap()
    };
    let remark: String = match update_comic.remark {
        Some(remark) => remark,
        _ => current_comic.remark.unwrap_or_default(),
    };

    // 修改
    let comic_row = sqlx::query_as!(
        Comic,
        "
        Update Comic 
        set 
        seriesflag = $1,
        seriesid = $2,
        Comic_name = $3,
        Comic_year = $4,
        Comic_status = $5,
        logo = $6,
        author = $7,
        localFlag = $8,
        localUrl = $9,
        remoteFlag = $10,
        remoteUrl = $11,
        container = $12,
        updatetime = $13,
        remark = $14
        Where id = $15
            Returning
            id,
            seriesflag,
            seriesid,
            comic_name,
            comic_year,
            comic_status,
            logo,
            author,
            localflag,
            localurl,
            remoteflag,
            remoteurl,
            container,
            updatetime,
            remark
        ",
                seriesflag,
                seriesid,
                comic_name,
                comic_year, 
                comic_status,
                logo,
                author,
                localflag,
                localurl,
                remoteflag,
                remoteurl,
                container,
                updatetime,
                remark,
                update_comic.id,
    )
    .fetch_one(pool)
    .await;

    // 判断是否修改成功
    match comic_row {
        Ok(comic_row) => {
            print_log(format!("Update Comic of id:{}, name:[{}]", update_comic.id, comic_name));
            Ok(comic_row)
        },
        Err(_comic_row) => Err(SEVXError::DBError("Update Failed".into())),
    }
}


/**
 * 删除 Comic DB
 */
pub async fn delete_comic_db (
    pool: &PgPool,
    delete_comic: DeleteComic,
) -> Result<String, SEVXError> {

    // 首先判断口令是否正确
    if delete_comic.password.eq("114514") {
        
        // 判断当前 Comic 是否存在
        let current_row = sqlx::query_as!(
            Comic,
            "Select * From Comic where id = $1",
            delete_comic.id
        )
        .fetch_optional(pool).await?;
        match current_row {

            // 存在则执行删除
            Some(_current_row) => {
                let _row = sqlx::query!(
                    "Delete From comic where id = $1",
                    delete_comic.id
                )
                .execute(pool)
                .await?;
                print_log(format!("Delete Comic of id:{}", delete_comic.id));
                Ok(format!("Delete Comic of id:{}", delete_comic.id))
            },

            // 不存在返回错误
            _ => Err(SEVXError::NotFound(format!("Comic of id:{} is not found", delete_comic.id)))
        }
    
    // 口令不正确提示认证错误
    } else {
        Err(SEVXError::AuthFailed("Password Error".into()))
    }
}
