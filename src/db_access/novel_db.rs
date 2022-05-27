use crate::models::novel_model::*;
use crate::models::auth_model::*;
use sqlx::postgres::PgPool;
use crate::error::SEVXError;
use chrono::{NaiveDate, Local};
use crate::db_access::auth_db::*;
use crate::log::print_log;


/**
 * 获取所有 Novel 列表
 * 成功应返回 Vec<Novel>，错误应返回SEVXError
 */
pub async fn get_all_novel_db(pool: &PgPool) -> Result<Vec<Novel>, SEVXError>{
    let rows = sqlx::query!(
        "Select * from Novel Order By id ASC"
    )
    .fetch_all(pool)
    .await?;

    let novel_vec: Vec<Novel> = rows.iter()
    .map(|item| Novel{
        id: item.id,
        seriesflag: item.seriesflag,
        seriesid: item.seriesid,
        novel_name: item.novel_name.clone(),
        novel_year: item.novel_year,
        novel_status: item.novel_status.clone(),
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

    match novel_vec.len() {
        0 => Err(SEVXError::NotFound("Not found any Novel".into())),
        _ => {
            print_log("Get Novel list".to_string());
            Ok(novel_vec)
        },
    }
}


/**
 * 根据单一 ID 获取具体 Novel
 * 成功返回 Novel，失败还不清楚
 */
pub async fn get_novel_for_id_db (
    pool: &PgPool,
    id: i32,
) -> Result<Novel, SEVXError> {
    let row = sqlx::query_as!(
        Novel,
        "Select * from Novel where id = $1", id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        // Success
        Some(row) => {
            print_log(format!("Get Novel of id:{}, name:[{}]", id, row.novel_name));
            Ok(row)
        }
        // Error
        _ => Err(SEVXError::NotFound(format!("Animaton of id = {} is not found!", id)))
    }
}


/**
 * 根据名称查询
 */
pub async fn search_novel_for_name_db (
    pool: &PgPool,
    name: String,
) -> Result<Vec<Novel>, SEVXError> {
    let rows = sqlx::query!("Select * from Novel where novel_name = $1 Order By id ASC", name)
    .fetch_all(pool)
    .await?;

    let novel_vec: Vec<Novel> = rows.iter()
    .map(|item| Novel{
        id: item.id,
        seriesflag: item.seriesflag,
        seriesid: item.seriesid,
        novel_name: item.novel_name.clone(),
        novel_year: item.novel_year,
        novel_status: item.novel_status.clone(),
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

    match novel_vec.len() {
        0 => Err(SEVXError::NotFound(format!("Not found any Novel of name:[{}]", name))),
        _ => {
            print_log(format!("Get Novel list of name:[{}]", name));
            Ok(novel_vec)
        },
    }
}


/**
 * 添加 Novel
 */
pub async fn add_novel_db (
    pool: &PgPool,
    add_novel: AddNovel,
    auth: Auth,
) -> Result<Novel, SEVXError> {
    let auth_res = get_auth_db(&pool, auth.uname.clone(), auth.upassword.clone()).await;
    match auth_res {
        Ok(_) => {
            let row = sqlx::query_as!(
                Novel,
                "Insert into Novel (
                    seriesflag,
                    seriesid,
                    Novel_name,
                    Novel_year,
                    Novel_status,
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
                novel_name,
                novel_year,
                novel_status,
                logo,
                author,
                localflag,
                localurl,
                remoteflag,
                remoteurl,
                container,
                updatetime,
                remark",
                add_novel.seriesflag,
                add_novel.seriesid,
                add_novel.novel_name,
                add_novel.novel_year,
                add_novel.novel_status,
                add_novel.logo,
                add_novel.author,
                add_novel.localflag,
                add_novel.localurl,
                add_novel.remoteflag,
                add_novel.remoteurl,
                add_novel.container,
                add_novel.remark
            )
            .fetch_one(pool)
            .await?;
        
            // 成功之后打印 Log， 返回新增加的
            print_log(format!("Add Novel of name:[{}]", add_novel.novel_name));
            Ok(row)
        }

        Err(_) => Err(SEVXError::AuthFailed(format!("Auth Failed of name:[{}] for Add Novel", auth.uname)))
    }
}


/**
 * 更新 Novel
 */
pub async fn update_novel_db (
    pool: &PgPool,
    update_novel: UpdateNovel,
    auth: Auth,
) -> Result<Novel, SEVXError> {
    let auth_res = get_auth_db(&pool, auth.uname.clone(), auth.upassword.clone()).await;
    match auth_res {
        Ok(_) => {
            // 判断要更新课程是否存在
            let current_novel = sqlx::query_as!(
                Novel,
                "Select * from Novel where id = $1", update_novel.id
            )
            .fetch_one(pool)
            .await
            .map_err(|_err| SEVXError::NotFound(format!("Novel of id:{} is not found", update_novel.id)))?;

            // 存在则继续

            // 配置将要更新的变量
            let seriesflag: bool = match update_novel.seriesflag {
                Some(seriesflag) => seriesflag,
                _ => current_novel.seriesflag,
            };
            let seriesid: i16 = match update_novel.seriesid {
                Some(seriesid) => seriesid,
                _ => current_novel.seriesid,
            };
            let novel_name: String = match update_novel.novel_name {
                Some(novel_name) => novel_name,
                _ => current_novel.novel_name
            };
            let novel_year: NaiveDate = match update_novel.novel_year {
                Some(novel_year) => novel_year,
                _ => current_novel.novel_year,
            };
            let novel_status: String = match update_novel.novel_status {
                Some(novel_status) => novel_status,
                _ => current_novel.novel_status,
            };
            let logo: String = match update_novel.logo {
                Some(logo) => logo,
                _ => current_novel.logo,
            };
            let author: String = match update_novel.author {
                Some(author) => author,
                _ => current_novel.author,
            };
            let localflag: bool = match update_novel.localflag {
                Some(localflag) => localflag,
                _ => current_novel.localflag,
            };
            let localurl: String = match update_novel.localurl {
                Some(localurl) => localurl,
                _ => current_novel.localurl.unwrap_or_default(),
            };
            let remoteflag: bool = match update_novel.remoteflag {
                Some(remoteflag) => remoteflag,
                _ => current_novel.remoteflag,
            };
            let remoteurl: String = match update_novel.remoteurl {
                Some(remoteurl) => remoteurl,
                _ => current_novel.remoteurl.unwrap_or_default(),
            };
            let container: String = match update_novel.container {
                Some(container) => container,
                _ => current_novel.container,
            };
            let updatetime: NaiveDate = {
                let fmt = "%Y-%m-%d";
                let now = format!("{}", Local::now().format(fmt));
                NaiveDate::parse_from_str(&now, "%Y-%m-%d").unwrap()
            };
            let remark: String = match update_novel.remark {
                Some(remark) => remark,
                _ => current_novel.remark.unwrap_or_default(),
            };

            // 修改
            let novel_row = sqlx::query_as!(
                Novel,
                "
                Update Novel 
                set 
                seriesflag = $1,
                seriesid = $2,
                Novel_name = $3,
                Novel_year = $4,
                Novel_status = $5,
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
                    novel_name,
                    novel_year,
                    novel_status,
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
                        novel_name,
                        novel_year, 
                        novel_status,
                        logo,
                        author,
                        localflag,
                        localurl,
                        remoteflag,
                        remoteurl,
                        container,
                        updatetime,
                        remark,
                        update_novel.id,
            )
            .fetch_one(pool)
            .await;

            // 判断是否修改成功
            match novel_row {
                Ok(novel_row) => {
                    print_log(format!("Update Novel of id:{}, name:[{}]", update_novel.id, novel_name));
                    Ok(novel_row)
                },
                Err(_novel_row) => Err(SEVXError::DBError("Update Failed".into())),
            }
        }

        Err(_) => Err(SEVXError::AuthFailed(format!("Auth Failed of name:[{}] for Update Novel", auth.uname)))
    }
}


/**
 * 删除 Novel DB
 */
pub async fn delete_novel_db (
    pool: &PgPool,
    delete_novel: DeleteNovel,
) -> Result<String, SEVXError> {
    // 先进行身份验证
    let auth = get_auth_db(&pool, delete_novel.name.clone(), delete_novel.password.clone()).await;
    match auth {
        Ok(_) => {
            // 判断当前 Novel 是否存在
            let current_row = sqlx::query_as!(
                Novel,
                "Select * From Novel where id = $1",
                delete_novel.id
            )
            .fetch_optional(pool).await?;
            match current_row {

                // 存在则执行删除
                Some(_current_row) => {
                    let _row = sqlx::query!(
                        "Delete From Novel where id = $1",
                        delete_novel.id
                    )
                    .execute(pool)
                    .await?;
                    print_log(format!("Delete Novel of id:{}", delete_novel.id));
                    Ok(format!("Delete Novel of id:{}", delete_novel.id))
                },

                // 不存在返回错误
                _ => Err(SEVXError::NotFound(format!("Novel of id:{} is not found", delete_novel.id)))
            }
        }

        Err(_) => Err(SEVXError::AuthFailed(format!("Auth Failed of name:[{}] for delete Novel", delete_novel.name)))
    }
}
