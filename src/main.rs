use clap::Parser;
use port_scanner::scanner;
use port_scanner::parser;

fn main() {
    let args = parser::ScannerConfig::parse();
    scanner::scan_port(args.ip, args.begin, args.end, args.dura);
}