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
        println!("每个执行器处理端口数: {}", ports_per_executor);

        let mut start_port = begin;
        let mut end_port = begin + ports_per_executor - 1;
        while end_port <= end {
            executors.push(AsyncScanExecutor::new(ip, start_port, end_port));
            if end_port == end {
                break; // 如果已经到达端口范围的末尾，则退出循环
            }
            start_port = end_port + 1;
            end_port = start_port.checked_add(ports_per_executor - 1).unwrap_or(end);
        }

        AsyncScannerEngine {
            ip,
            begin,
            end,
            max_concurrent,
            executors,
        }
    }

    pub fn run(&mut self, dura: u64) {

        let mut tasks = Vec::new();
        // 直接遍历 self.executors 的引用
        for executor in &mut self.executors {
            tasks.push(executor.execute(dura));
        }
        println!("开始扫描 {}:{}-{}", self.ip, self.begin, self.end);
        // 等待所有任务完成
        futures::executor::block_on(futures::future::join_all(tasks));
        //获取扫描结果
        let mut res = Vec::new();
        for executor in &self.executors {
            res.extend(executor.get_results());
        }

        //排序
        res.sort_unstable();
        //输出结果
        for port in &res {
            println!("{}:{} is open", self.ip, port);
        }

        println!("共找到 {} 个开放端口", &res.len());
    }
}