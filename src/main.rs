use port_scanner::scanner;
fn main() {
    let ip = "";
    let start = 35440;
    let end = 40000;
    scanner::scan_port(ip.to_string(), start, end);
}