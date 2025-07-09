use std::net::IpAddr;
use async_std::io as async_io;

pub struct AsyncScanExecutor {
    ip: IpAddr,
    port_list: Vec<u16>,
}

impl AsyncScanExecutor {
    pub fn new(ip: IpAddr, start: u16, end: u16) -> Self {
        // 不用做参数检查，因为在创建 AsyncScannerEngine 时已经确保了 IP 地址和端口范围的合法性
        let port_list = (start..=end).collect();
        AsyncScanExecutor { ip, port_list }
    }


    pub async fn execute(&self, dura: u64) {
        // 异步执行端口扫描逻辑
        for &port in &self.port_list {
            match self.ping(port, dura).await {
                Ok(_) => println!("{}:{} is open", self.ip, port),
                Err(_) => println!("{}:{} is closed", self.ip, port),
            }
        }
    }

    async fn ping(&self, port: u16, dura: u64) -> Result<(), std::io::Error> {
        // 异步 TCP 连接逻辑
        let socket = std::net::SocketAddr::new(self.ip, port);
        let timeout = std::time::Duration::from_millis(dura);
        let stream = async_io::timeout(timeout, async_std::net::TcpStream::connect(socket)).await?;
        stream.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }
}