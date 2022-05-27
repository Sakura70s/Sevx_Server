use crate::models::animation_model::*;
use crate::models::auth_model::*;
use sqlx::postgres::PgPool;
use crate::error::SEVXError;
use chrono::{NaiveDate, Local};
use crate::db_access::auth_db::*;
use crate::log::print_log;


/**
 * 获取所有 Animation 列表
 * 成功应返回 Vec<Animation>，错误应返回SEVXError
 */
pub async fn get_all_animation_db(pool: &PgPool) -> Result<Vec<Animation>, SEVXError>{
    let rows = sqlx::query!(
        "Select * from Animation Order By id ASC"
    )
    .fetch_all(pool)
    .await?;

    let animation_vec: Vec<Animation> = rows.iter()
    .map(|item| Animation{
        id: item.id,
        seriesflag: item.seriesflag,
        seriesid: item.seriesid,
        animation_name: item.animation_name.clone(),
        animation_year: item.animation_year,
        director: item.director.clone(),
        screenwriter: item.screenwriter.clone(),
        make: item.make.clone(),
        logo: item.logo.clone(),
        amount: item.amount,
        localflag: item.localflag,
        localurl: item.localurl.clone(),
        remoteflag: item.remoteflag,
        remoteurl: item.remoteurl.clone(),
        container: item.container.clone(),
        codev: item.codev.clone(),
        codea: item.codea.clone(),
        subtype: item.subtype.clone(),
        subteam: item.subteam.clone(),
        lastwatch: item.lastwatch,
        updatetime: NaiveDate::from(item.updatetime),
        remark: item.remark.clone(),
    }).collect();

    // 若集合长度为0则证明没有找到
    match animation_vec.len() {
        0 => Err(SEVXError::NotFound("Not found any Animation".into())),
        _ => {
            print_log("Get Animation list".to_string());
            Ok(animation_vec)
        },
    }
}


/**
 * 根据单一 ID 获取具体 Animation
 * 成功返回 Animation，失败还不清楚
 */
pub async fn get_animation_for_id_db (
    pool: &PgPool,
    id: i32,
) -> Result<Animation, SEVXError> {
    let row = sqlx::query_as!(
        Animation,
        "Select * from Animation where id = $1", id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        // Success
        Some(row) => {
            print_log(format!("Get Animation of id:{}, name:[{}]", id, row.animation_name));
            Ok(row)
        }
        // Error
        _ => Err(SEVXError::NotFound(format!("Animaton of id = {} is not found!", id)))
    }
}


/**
 * 根据名称查询
 */
pub async fn search_animation_for_name_db (
    pool: &PgPool,
    name: String,
) -> Result<Vec<Animation>, SEVXError> {
    let rows = sqlx::query!("Select * from Animation where animation_name = $1 Order By id ASC", name)
    .fetch_all(pool)
    .await?;

    let animation_vec: Vec<Animation> = rows.iter()
    .map(|item| Animation{
        id: item.id,
        seriesflag: item.seriesflag,
        seriesid: item.seriesid,
        animation_name: item.animation_name.clone(),
        animation_year: item.animation_year,
        director: item.director.clone(),
        screenwriter: item.screenwriter.clone(),
        make: item.make.clone(),
        logo: item.logo.clone(),
        amount: item.amount,
        localflag: item.localflag,
        localurl: item.localurl.clone(),
        remoteflag: item.remoteflag,
        remoteurl: item.remoteurl.clone(),
        container: item.container.clone(),
        codev: item.codev.clone(),
        codea: item.codea.clone(),
        subtype: item.subtype.clone(),
        subteam: item.subteam.clone(),
        lastwatch: item.lastwatch,
        updatetime: NaiveDate::from(item.updatetime),
        remark: item.remark.clone(),
    }).collect();

    match animation_vec.len() {
        0 => Err(SEVXError::NotFound(format!("Not found any Animation of name:[{}]", name))),
        _ => {
            print_log(format!("Get Animation list of name:[{}]", name));
            Ok(animation_vec)
        },
    }
}


/**
 * 添加 Animation
 */
pub async fn add_animation_db (
    pool: &PgPool,
    add_animation: AddAnimation,
    auth: Auth,
) -> Result<Animation, SEVXError> {

    let auth_res = get_auth_db(&pool, auth.uname.clone(), auth.upassword.clone()).await;
    match auth_res {
        Ok(_) => {
            let row = sqlx::query_as!(
                Animation,
                "Insert into Animation (
                    seriesFlag,
                    seriesId,
                    Animation_name,
                    Animation_year,
                    director,
                    screenWriter,
                    make,
                    logo,
                    amount,
                    localFlag,
                    localUrl,
                    remoteFlag,
                    remoteUrl,
                    container,
                    codev,
                    codea,
                    subType,
                    subTeam,
                    remark
                ) Values (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                    $11, $12, $13, $14, $15, $16, $17, $18, $19
                ) Returning
                id,
                seriesFlag,
                seriesId,
                animation_name,
                animation_year,
                director,
                screenwriter,
                make,
                logo,
                amount,
                localflag,
                localurl,
                remoteflag,
                remoteurl,
                container,
                codev,
                codea,
                subtype,
                subteam,
                lastwatch,
                updatetime,
                remark",
                add_animation.seriesflag,
                add_animation.seriesid,
                add_animation.animation_name,
                add_animation.animation_year,
                add_animation.director,
                add_animation.screenwriter,
                add_animation.make,
                add_animation.logo,
                add_animation.amount,
                add_animation.localflag,
                add_animation.localurl,
                add_animation.remoteflag,
                add_animation.remoteurl,
                add_animation.container,
                add_animation.codev,
                add_animation.codea,
                add_animation.subtype,
                add_animation.subteam,
                add_animation.remark
            )
            .fetch_one(pool)
            .await?;
        
            // 成功之后打印 Log， 返回新增加的
            print_log(format!("Add Animation of name:[{}]", add_animation.animation_name));
            Ok(row)
        }
        Err(_) => Err(SEVXError::AuthFailed(format!("Auth Failed of name:[{}] for update Animation", auth.uname)))
    }

}


/**
 * 更新 Animation
 */
pub async fn update_animation_db (
    pool: &PgPool,
    update_animation: UpdateAnimation,
    auth: Auth,
) -> Result<Animation, SEVXError> {
    // 鉴权
    let auth_res = get_auth_db(&pool, auth.uname.clone(), auth.upassword.clone()).await;
    match auth_res {
        // 认证成功
        Ok(_) => {
            // 判断要更新课程是否存在
            let current_animation = sqlx::query_as!(
                Animation,
                "Select * from Animation where id = $1", update_animation.id
            )
            .fetch_one(pool)
            .await
            .map_err(|_err| SEVXError::NotFound(format!("Animation of id:{} is not found", update_animation.id)))?;

            // 存在则继续

            // 配置将要更新的变量
            let seriesflag: bool = match update_animation.seriesflag {
                Some(seriesflag) => seriesflag,
                _ => current_animation.seriesflag,
            };
            let seriesid: i16 = match update_animation.seriesid {
                Some(seriesid) => seriesid,
                _ => current_animation.seriesid,
            };
            let animation_name: String = match update_animation.animation_name {
                Some(animation_name) => animation_name,
                _ => current_animation.animation_name
            };
            let animation_year: NaiveDate = match update_animation.animation_year {
                Some(animation_year) => animation_year,
                _ => current_animation.animation_year,
            };
            let director: String = match update_animation.director {
                Some(director) => director,
                _ => current_animation.director,
            };
            let screenwriter: String = match update_animation.screenwriter {
                Some(screenwriter) => screenwriter,
                _ => current_animation.screenwriter,
            };
            let make: String = match update_animation.make {
                Some(make) => make,
                _ => current_animation.make,
            };
            let logo: String = match update_animation.logo {
                Some(logo) => logo,
                _ => current_animation.logo,
            };
            let amount: i16 = match update_animation.amount {
                Some(amount) => amount,
                _ => current_animation.amount,
            };
            let localflag: bool = match update_animation.localflag {
                Some(localflag) => localflag,
                _ => current_animation.localflag,
            };
            let localurl: String = match update_animation.localurl {
                Some(localurl) => localurl,
                _ => current_animation.localurl.unwrap_or_default(),
            };
            let remoteflag: bool = match update_animation.remoteflag {
                Some(remoteflag) => remoteflag,
                _ => current_animation.remoteflag,
            };
            let remoteurl: String = match update_animation.remoteurl {
                Some(remoteurl) => remoteurl,
                _ => current_animation.remoteurl.unwrap_or_default(),
            };
            let container: String = match update_animation.container {
                Some(container) => container,
                _ => current_animation.container,
            };
            let codev: String = match update_animation.codev {
                Some(codev) => codev,
                _ => current_animation.codev,
            };
            let codea: String = match update_animation.codea {
                Some(codea) => codea,
                _ => current_animation.codea,
            };
            let subtype: String = match update_animation.subtype {
                Some(subtype) => subtype,
                _ => current_animation.subtype,
            };
            let subteam: String = match update_animation.subteam {
                Some(subteam) => subteam,
                _ => current_animation.subteam.unwrap_or_default(),
            };
            let lastwatch: NaiveDate = match update_animation.lastwatch {
                Some(lastwatch) => lastwatch,
                _ => current_animation.lastwatch,
            };
            let updatetime: NaiveDate = {
                let fmt = "%Y-%m-%d";
                let now = format!("{}", Local::now().format(fmt));
                NaiveDate::parse_from_str(&now, "%Y-%m-%d").unwrap()
            };
            let remark: String = match update_animation.remark {
                Some(remark) => remark,
                _ => current_animation.remark.unwrap_or_default(),
            };

            // 修改
            let animation_row = sqlx::query_as!(
                Animation,
                "
                Update Animation set seriesflag = $1, seriesid = $2, animation_name = $3,
                animation_year = $4, director = $5, screenwriter = $6,
                make = $7, logo = $8, amount = $9,
                localflag = $10, localurl = $11,remoteflag = $12, 
                remoteurl = $13, container = $14, codev = $15,
                codea = $16, subtype = $17, subteam = $18,
                lastwatch = $19, updatetime = $20, remark = $21
                Where id = $22
                Returning
                id, seriesFlag, seriesId, animation_name, animation_year, director,
                screenwriter, make, logo, amount, localflag, localurl, remoteflag,
                remoteurl, container, codev, codea, subtype, subteam, lastwatch,
                updatetime, remark
                ", seriesflag, seriesid, animation_name,
                animation_year, director, screenwriter,
                make, logo, amount,
                localflag, localurl, remoteflag,
                remoteurl, container, codev,
                codea, subtype, subteam,
                lastwatch, updatetime, remark, update_animation.id,
            )
            .fetch_one(pool)
            .await;

            // 判断是否修改成功
            match animation_row {
                Ok(animation_row) => {
                    print_log(format!("Update Animation of id:{}, new name:[{}]", update_animation.id, animation_name));
                    Ok(animation_row)
                },
                Err(_animation_row) => Err(SEVXError::DBError("Update Failed".into())),
            }
        }

        // 认证失败
        Err(_) => Err(SEVXError::AuthFailed(format!("Auth Failed of name:[{}] for update Animation", auth.uname)))
    }
}


/**
 * 删除 Animation DB
 */
pub async fn delete_animation_db (
    pool: &PgPool,
    delete_animation: DeleteAnimation,
) -> Result<String, SEVXError> {
    // 先进行身份验证
    let auth = get_auth_db(&pool, delete_animation.name.clone(), delete_animation.password.clone()).await;
    match auth {
        Ok(_) => {
            // 判断当前 Animation 是否存在
            let current_row = sqlx::query_as!(
                Animation,
                "Select * From Animation where id = $1",
                delete_animation.id
            )
            .fetch_optional(pool).await?;
            match current_row {

                // 存在则执行删除
                Some(_current_row) => {
                    let _row = sqlx::query!(
                        "Delete From Animation where id = $1",
                        delete_animation.id
                    )
                    .execute(pool)
                    .await?;
                    print_log(format!("Delete Animation of id:{}", delete_animation.id));
                    Ok(format!("Delete Animation of id:{}", delete_animation.id))
                },

                // 不存在返回错误
                _ => Err(SEVXError::NotFound(format!("Animation of id:{} is not found", delete_animation.id)))
            }
        }

        Err(_) => Err(SEVXError::AuthFailed(format!("Auth Failed of name:[{}] for delete Animation", delete_animation.name)))
    }
}
