use actix_web::{web, App, HttpServer};
use std::io;
// use std::sync::Mutex;
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;

// 指定模块路径
#[path = "./handlers.rs"]
mod handlers;
#[path = "./routers.rs"]
mod routers;
#[path = "./state.rs"]
mod state;
#[path = "./models.rs"]
mod models;
#[path = "./db_access.rs"]
mod db_access;
#[path = "./error.rs"]
mod error;

// 引入模块
use routers::*;
use state::AppState;

// 定义为 Actix的入口
#[actix_rt::main]
async fn main() -> io::Result<()> {

    // 读取环境变量
    dotenv().ok();

    // 设置连接字符串
    let database_url = env::var("DATABASE_URL").expect("Not Found");

    // 创建连接池
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    // 暂时不知道
    let shared_data = web::Data::new(
        AppState {
            db: db_pool,
        }
    );

    // 构建App
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(course_routes)
    };

    // 程序入口（开始监听）
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
