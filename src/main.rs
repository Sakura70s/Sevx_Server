use actix_web::{web, App, HttpServer};
use std::io;
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;
use state::AppState;

// 指定模块路径
#[path = "./handlers/mod.rs"]
mod handlers;
#[path = "./routers/mod.rs"]
mod routers;
#[path = "./state.rs"]
mod state;
#[path = "./models/mod.rs"]
mod models;
#[path = "./db_access/mod.rs"]
mod db_access;
#[path = "./error.rs"]
mod error;
#[path = "./log.rs"]
mod log;

// 定义为 Actix 的入口
#[actix_rt::main]
async fn main() -> io::Result<()> {

    // 读取环境变量
    dotenv().ok();

    // 设置连接字符串
    let database_url = env::var("DATABASE_URL").expect("Not Found");

    // 创建连接池
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    // 实例化Data
    let shared_data = web::Data::new(
        AppState {
            db: db_pool,
        }
    );

    // 构建App
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(routers::animation_router::animation_routers)
            .configure(routers::film_router::film_routers)
            .configure(routers::tv_router::tv_routers)
            .configure(routers::sv_router::sv_routers)
            .configure(routers::music_router::music_routers)
            .configure(routers::novel_router::novel_routers)
            .configure(routers::comic_router::comic_routers)
            .configure(routers::auth_router::auth_routers)
    };

    // 设置监听端口
    let addr = String::from("localhost:3000");
    println!("年少不知萝莉好，\n只把及笄当块宝。\n如猫般柔软可爱，\n如恶魔狡猾诱人，\n精灵般美丽自然，
                \n天使般纯真圣洁，\n所以吾之故求也；\n\n吾之独醉萝莉之美，\n豆蔻年华吾之所求。\n\n");
    println!("当前服务器正在监听以下端口：{}", addr);

    // 程序入口（开始监听）
    HttpServer::new(app).bind(addr)?.run().await
}
