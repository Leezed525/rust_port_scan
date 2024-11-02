use port_scanner::scanner;
fn main() {
    let ip = "";
    let start = 1;
    let end = 8890;
    scanner::scan_port(ip.to_string(), start, end);
}