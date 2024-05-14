use std::path::Path;
use tokio::net::UnixStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use serde_json::{Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Docker Unix 套接字路径
    let docker_socket_path = "/var/run/docker.sock";

    // 要查询的容器 ID
    let container_id = "YOUR_CONTAINER_ID";

    // 使用 Unix 套接字路径构建 Unix 套接字客户端
    let mut stream = UnixStream::connect(Path::new(docker_socket_path)).await?;

    // 构建请求消息
    let request = format!("GET /containers/{}/json HTTP/1.0\r\n\r\n", container_id);

    // 发送请求消息到 Docker Unix 套接字
    stream.write_all(request.as_bytes()).await?;

    // 读取 Docker 守护进程的响应
    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    // 解析 JSON 响应
    let data: Value = serde_json::from_str(&response)?;

    // 获取容器启动时的命令
    let run_command = data["Config"]["Cmd"].clone();
    let run_command_str = serde_json::to_string(&run_command)?;

    println!("Docker run command for container {}: {}", container_id, run_command_str);

    Ok(())
}
