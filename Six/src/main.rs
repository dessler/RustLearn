
use chrono::{Local, DateTime};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use tokio::time::{sleep, Duration};

// 获取当前时间并格式化
fn get_current_time_formatted() -> String {
    let now: DateTime<Local> = Local::now();
    format!("{:?}", now)
}

// 生成随机名字
fn generate_random_name(rng: &mut impl Rng) -> String {
    let name_length = rng.gen_range(5..=10);
    (0..name_length)
        .map(|_| rng.sample(Alphanumeric))
        .map(char::from)
        .collect()
}

// 随机等待10-60秒
async fn async_rand_sleep_time() -> Duration {
    let mut rng = thread_rng();
    let seconds = rng.gen_range(10..=60);
    Duration::from_secs(seconds)
}

// 异步初始化日志
async fn initialize_logger() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
}

#[tokio::main]
async fn main() {
    // 初始化日志记录器
    initialize_logger().await;

    // 调用初始等待函数，等待10秒
    info!("等待 10 秒钟开始执行程序...");
    sleep(Duration::from_secs(10)).await;

    // 死循环持续调用随机名字和当前时间
    loop {
        let mut rng = thread_rng();
        let random_name = generate_random_name(&mut rng);
        let formatted_time = get_current_time_formatted();

        info!("生成的名字: {}，当前时间: {}", random_name, formatted_time);

        let sleep_duration = async_rand_sleep_time().await;
        info!("随机等待时间: {:?}", sleep_duration);
        sleep(sleep_duration).await;
    }    
}

// 知识点
// 异步编程（async/await）：使用 async 和 await 关键字定义异步函数和等待异步操作完成。在 tokio::main 宏中运行异步任务。
// 时间处理（chrono）：使用 chrono 库获取当前本地时间，并进行格式化。
// 随机数生成（rand）：使用 rand 库生成随机数，包括生成随机名字的长度和随机等待时间。
// 日志记录（log + simple_logger）：使用 log 库输出日志，并通过 simple_logger 初始化日志记录器，设定日志级别为 Info。
// 线程睡眠（tokio::time::sleep）：使用 tokio::time::sleep 来实现异步等待，以模拟随机等待时间。
// 循环执行：使用 loop 关键字创建一个无限循环，持续生成随机名字和当前时间，并输出日志，直到程序手动中止。