use mysql::{Pool, PooledConn};
use mysql::prelude::*;
use std::env;
use dotenv::dotenv;

pub struct DbPool {
    pub pool: Pool,
}

pub fn init_db() -> Result<DbPool, mysql::Error> {
    dotenv().ok();

    // 从环境变量中获取数据库连接信息
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_port = env::var("DB_PORT").expect("DB_PORT must be set");
    let db_user = env::var("DB_USER").expect("DB_USER must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");

    // 创建数据库连接池
    let db_url = format!("mysql://{}:{}@{}:{}/{}", db_user, db_password, db_host, db_port, db_name);
    let pool = Pool::new(&db_url)?;

    // 确保 `users` 表存在
    let mut conn = pool.get_conn()?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            username VARCHAR(50) NOT NULL UNIQUE,
            email VARCHAR(100) NOT NULL UNIQUE,
            password VARCHAR(100) NOT NULL
        )"
    )?;

    Ok(DbPool { pool })
}