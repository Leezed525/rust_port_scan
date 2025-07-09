use clap::Parser;
use port_scanner::scanner;
use port_scanner::parser;
use port_scanner::scanner::engine;
use tokio;
use std::time::{Instant, Duration};



#[tokio::main]
async fn main() {
    let args = parser::ScannerConfig::parse();
    // if args.scan_type == "sync" {
    //     scanner::scan_port_sync(args.ip, args.begin, args.end, args.dura);
    // } else if args.scan_type == "async" {
    //     scanner::scan_port_async(args.ip, args.begin, args.end, args.dura,args.max_concurrent).await;
    // }

    //计算消耗的时间
    let start_time = Instant::now();
    let mut engine = engine::AsyncScannerEngine::new(&args.ip, args.begin, args.end, args.max_concurrent);
    engine.run(args.dura);
    let elapsed_time = start_time.elapsed();
    println!("扫描完成，耗时: {:?}", elapsed_time);
}