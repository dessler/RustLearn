use actix_web::{web, App, HttpServer, middleware, Responder, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::io;
use env_logger::Env;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    name: String,
    time: f64,
}

#[derive(Debug)]
struct NotFoundError;

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Not Found")
    }
}

impl ResponseError for NotFoundError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::NotFound().body("Not Found")
    }
}

async fn handle_not_found() -> Result<&'static str, NotFoundError> {
    Err(NotFoundError)
}

async fn handle_a1_request(data: web::Json<Data>) -> impl Responder {
    // 发送请求给B服务的b1接口
    let client = Client::new();
    match client.post("http://localhost:5001/b1").json(&data).send().await {
        Ok(_) => HttpResponse::Ok().body("Request sent to B1"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error sending request to B1: {}", e)),
    }
}

async fn handle_a2_request() -> impl Responder {
    // 查询B服务的b4接口的结果，并返回
    let client = Client::new();
    match client.get("http://localhost:5001/b4").send().await {
        Ok(res) => {
            if res.status().is_success() {
                let body = res.text().await.unwrap_or_else(|_| String::from("Error reading response"));
                HttpResponse::Ok().body(body)
            } else {
                HttpResponse::NotFound().body("No data available from B4 service")
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Unable to connect to B4 service"),
    }
}



#[actix_web::main]
async fn main() -> io::Result<()> {
    // 初始化日志记录器
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/a1", web::post().to(handle_a1_request))
            .route("/a2", web::get().to(handle_a2_request))
            .default_service(web::route().to(handle_not_found))
    })
        .bind("127.0.0.1:5000")?
        .run()
        .await
}

