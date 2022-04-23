use crate::models::tv_model::*;
use sqlx::postgres::PgPool;
use crate::error::SEVXError;
use chrono::{NaiveDate, Local};
use crate::log::print_log;


/**
 * 获取所有 Tv 列表
 * 成功应返回 Vec<Tv>，错误应返回SEVXError
 */
pub async fn get_all_tv_db(pool: &PgPool) -> Result<Vec<Tv>, SEVXError>{
    let rows = sqlx::query!(
        "Select * from Tv"
    )
    .fetch_all(pool)
    .await?;

    let tv_vec: Vec<Tv> = rows.iter()
    .map(|item| Tv{
        id: item.id,
        seriesflag: item.seriesflag,
        seriesid: item.seriesid,
        tv_name: item.tv_name.clone(),
        tv_year: item.tv_year,
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

    match tv_vec.len() {
        0 => Err(SEVXError::NotFound("Not found any Tv".into())),
        _ => {
            print_log("Get Tv list".to_string());
            Ok(tv_vec)
        },
    }
}


/**
 * 根据单一 ID 获取具体 Tv
 * 成功返回 Tv，失败还不清楚
 */
pub async fn get_tv_for_id_db (
    pool: &PgPool,
    id: i32,
) -> Result<Tv, SEVXError> {
    let row = sqlx::query_as!(
        Tv,
        "Select * from Tv where id = $1", id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        // Success
        Some(row) => {
            print_log(format!("Get Tv of id:{}, name:[{}]", id, row.tv_name));
            Ok(row)
        }
        // Error
        _ => Err(SEVXError::NotFound(format!("Animaton of id = {} is not found!", id)))
    }
}


/**
 * 根据名称查询
 */
pub async fn search_tv_for_name_db (
    pool: &PgPool,
    name: String,
) -> Result<Vec<Tv>, SEVXError> {
    let rows = sqlx::query!("Select * from Tv where tv_name = $1", name)
    .fetch_all(pool)
    .await?;

    let tv_vec: Vec<Tv> = rows.iter()
    .map(|item| Tv{
        id: item.id,
        seriesflag: item.seriesflag,
        seriesid: item.seriesid,
        tv_name: item.tv_name.clone(),
        tv_year: item.tv_year,
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

    match tv_vec.len() {
        0 => Err(SEVXError::NotFound(format!("Not found any Tv of name:[{}]", name))),
        _ => {
            print_log(format!("Get Tv list of name:[{}]", name));
            Ok(tv_vec)
        },
    }
}


/**
 * 添加 Tv
 */
pub async fn add_tv_db (
    pool: &PgPool,
    add_tv: AddTv,
) -> Result<Tv, SEVXError> {
    let row = sqlx::query_as!(
        Tv,
        "Insert into Tv (
            seriesFlag,
            seriesId,
            Tv_name,
            Tv_year,
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
        tv_name,
        tv_year,
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
        add_tv.seriesflag,
        add_tv.seriesid,
        add_tv.tv_name,
        add_tv.tv_year,
        add_tv.director,
        add_tv.screenwriter,
        add_tv.make,
        add_tv.logo,
        add_tv.amount,
        add_tv.localflag,
        add_tv.localurl,
        add_tv.remoteflag,
        add_tv.remoteurl,
        add_tv.container,
        add_tv.codev,
        add_tv.codea,
        add_tv.subtype,
        add_tv.subteam,
        add_tv.remark
    )
    .fetch_one(pool)
    .await?;

    // 成功之后打印 Log， 返回新增加的
    print_log(format!("Add Tv of name:[{}]", add_tv.tv_name));
    Ok(row)
}


/**
 * 更新 Tv
 */
pub async fn update_tv_db (
    pool: &PgPool,
    update_tv: UpdateTv,
) -> Result<Tv, SEVXError> {

    // 判断要更新课程是否存在
    let current_tv = sqlx::query_as!(
        Tv,
        "Select * from Tv where id = $1", update_tv.id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| SEVXError::NotFound(format!("Tv of id:{} is not found", update_tv.id)))?;

    // 存在则继续

    // 配置将要更新的变量
    let seriesflag: bool = match update_tv.seriesflag {
        Some(seriesflag) => seriesflag,
        _ => current_tv.seriesflag,
    };
    let seriesid: i16 = match update_tv.seriesid {
        Some(seriesid) => seriesid,
        _ => current_tv.seriesid,
    };
    let tv_name: String = match update_tv.tv_name {
        Some(tv_name) => tv_name,
        _ => current_tv.tv_name
    };
    let tv_year: NaiveDate = match update_tv.tv_year {
        Some(tv_year) => tv_year,
        _ => current_tv.tv_year,
    };
    let director: String = match update_tv.director {
        Some(director) => director,
        _ => current_tv.director,
    };
    let screenwriter: String = match update_tv.screenwriter {
        Some(screenwriter) => screenwriter,
        _ => current_tv.screenwriter,
    };
    let make: String = match update_tv.make {
        Some(make) => make,
        _ => current_tv.make,
    };
    let logo: String = match update_tv.logo {
        Some(logo) => logo,
        _ => current_tv.logo,
    };
    let amount: i16 = match update_tv.amount {
        Some(amount) => amount,
        _ => current_tv.amount,
    };
    let localflag: bool = match update_tv.localflag {
        Some(localflag) => localflag,
        _ => current_tv.localflag,
    };
    let localurl: String = match update_tv.localurl {
        Some(localurl) => localurl,
        _ => current_tv.localurl.unwrap_or_default(),
    };
    let remoteflag: bool = match update_tv.remoteflag {
        Some(remoteflag) => remoteflag,
        _ => current_tv.remoteflag,
    };
    let remoteurl: String = match update_tv.remoteurl {
        Some(remoteurl) => remoteurl,
        _ => current_tv.remoteurl.unwrap_or_default(),
    };
    let container: String = match update_tv.container {
        Some(container) => container,
        _ => current_tv.container,
    };
    let codev: String = match update_tv.codev {
        Some(codev) => codev,
        _ => current_tv.codev,
    };
    let codea: String = match update_tv.codea {
        Some(codea) => codea,
        _ => current_tv.codea,
    };
    let subtype: String = match update_tv.subtype {
        Some(subtype) => subtype,
        _ => current_tv.subtype,
    };
    let subteam: String = match update_tv.subteam {
        Some(subteam) => subteam,
        _ => current_tv.subteam.unwrap_or_default(),
    };
    let lastwatch: NaiveDate = match update_tv.lastwatch {
        Some(lastwatch) => lastwatch,
        _ => current_tv.lastwatch,
    };
    let updatetime: NaiveDate = {
        let fmt = "%Y-%m-%d";
        let now = format!("{}", Local::now().format(fmt));
        NaiveDate::parse_from_str(&now, "%Y-%m-%d").unwrap()
    };
    let remark: String = match update_tv.remark {
        Some(remark) => remark,
        _ => current_tv.remark.unwrap_or_default(),
    };

    // 修改
    let tv_row = sqlx::query_as!(
        Tv,
        "
        Update Tv set seriesflag = $1, seriesid = $2, tv_name = $3,
        tv_year = $4, director = $5, screenwriter = $6,
        make = $7, logo = $8, amount = $9,
        localflag = $10, localurl = $11,remoteflag = $12, 
        remoteurl = $13, container = $14, codev = $15,
        codea = $16, subtype = $17, subteam = $18,
        lastwatch = $19, updatetime = $20, remark = $21
        Where id = $22
        Returning
        id, seriesFlag, seriesId, tv_name, tv_year, director,
        screenwriter, make, logo, amount, localflag, localurl, remoteflag,
        remoteurl, container, codev, codea, subtype, subteam, lastwatch,
        updatetime, remark
        ", seriesflag, seriesid, tv_name,
        tv_year, director, screenwriter,
        make, logo, amount,
        localflag, localurl, remoteflag,
        remoteurl, container, codev,
        codea, subtype, subteam,
        lastwatch, updatetime, remark, update_tv.id,
    )
    .fetch_one(pool)
    .await;

    // 判断是否修改成功
    match tv_row {
        Ok(tv_row) => {
            print_log(format!("Update Tv of id:{}", update_tv.id));
            Ok(tv_row)
        },
        Err(_tv_row) => Err(SEVXError::DBError("Update Failed".into())),
    }
}


/**
 * 删除 Tv DB
 */
pub async fn delete_tv_db (
    pool: &PgPool,
    delete_tv: DeleteTv,
) -> Result<String, SEVXError> {

    // 首先判断口令是否正确
    if delete_tv.password.eq("114514") {
        
        // 判断当前 Tv 是否存在
        let current_row = sqlx::query_as!(
            Tv,
            "Select * From Tv where id = $1",
            delete_tv.id
        )
        .fetch_optional(pool).await?;
        match current_row {

            // 存在则执行删除
            Some(_current_row) => {
                let _row = sqlx::query!(
                    "Delete From tv where id = $1",
                    delete_tv.id
                )
                .execute(pool)
                .await?;
                print_log(format!("Delete Tv of id:{}", delete_tv.id));
                Ok(format!("Delete Tv of id:{}", delete_tv.id))
            },

            // 不存在返回错误
            _ => Err(SEVXError::NotFound(format!("Tv of id:{} is not found", delete_tv.id)))
        }
    
    // 口令不正确提示认证错误
    } else {
        Err(SEVXError::AuthFailed("Password Error".into()))
    }
}
