use crate::models::sv_model::*;
use sqlx::postgres::PgPool;
use crate::error::SEVXError;
use chrono::{NaiveDate, Local};
use crate::log::print_log;


/**
 * 获取所有 Sv 列表
 * 成功应返回 Vec<Sv>，错误应返回SEVXError
 */
pub async fn get_all_sv_db(pool: &PgPool) -> Result<Vec<Sv>, SEVXError>{
    let rows = sqlx::query!(
        "Select * from Sv Order By id ASC"
    )
    .fetch_all(pool)
    .await?;

    let sv_vec: Vec<Sv> = rows.iter()
    .map(|item| Sv{
        id: item.id,
        sv_name: item.sv_name.clone(),
        sv_year: item.sv_year,
        sv_type: item.sv_type.clone(),
        logo: item.logo.clone(),
        author: item.author.clone(),
        localflag: item.localflag,
        localurl: item.localurl.clone(),
        remoteflag: item.remoteflag,
        remoteurl: item.remoteurl.clone(),
        container: item.container.clone(),
        codev: item.codev.clone(),
        codea: item.codea.clone(),
        updatetime: NaiveDate::from(item.updatetime),
        remark: item.remark.clone(),
    }).collect();

    match sv_vec.len() {
        0 => Err(SEVXError::NotFound("Not found any Sv".into())),
        _ => {
            print_log("Get Sv list".to_string());
            Ok(sv_vec)
        },
    }
}


/**
 * 根据单一 ID 获取具体 Sv
 * 成功返回 Sv，失败还不清楚
 */
pub async fn get_sv_for_id_db (
    pool: &PgPool,
    id: i32,
) -> Result<Sv, SEVXError> {
    let row = sqlx::query_as!(
        Sv,
        "Select * from Sv where id = $1", id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        // Success
        Some(row) => {
            print_log(format!("Get Sv of id:{}, name:[{}]", id, row.sv_name));
            Ok(row)
        }
        // Error
        _ => Err(SEVXError::NotFound(format!("Animaton of id = {} is not found!", id)))
    }
}


/**
 * 根据名称查询
 */
pub async fn search_sv_for_name_db (
    pool: &PgPool,
    name: String,
) -> Result<Vec<Sv>, SEVXError> {
    let rows = sqlx::query!("Select * from Sv where sv_name = $1 Order By id ASC", name)
    .fetch_all(pool)
    .await?;

    let sv_vec: Vec<Sv> = rows.iter()
    .map(|item| Sv{
        id: item.id,
        sv_name: item.sv_name.clone(),
        sv_year: item.sv_year,
        sv_type: item.sv_type.clone(),
        logo: item.logo.clone(),
        author: item.author.clone(),
        localflag: item.localflag,
        localurl: item.localurl.clone(),
        remoteflag: item.remoteflag,
        remoteurl: item.remoteurl.clone(),
        container: item.container.clone(),
        codev: item.codev.clone(),
        codea: item.codea.clone(),
        updatetime: NaiveDate::from(item.updatetime),
        remark: item.remark.clone(),
    }).collect();

    match sv_vec.len() {
        0 => Err(SEVXError::NotFound(format!("Not found any Sv of name:[{}]", name))),
        _ => {
            print_log(format!("Get Sv list of name:[{}]", name));
            Ok(sv_vec)
        },
    }
}


/**
 * 添加 Sv
 */
pub async fn add_sv_db (
    pool: &PgPool,
    add_sv: AddSv,
) -> Result<Sv, SEVXError> {
    let row = sqlx::query_as!(
        Sv,
        "Insert into Sv (
            Sv_name,
            Sv_year,
            Sv_type,
            logo,
            author,
            localFlag,
            localUrl,
            remoteFlag,
            remoteUrl,
            container,
            codev,
            codea,
            remark
        ) Values (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13
        ) Returning
        id,
        sv_name,
        sv_year,
        sv_type,
        logo,
        author,
        localflag,
        localurl,
        remoteflag,
        remoteurl,
        container,
        codev,
        codea,
        updatetime,
        remark",
        add_sv.sv_name,
        add_sv.sv_year,
        add_sv.sv_type,
        add_sv.logo,
        add_sv.author,
        add_sv.localflag,
        add_sv.localurl,
        add_sv.remoteflag,
        add_sv.remoteurl,
        add_sv.container,
        add_sv.codev,
        add_sv.codea,
        add_sv.remark
    )
    .fetch_one(pool)
    .await?;

    // 成功之后打印 Log， 返回新增加的
    print_log(format!("Add Sv of name:[{}]", add_sv.sv_name));
    Ok(row)
}


/**
 * 更新 Sv
 */
pub async fn update_sv_db (
    pool: &PgPool,
    update_sv: UpdateSv,
) -> Result<Sv, SEVXError> {

    // 判断要更新课程是否存在
    let current_sv = sqlx::query_as!(
        Sv,
        "Select * from Sv where id = $1", update_sv.id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| SEVXError::NotFound(format!("Sv of id:{} is not found", update_sv.id)))?;

    // 存在则继续

    // 配置将要更新的变量
    let sv_name: String = match update_sv.sv_name {
        Some(sv_name) => sv_name,
        _ => current_sv.sv_name
    };
    let sv_year: NaiveDate = match update_sv.sv_year {
        Some(sv_year) => sv_year,
        _ => current_sv.sv_year,
    };
    let sv_type: String = match update_sv.sv_type {
        Some(sv_type) => sv_type,
        _ => current_sv.sv_type,
    };
    let logo: String = match update_sv.logo {
        Some(logo) => logo,
        _ => current_sv.logo,
    };
    let author: String = match update_sv.author {
        Some(author) => author,
        _ => current_sv.author,
    };
    let localflag: bool = match update_sv.localflag {
        Some(localflag) => localflag,
        _ => current_sv.localflag,
    };
    let localurl: String = match update_sv.localurl {
        Some(localurl) => localurl,
        _ => current_sv.localurl.unwrap_or_default(),
    };
    let remoteflag: bool = match update_sv.remoteflag {
        Some(remoteflag) => remoteflag,
        _ => current_sv.remoteflag,
    };
    let remoteurl: String = match update_sv.remoteurl {
        Some(remoteurl) => remoteurl,
        _ => current_sv.remoteurl.unwrap_or_default(),
    };
    let container: String = match update_sv.container {
        Some(container) => container,
        _ => current_sv.container,
    };
    let codev: String = match update_sv.codev {
        Some(codev) => codev,
        _ => current_sv.codev,
    };
    let codea: String = match update_sv.codea {
        Some(codea) => codea,
        _ => current_sv.codea,
    };
    let updatetime: NaiveDate = {
        let fmt = "%Y-%m-%d";
        let now = format!("{}", Local::now().format(fmt));
        NaiveDate::parse_from_str(&now, "%Y-%m-%d").unwrap()
    };
    let remark: String = match update_sv.remark {
        Some(remark) => remark,
        _ => current_sv.remark.unwrap_or_default(),
    };

    // 修改
    let sv_row = sqlx::query_as!(
        Sv,
        "
        Update Sv 
        set sv_name = $1,
        sv_year = $2,
        sv_type = $3,
        logo = $4, 
        author = $5,
        localflag = $6,
        localurl = $7,
        remoteflag = $8, 
        remoteurl = $9,
        container = $10,
        codev = $11,
        codea = $12,
        updatetime = $13,
        remark = $14
        Where id = $15
            Returning
            id,
            sv_name,
            sv_year,
            sv_type,
            logo,
            author,
            localflag,
            localurl,
            remoteflag,
            remoteurl,
            container,
            codev,
            codea,
            updatetime,
            remark
        ",
                sv_name,
                sv_year, 
                sv_type,
                logo,
                author,
                localflag,
                localurl,
                remoteflag,
                remoteurl,
                container,
                codev,
                codea,
                updatetime,
                remark,
                update_sv.id,
    )
    .fetch_one(pool)
    .await;

    // 判断是否修改成功
    match sv_row {
        Ok(sv_row) => {
            print_log(format!("Update Sv of id:{}", update_sv.id));
            Ok(sv_row)
        },
        Err(_sv_row) => Err(SEVXError::DBError("Update Failed".into())),
    }
}


/**
 * 删除 Sv DB
 */
pub async fn delete_sv_db (
    pool: &PgPool,
    delete_sv: DeleteSv,
) -> Result<String, SEVXError> {

    // 首先判断口令是否正确
    if delete_sv.password.eq("114514") {
        
        // 判断当前 Sv 是否存在
        let current_row = sqlx::query_as!(
            Sv,
            "Select * From Sv where id = $1",
            delete_sv.id
        )
        .fetch_optional(pool).await?;
        match current_row {

            // 存在则执行删除
            Some(_current_row) => {
                let _row = sqlx::query!(
                    "Delete From sv where id = $1",
                    delete_sv.id
                )
                .execute(pool)
                .await?;
                print_log(format!("Delete Sv of id:{}", delete_sv.id));
                Ok(format!("Delete Sv of id:{}", delete_sv.id))
            },

            // 不存在返回错误
            _ => Err(SEVXError::NotFound(format!("Sv of id:{} is not found", delete_sv.id)))
        }
    
    // 口令不正确提示认证错误
    } else {
        Err(SEVXError::AuthFailed("Password Error".into()))
    }
}
