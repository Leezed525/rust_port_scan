use std::collections::HashMap;
use std::net;
//时间包
use std::time::Instant;
use std::time::Duration;
use std::io;
use std::iter::Map;
use std::net::Shutdown;
use std::sync::Arc;
use async_std::io as async_io;
use log::debug;
use futures::future::join_all;
use tokio::sync::Semaphore;


use async_std::net as async_net;

//建立tcp 连接
fn ping(ip: &str, port: u16, dura: u64) -> io::Result<net::TcpStream> {
    //设置超时时间
    let timeout = Duration::from_millis(dura);
    // 创建socketAddr
    let socket = net::SocketAddr::new(ip.parse().unwrap(), port);
    net::TcpStream::connect_timeout(&socket, timeout)
}

fn check_ipv4_valid(ip: &str) -> bool {
    ip.parse::<net::Ipv4Addr>().is_ok()
}


pub fn scan_port_sync(ip: String, start: u16, end: u16, dura: u64) {
    //判断ip类型
    if !check_ipv4_valid(&ip) {
        println!("Invalid IP address {}", ip);
        return;
    }
    for port in start..end {
        let start = Instant::now();
        match ping(&ip, port, dura) {
            Ok(stream) => {
                println!("{}:{} is open", ip, port);
                stream.shutdown(Shutdown::Both).expect("shutdown call failed");
            }
            Err(_) => println!("{}:{} is closed", ip, port),
        };
        debug!("Time elapsed: {:?}", start.elapsed());
    }
}


//创建异步tcp连接
async fn async_ping(ip: &str, port: u16, dura: u64, semaphore: Arc<Semaphore>) -> Result<String,std::io::Error> {
    //设置并发量
    let _permit = semaphore.acquire().await.unwrap();
    let timeout = Duration::from_millis(dura);
    let socket = net::SocketAddr::new(ip.parse().unwrap(), port);
    let stream = async_io::timeout(timeout, async_net::TcpStream::connect(socket)).await?;
    //关闭连接
    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    Ok("success".to_string())
}

pub async fn scan_port_async(ip: String, start: u16, end: u16, dura: u64, max_concurrent: usize) {
    //判断ip类型
    if !check_ipv4_valid(&ip) {
        println!("Invalid IP address {}", ip);
        return;
    }

    let time = Instant::now();
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    let mut ping_tasks = Vec::new();
    for port in start..end {
        let semaphore = Arc::clone(&semaphore);
        ping_tasks.push(async_ping(&ip, port, dura, semaphore));
    }
    println!("{:?}", ping_tasks.len());
    println!("Start to scan");
    let results = join_all(ping_tasks).await;
    println!("Time elapsed: {:?}", time.elapsed());
    let mut mp = HashMap::new();
    for (port, result) in (start..end).zip(results) {
        match result {
            Ok(string) => {
                println!("{}:{} is open", ip, port);
            }
            Err(E) => {
                //输出错误信息
                let count = mp.entry(E.to_string()).or_insert(0);
                *count += 1;
            }
        };
    }

    for (err, count) in mp {
        println!("{}: {}", err, count);
    }
}


mod test {
    use super::ping;
    #[test]
    fn test_ping() {
        let ip = "127.0.0.1";
        let port = 80;
        assert!(!ping(ip, port, 100));
    }
}