use std::net;
//时间包
use std::time::Instant;
use std::time::Duration;
use std::io;
use std::net::Shutdown;
use async_std::io as async_io;
use log::debug;

use async_std::net as async_net;

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


pub fn scan_port(ip: String, start: u16, end: u16, dura: u64) {
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

        // if ping(&ip, port, dura) {
        //     println!("{}:{} is open", ip, port);
        // } else {
        //     println!("{}:{} is closed", ip, port);
        // }

        debug!("Time elapsed: {:?}", start.elapsed());
    }
}

pub async fn scan_port_async(ip: String, start: u16, end: u16, dura: u64) {
    //判断ip类型
    if !check_ipv4_valid(&ip) {
        println!("Invalid IP address {}", ip);
        return;
    }
    for port in start..end {
        let start = Instant::now();

        match async_ping(&ip, port, dura) {
            Ok(stream) => {
                println!("{}:{} is open", ip, port);
                stream.shutdown(Shutdown::Both).expect("shutdown call failed");
            }
            Err(_) => println!("{}:{} is closed", ip, port),
        };

        // if ping(&ip, port, dura) {
        //     println!("{}:{} is open", ip, port);
        // } else {
        //     println!("{}:{} is closed", ip, port);
        // }

        debug!("Time elapsed: {:?}", start.elapsed());
    }
}


async fn async_ping(ip: &str, port: u16, dura: u64) -> async_io::Result<async_net::TcpStream> {
    let timeout = Duration::from_millis(dura);
    let socket = net::SocketAddr::new(ip.parse().unwrap(), port);
    let stream = async_io::timeout(timeout, async_net::TcpStream::connect(socket)).await?;
    Ok(stream)
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