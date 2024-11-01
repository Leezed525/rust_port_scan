use std::net;


fn ping(ip: &str, port: u16) -> bool {
    let ip_port = format!("{}:{}", ip, port);
    match net::TcpStream::connect(ip_port) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn check_ipv4_valid(ip: &str) -> bool {
    ip.parse::<net::Ipv4Addr>().is_ok()
}


pub fn scan_port(ip: String, start: u16, end: u16) {
    //判断ip类型
    if !check_ipv4_valid(&ip) {
        println!("Invalid IP address {}", ip);
        return;
    }
    for port in start..end {
        if ping(&ip, port) {
            println!("{}:{} is open", ip, port);
        } else {
            println!("{}:{} is closed", ip, port);
        }
    }
}


mod test {
    use super::ping;
    #[test]
    fn test_ping() {
        let ip = "127.0.0.1";
        let port = 80;
        assert!(!ping(ip, port));
    }
    #[test]
    fn test_scan_port() {
        let ip = "";
        let start = 1;
        let end = 10000;
        super::scan_port(ip.to_string(), start, end);
        assert!(false);
    }
}