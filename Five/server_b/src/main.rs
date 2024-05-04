use actix_web::{web, App, HttpServer, HttpResponse, Error, Result};
use serde::{Deserialize, Serialize};
use std::{io, sync::{Arc, Mutex}, time::Duration};
use chrono::{Utc, DateTime};
use actix_web::error::{ErrorInternalServerError, ErrorForbidden};

// 定义投票数据结构
#[derive(Debug, Deserialize, Serialize)]
struct VoteData {
    name: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    time: DateTime<Utc>,
}

// 定义状态类型
struct State {
    b1_state: bool,
    b3_data: Option<VoteData>,
}

// 实现状态类型的构造方法
impl State {
    fn new() -> Self {
        State { b1_state: false, b3_data: None }
    }
}

// 处理 b1 请求的异步函数
async fn handle_b1_request(state: web::Data<Arc<Mutex<State>>>) -> Result<HttpResponse, Error> {
    let mut state_guard = state.lock().map_err(|_| ErrorInternalServerError("Failed to lock state"))?;

    if state_guard.b1_state {
        return Err(ErrorForbidden("b1 service is already in use"));
    }

    state_guard.b1_state = true;

    let state_clone = Arc::clone(&state);

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(600)).await;
        let mut state_guard = state_clone.lock().unwrap();
        state_guard.b1_state = false;
    });

    Ok(HttpResponse::Ok().body("b1 service state set to true"))
}

// 处理 b2 请求的异步函数
async fn handle_b2_request(data: web::Json<VoteData>, state: web::Data<Arc<Mutex<State>>>) -> Result<HttpResponse, Error> {
    let mut state_guard = state.lock().map_err(|_| ErrorInternalServerError("Failed to lock state"))?;

    if !state_guard.b1_state {
        return Err(ErrorForbidden("b1 service is not available"));
    }

    state_guard.b3_data = Some(data.into_inner());

    Ok(HttpResponse::Ok().body("Vote submitted successfully"))
}

// 处理 b3 请求的异步函数
async fn handle_b3_request(state: web::Data<Arc<Mutex<State>>>) -> Result<HttpResponse, Error> {
    let state_guard = state.lock().map_err(|_| ErrorInternalServerError("Failed to lock state"))?;

    if let Some(data) = &state_guard.b3_data {
        Ok(HttpResponse::Ok().json(data))
    } else {
        Ok(HttpResponse::NotFound().body("No data available in b3 service"))
    }
}

// 处理 a1 请求的异步函数
async fn handle_a1_request() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("This is a1 service"))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let state = Arc::new(Mutex::new(State::new()));

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(state.clone()))
            .route("/b1", web::post().to(handle_b1_request))
            .route("/b2", web::post().to(handle_b2_request))
            .route("/b3", web::get().to(handle_b3_request))
            .route("/a1", web::get().to(handle_a1_request))
    })
    .bind("127.0.0.1:5001")?
    .run()
    .await
}
