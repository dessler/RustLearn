// handlers.rs

use actix_web::{web, HttpResponse, Responder};
use crate::db;

// 注册处理函数
pub async fn register_user(form: web::Json<(String, String, String)>) -> impl Responder {
    let (username, email, password) = form.into_inner();

    // 检查用户名、邮箱、密码是否为空
    if username.is_empty() || email.is_empty() || password.is_empty() {
        return HttpResponse::BadRequest().body("Username, email, and password are required");
    }

    // 执行用户注册逻辑
    match db::register_user(&username, &email, &password) {
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(_) => HttpResponse::Conflict().body("Username or email already exists"),
    }
}
