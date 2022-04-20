use crate::models::animation_model::*;
use sqlx::postgres::PgPool;
use crate::error::SEVXError;
use chrono::NaiveDate;
use crate::log::print_log;

/**
 * 获取所有动漫列表
 * 成功应返回 Vec，错误应返回SEVXError
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
        series_flag: item.seriesflag,
        series_id: item.seriesid,
        animation_name: item.animation_name.clone(),
        animation_year: item.animation_year,
        director: item.director.clone(),
        screen_writer: item.screenwriter.clone(),
        make: item.make.clone(),
        logo: item.logo.clone(),
        amount: item.amount,
        local_flag: item.localflag,
        local_url: item.localurl.clone(),
        remote_flag: item.remoteflag,
        remote_url: item.remoteurl.clone(),
        container: item.container.clone(),
        codev: item.codev.clone(),
        codea: item.codea.clone(),
        sub_type: item.subtype.clone(),
        sub_team: item.subteam.clone(),
        last_watch: item.lastwatch,
        update_time: NaiveDate::from(item.updatetime),
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