use clap::Parser;
use port_scanner::scanner;
use port_scanner::parser;

fn main() {
    let args = parser::ScannerConfig::parse();

    println!("Hello {}!", args.ip);
    println!("Hello {}!", args.begin);
    println!("Hello {}!", args.end);


    scanner::scan_port(args.ip, args.begin, args.end);
}