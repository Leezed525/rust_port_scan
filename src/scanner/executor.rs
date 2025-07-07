use std::net::IpAddr;

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
}