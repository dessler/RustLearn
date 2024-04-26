// main.rs
use actix_web::{web, App, HttpServer};
use crate::db::DbPool;
mod handlers;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化数据库
    let db_pool = db::init_db().expect("Failed to initialize database");

    // 启动 HTTP 服务器并绑定路由
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone())) // 将数据库连接池传递给应用程序状态
            // 注册路由
            .route("/register", web::post().to(handlers::register_user))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}