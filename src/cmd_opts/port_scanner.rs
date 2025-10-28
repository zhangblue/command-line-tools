use clap::Parser;
use std::net::IpAddr;

#[derive(Debug, Parser)]
pub struct PortScannerOpts {
    #[arg(short, long, help = "扫描的地址", long_help = "要扫描的地址")]
    pub addr: IpAddr,
    #[arg(
        short = 's',
        long,
        help = "扫描起始端口",
        long_help = "扫描起始端口",
        default_value = "1"
    )]
    pub port_start: u16,
    #[arg(
        short = 'e',
        long,
        help = "扫描结束端口",
        long_help = "扫描结束端口",
        default_value = "1024"
    )]
    pub port_end: u16,
}
