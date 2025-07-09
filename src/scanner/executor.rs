use std::net::IpAddr;
use async_std::io as async_io;
use log::debug;
use rand::Rng; // 需要在 Cargo.toml 中添加 rand = "0.8"

pub struct AsyncScanExecutor {
    ip: IpAddr,
    port_list: Vec<u16>,
    res: Vec<u16>, // 用于存储扫描结果
}

impl AsyncScanExecutor {
    pub fn new(ip: IpAddr, start: u16, end: u16) -> Self {
        // 不用做参数检查，因为在创建 AsyncScannerEngine 时已经确保了 IP 地址和端口范围的合法性
        let port_list = (start..=end).collect();
        AsyncScanExecutor { ip, port_list, res: Vec::new() }
    }

    pub fn get_results(&self) -> Vec<u16> {
        // 返回扫描结果
        self.res.clone()
    }


    pub async fn execute(&mut self, dura: u64) {
        // 异步执行端口扫描逻辑
        for &port in &self.port_list {
            match self.ping(port, dura).await {
                Ok(_) => {
                    // println!("{}:{} is open", self.ip, port);
                    self.res.push(port); // 将结果存储到 res 中
                }
                Err(e) => {
                    match e.kind() {
                        std::io::ErrorKind::TimedOut => {
                            // debug!("Timeout for {}:{}", self.ip, port); // 可以使用 debug 级别记录超时
                        }
                        std::io::ErrorKind::ConnectionRefused => {
                            // println!("{}:{} connection refused", self.ip, port); // 端口关闭或防火墙拒绝
                        }
                        _ => {
                            println!("error:{} ,{}:{} cannot open", e, self.ip, port);
                        }
                    }
                }
            }
            // !!! 在这里添加随机延迟 !!!
            let mut rng = rand::thread_rng();
            let delay_ms = rng.gen_range(10..=100); // 例如，随机延迟 10 到 100 毫秒
            async_std::task::sleep(std::time::Duration::from_millis(delay_ms)).await;
        }
    }

    async fn ping(&self, port: u16, dura: u64) -> Result<(), std::io::Error> {
        // 异步 TCP 连接逻辑
        let socket = std::net::SocketAddr::new(self.ip, port);
        let timeout = std::time::Duration::from_millis(dura);
        let stream = async_io::timeout(timeout, async_std::net::TcpStream::connect(socket)).await?;
        // 关闭连接(两端的连接，即发送和接收都关闭),会有开销
        stream.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }
}