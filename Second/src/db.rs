// db.rs

use rusqlite::{Connection, Result};

// 初始化数据库，创建用户表
pub fn init_db() -> Result<()> {
    let conn = Connection::open("users.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

// 用户注册
pub fn register_user(username: &str, email: &str, password: &str) -> Result<()> {
    let conn = Connection::open("users.db")?;

    // 检查用户名是否已存在
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE username = ?1")?;
    let count: i64 = stmt.query_row([username], |row| row.get(0))?;
    if count > 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows); // 用户名已存在
    }

    // 检查邮箱是否已存在
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE email = ?1")?;
    let count: i64 = stmt.query_row([email], |row| row.get(0))?;
    if count > 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows); // 邮箱已存在
    }

    // 插入新用户信息
    conn.execute(
        "INSERT INTO users (username, email, password) VALUES (?1, ?2, ?3)",
        &[username, email, password],
    )?;

    Ok(())
}
