use std::net::IpAddr;
use std::sync::Arc;
use super::executor::AsyncScanExecutor;

pub struct AsyncScannerEngine {
    // Fields for the scanner engine
    ip: IpAddr, // 扫描的目标IP地址
    begin: u16,
    end: u16, // 扫描端口范围
    max_concurrent: usize, // 最大并发连接数
    executors: Vec<AsyncScanExecutor>, // 扫描执行器列表
}

impl AsyncScannerEngine {
    // 创建新的异步扫描引擎
    pub fn new(ip: &str, begin: u16, end: u16, max_concurrent: usize) -> Self {
        // 检查IP地址是否合法
        let ip = ip.parse::<IpAddr>().expect("IP 地址不合法");

        // 检查端口范围是否合法
        if begin > end || begin < 1 || end > 65535 {
            panic!("端口范围不合法: {}-{}", begin, end);
        }

        //划分任务
        let mut executors = Vec::new();
        let port_range = end - begin + 1;
        let ports_per_executor = (port_range as f64 / max_concurrent as f64).ceil() as u16;
        for i in 0..max_concurrent {
            let start_port = begin + i as u16 * ports_per_executor;
            let end_port = if i == max_concurrent - 1 {
                end
            } else {
                start_port + ports_per_executor - 1
            };
            executors.push(AsyncScanExecutor::new(ip, start_port, end_port));
        }

        AsyncScannerEngine {
            ip,
            begin,
            end,
            max_concurrent,
            executors,
        }
    }

    pub fn run(&self, dura: u64) {
        let mut tasks = Vec::new();
        // 直接遍历 self.executors 的引用
        for executor in &self.executors {
            tasks.push(executor.execute(dura));
        }
        // 等待所有任务完成
        futures::executor::block_on(futures::future::join_all(tasks));
        println!("扫描完成");
    }
}