use clap::Parser;
use port_scanner::scanner;
use port_scanner::parser;

fn main() {
    let args = parser::ScannerConfig::parse();
    if args.scan_type == "sync" {
        scanner::scan_port_sync(args.ip, args.begin, args.end, args.dura);
    } else if args.scan_type == "async" {
        scanner::scan_port_async(args.ip, args.begin, args.end, args.dura);
    }
}