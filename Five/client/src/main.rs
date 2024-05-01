use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::{Error, Client};
use std::time::{Duration, SystemTime};
use tokio::time;
use log::{error, info};
use env_logger;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志系统
    env_logger::init();

    // 初始化 HTTP 客户端
    let client = Client::new();

    // 设置定时器任务
    loop {
        // 随机生成名字
        let name = generate_random_name();

        // 查询对方接口
        let should_report = check_partner_api(&client).await?;

        // 如果对方接口返回 true，则发送报告
        if should_report {
            // 克隆 report_content
            let report_content = format!("Name: {}, Time: {}", name, get_current_time());
            report(name, report_content.clone(), &client).await;
        }

        // 随机等待 10-60 秒
        let sleep_time = rand_sleep_time();
        time::sleep(sleep_time).await;
    }
}

// 随机生成名字
fn generate_random_name() -> String {
    let names = vec!["Alice", "Bob", "Charlie", "David", "Eve"];
    let mut rng = thread_rng();
    (*names.choose(&mut rng).unwrap()).to_string()
}

// 查询对方接口
async fn check_partner_api(client: &Client) -> Result<bool, Error> {
    // 发送 HTTP GET 请求到对方接口，此处假设对方接口地址为 http://partner-api.com/check
    match client.get("http://partner-api.com/check").send().await {
        Ok(response) => {
            // 解析响应
            let should_report = response.text().await? == "true";
            Ok(should_report)
        }
        Err(err) => {
            // 记录错误日志
            error!("Failed to connect to partner API: {}", err);
            Ok(false) // 返回 false，表示不需要发送报告
        }
    }
}


// 发送报告
async fn report(name: String, report_content: String, client: &Client) {
    // 发送报告，此处假设报告地址为 http://report-api.com/report
    match client.post("http://report-api.com/report")
        .body(report_content.clone()) // 修复：传递 report_content 的克隆值而不是移动它
        .send()
        .await {
            Ok(_) => {
                // 记录信息日志
                info!("Report sent by {}: {}", name, report_content);
            }
            Err(err) => {
                // 记录错误日志
                error!("Failed to send report: {}", err);
            }
        }
}

// 获取当前时间
fn get_current_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

// 生成随机等待时间
fn rand_sleep_time() -> Duration {
    let mut rng = thread_rng();
    let seconds = rng.gen_range(10..=60);
    Duration::from_secs(seconds)
}
