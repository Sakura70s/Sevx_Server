use crate::models::film_model::*;
use sqlx::postgres::PgPool;
use crate::error::SEVXError;
use chrono::{NaiveDate, Local};
use crate::log::print_log;


/**
 * 获取所有Film列表
 * 成功应返回 Vec<Film>，错误应返回SEVXError
 */
pub async fn get_all_film_db(pool: &PgPool) -> Result<Vec<Film>, SEVXError>{
    let rows = sqlx::query!(
        "Select * from Film"
    )
    .fetch_all(pool)
    .await?;

    let film_vec: Vec<Film> = rows.iter()
    .map(|item| Film{
        id: item.id,
        seriesflag: item.seriesflag,
        seriesid: item.seriesid,
        film_name: item.film_name.clone(),
        film_year: item.film_year,
        director: item.director.clone(),
        screenwriter: item.screenwriter.clone(),
        make: item.make.clone(),
        logo: item.logo.clone(),
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

    match film_vec.len() {
        0 => Err(SEVXError::NotFound("Not found any Film".into())),
        _ => {
            print_log("Get Film list".to_string());
            Ok(film_vec)
        },
    }
}


/**
 * 根据单一 ID 获取具体 Film
 * 成功返回 Film，失败还不清楚
 */
pub async fn get_film_for_id_db (
    pool: &PgPool,
    id: i32,
) -> Result<Film, SEVXError> {
    let row = sqlx::query_as!(
        Film,
        "Select * from Film where id = $1", id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        // Success
        Some(row) => {
            print_log(format!("Get Film of id:{}, name:[{}]", id, row.film_name));
            Ok(row)
        }
        // Error
        _ => Err(SEVXError::NotFound(format!("Animaton of id = {} is not found!", id)))
    }
}


/**
 * 根据名称查询
 */
pub async fn search_film_for_name_db (
    pool: &PgPool,
    name: String,
) -> Result<Vec<Film>, SEVXError> {
    let rows = sqlx::query!("Select * from Film where film_name = $1", name)
    .fetch_all(pool)
    .await?;

    let film_vec: Vec<Film> = rows.iter()
    .map(|item| Film{
        id: item.id,
        seriesflag: item.seriesflag,
        seriesid: item.seriesid,
        film_name: item.film_name.clone(),
        film_year: item.film_year,
        director: item.director.clone(),
        screenwriter: item.screenwriter.clone(),
        make: item.make.clone(),
        logo: item.logo.clone(),
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

    match film_vec.len() {
        0 => Err(SEVXError::NotFound(format!("Not found any Film of name:[{}]", name))),
        _ => {
            print_log(format!("Get Film list of name:[{}]", name));
            Ok(film_vec)
        },
    }
}


/**
 * 添加Film
 */
pub async fn add_film_db (
    pool: &PgPool,
    add_film: AddFilm,
) -> Result<Film, SEVXError> {
    let row = sqlx::query_as!(
        Film,
        "Insert into Film (
            seriesFlag,
            seriesId,
            Film_name,
            Film_year,
            director,
            screenWriter,
            make,
            logo,
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
            $11, $12, $13, $14, $15, $16, $17, $18
        ) Returning
        id,
        seriesFlag,
        seriesId,
        film_name,
        film_year,
        director,
        screenwriter,
        make,
        logo,
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
        add_film.seriesflag,
        add_film.seriesid,
        add_film.film_name,
        add_film.film_year,
        add_film.director,
        add_film.screenwriter,
        add_film.make,
        add_film.logo,
        add_film.localflag,
        add_film.localurl,
        add_film.remoteflag,
        add_film.remoteurl,
        add_film.container,
        add_film.codev,
        add_film.codea,
        add_film.subtype,
        add_film.subteam,
        add_film.remark
    )
    .fetch_one(pool)
    .await?;

    // 成功之后打印 Log， 返回新增加的
    print_log(format!("Add Film of name:[{}]", add_film.film_name));
    Ok(row)
}


/**
 * 更新Film
 */
pub async fn update_film_db (
    pool: &PgPool,
    update_film: UpdateFilm,
) -> Result<Film, SEVXError> {

    // 判断要更新课程是否存在
    let current_film = sqlx::query_as!(
        Film,
        "Select * from Film where id = $1", update_film.id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| SEVXError::NotFound(format!("Film of id:{} is not found", update_film.id)))?;

    // 存在则继续

    // 配置将要更新的变量
    let seriesflag: bool = match update_film.seriesflag {
        Some(seriesflag) => seriesflag,
        _ => current_film.seriesflag,
    };
    let seriesid: i16 = match update_film.seriesid {
        Some(seriesid) => seriesid,
        _ => current_film.seriesid,
    };
    let film_name: String = match update_film.film_name {
        Some(film_name) => film_name,
        _ => current_film.film_name
    };
    let film_year: NaiveDate = match update_film.film_year {
        Some(film_year) => film_year,
        _ => current_film.film_year,
    };
    let director: String = match update_film.director {
        Some(director) => director,
        _ => current_film.director,
    };
    let screenwriter: String = match update_film.screenwriter {
        Some(screenwriter) => screenwriter,
        _ => current_film.screenwriter,
    };
    let make: String = match update_film.make {
        Some(make) => make,
        _ => current_film.make,
    };
    let logo: String = match update_film.logo {
        Some(logo) => logo,
        _ => current_film.logo,
    };
    let localflag: bool = match update_film.localflag {
        Some(localflag) => localflag,
        _ => current_film.localflag,
    };
    let localurl: String = match update_film.localurl {
        Some(localurl) => localurl,
        _ => current_film.localurl.unwrap_or_default(),
    };
    let remoteflag: bool = match update_film.remoteflag {
        Some(remoteflag) => remoteflag,
        _ => current_film.remoteflag,
    };
    let remoteurl: String = match update_film.remoteurl {
        Some(remoteurl) => remoteurl,
        _ => current_film.remoteurl.unwrap_or_default(),
    };
    let container: String = match update_film.container {
        Some(container) => container,
        _ => current_film.container,
    };
    let codev: String = match update_film.codev {
        Some(codev) => codev,
        _ => current_film.codev,
    };
    let codea: String = match update_film.codea {
        Some(codea) => codea,
        _ => current_film.codea,
    };
    let subtype: String = match update_film.subtype {
        Some(subtype) => subtype,
        _ => current_film.subtype,
    };
    let subteam: String = match update_film.subteam {
        Some(subteam) => subteam,
        _ => current_film.subteam.unwrap_or_default(),
    };
    let lastwatch: NaiveDate = match update_film.lastwatch {
        Some(lastwatch) => lastwatch,
        _ => current_film.lastwatch,
    };
    let updatetime: NaiveDate = {
        let fmt = "%Y-%m-%d";
        let now = format!("{}", Local::now().format(fmt));
        NaiveDate::parse_from_str(&now, "%Y-%m-%d").unwrap()
    };
    let remark: String = match update_film.remark {
        Some(remark) => remark,
        _ => current_film.remark.unwrap_or_default(),
    };

    // 修改
    let film_row = sqlx::query_as!(
        Film,
        "
        Update Film set seriesflag = $1, seriesid = $2, film_name = $3,
        film_year = $4, director = $5, screenwriter = $6,
        make = $7, logo = $8,
        localflag = $9, localurl = $10,remoteflag = $11, 
        remoteurl = $12, container = $13, codev = $14,
        codea = $15, subtype = $16, subteam = $17,
        lastwatch = $18, updatetime = $19, remark = $20
        Where id = $21
        Returning
        id, seriesFlag, seriesId, film_name, film_year, director,
        screenwriter, make, logo, localflag, localurl, remoteflag,
        remoteurl, container, codev, codea, subtype, subteam, lastwatch,
        updatetime, remark
        ", seriesflag, seriesid, film_name,
        film_year, director, screenwriter,
        make, logo,
        localflag, localurl, remoteflag,
        remoteurl, container, codev,
        codea, subtype, subteam,
        lastwatch, updatetime, remark, update_film.id,
    )
    .fetch_one(pool)
    .await;

    // 判断是否修改成功
    match film_row {
        Ok(film_row) => {
            print_log(format!("Update Film of id:{}, name:[{}]", update_film.id, film_name));
            Ok(film_row)
        },
        Err(_film_row) => Err(SEVXError::DBError("Update Failed".into())),
    }
}


/**
 * 删除 Film DB
 */
pub async fn delete_film_db (
    pool: &PgPool,
    delete_film: DeleteFilm,
) -> Result<String, SEVXError> {

    // 首先判断口令是否正确
    if delete_film.password.eq("114514") {
        
        // 判断当前 Film 是否存在
        let current_row = sqlx::query_as!(
            Film,
            "Select * From Film where id = $1",
            delete_film.id
        )
        .fetch_optional(pool).await?;
        match current_row {

            // 存在则执行删除
            Some(_current_row) => {
                let _row = sqlx::query!(
                    "Delete From film where id = $1",
                    delete_film.id
                )
                .execute(pool)
                .await?;
                print_log(format!("Delete Film of id:{}", delete_film.id));
                Ok(format!("Delete Film of id:{}", delete_film.id))
            },

            // 不存在返回错误
            _ => Err(SEVXError::NotFound(format!("Film of id:{} is not found", delete_film.id)))
        }
    
    // 口令不正确提示认证错误
    } else {
        Err(SEVXError::AuthFailed("Password Error".into()))
    }
}
