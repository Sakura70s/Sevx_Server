use crate::models::animation_model::*;
use sqlx::postgres::PgPool;
use crate::error::SEVXError;
use chrono::NaiveDate;
use crate::log::print_log;

/**
 * 获取所有动漫列表
 * 成功应返回 Vec<Animation>，错误应返回SEVXError
 */
pub async fn get_all_animation_db(pool: &PgPool) -> Result<Vec<Animation>, SEVXError>{
    let rows = sqlx::query!(
        "Select * from Animation"
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

    match animation_vec.len() {
        0 => Err(SEVXError::NotFound("Not found any Animation".into())),
        _ => {
            print_log("Get Animation list".to_string());
            Ok(animation_vec)
        },
    }
}

/**
 * 根据单一 ID 获取具体 动漫
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
 * 添加动漫
 */
pub async fn add_animation_db (
    pool: &PgPool,
    add_animation: AddAnimation,
) -> Result<Animation, SEVXError> {
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

    print_log(format!("Add Animation of name:{}", add_animation.animation_name));
    Ok(row)
}