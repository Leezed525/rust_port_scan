use clap::Parser;


#[derive(Parser)]
#[clap(version = "1.0", author = "Leezed <leezed525@qq.com>", about = "Does awesome things")]
pub struct ScannerConfig {
    #[clap(long)]
    pub ip: String,

    #[clap(short, long, default_value_t = 80)]
    pub begin: u16,

    #[clap(short, long, default_value_t = 443)]
    pub end: u16,

    #[clap(short, long, default_value_t = 100)]
    pub dura: u64,

    #[clap(short, long, default_value = "sync")]
    pub scan_type: String,
}