// handlers.rs
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::Pool;
use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Deserialize)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register_user(
    pool: web::Data<DbPool>,  // 获取数据库连接池
    info: web::Json<RegisterInfo>, // 从请求体中获取注册信息
) -> HttpResponse {
    let mut conn = pool.pool.get_conn().unwrap();

    // 检查用户名是否已存在
    let count_username: i64 = conn.query_first(
        "SELECT COUNT(*) FROM users WHERE username = ?",
        (info.username.as_str(),),
    ).unwrap_or(None).unwrap_or(0);

    if count_username > 0 {
        return HttpResponse::Conflict().body("Username already exists");
    }

    // 检查邮箱是否已存在
    let count_email: i64 = conn.query_first(
        "SELECT COUNT(*) FROM users WHERE email = ?",
        (info.email.as_str(),),
    ).unwrap_or(None).unwrap_or(0);

    if count_email > 0 {
        return HttpResponse::Conflict().body("Email already exists");
    }

    // 将用户数据插入数据库
    let _result = conn.exec_drop(
        "INSERT INTO users (username, email, password) VALUES (?, ?, ?)",
        (info.username.as_str(), info.email.as_str(), info.password.as_str()),
    );

    HttpResponse::Created().body("User registered successfully")
}