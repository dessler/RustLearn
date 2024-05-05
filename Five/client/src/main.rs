use reqwest::{Client, Error};
use std::time::Duration;
use tokio::time;
use rand::{self, Rng};
use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志系统
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .init();

    // 初始化 HTTP 客户端
    let client = Client::new();

    loop {
        // 查询对方接口
        let (should_report, response_text) = match check_partner_api(&client).await {
            Ok(result) => result,
            Err(err) => {
                error!("Failed to check partner API: {}", err);
                return Ok(()); // 立即返回 Ok(())，继续下一次循环
            }
        };

        // 如果对方接口返回 true，则发送报告
        if should_report {
            // 生成报告内容
            let report_content = format!("Report: {}", response_text);

            // 发送报告
            if let Err(err) = report(&client, report_content).await {
                error!("签到失败: {}", err);
            } else {
                info!("签到成功.");
            }
        } else {
            info!("没有人点名，不需要签到.");
        }

        // 随机等待 10 到 60 秒
        let sleep_time = rand_sleep_time();
        time::sleep(sleep_time).await;
    }
}

// 查询对方接口
async fn check_partner_api(client: &Client) -> Result<(bool, String), Error> {
    let response = client.get("http://localhost:5001/b1").send().await?;
    let response_text = response.text().await?;

    let should_report = response_text.trim() == "true";

    Ok((should_report, response_text))
}

// 签到
async fn report(client: &Client, report_content: String) -> Result<(), Error> {
    let response = client
        .post("http://localhost:5001/b2")
        .body(report_content)
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                // 请求成功
                return Ok(());
            } else {
                println!("无法连接点名地址");
                return Ok(());
            }
        }
        Err(err) => {
            return Err(err.into());
        }
    }
}

// 生成随机等待时间
fn rand_sleep_time() -> Duration {
    let mut rng = rand::thread_rng();
    let seconds = rng.gen_range(10..=60);
    Duration::from_secs(seconds)
}