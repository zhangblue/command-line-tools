use crate::cmd_opts::port_scanner::PortScannerOpts;
use crate::error;
use std::net::IpAddr;
use std::process::exit;

pub async fn process_port_scanner(opts: PortScannerOpts) -> error::Result<()> {
    args_validate(&opts);

    for port in opts.port_start..=opts.port_end {
        let scan_attempt = scan(opts.addr, port).await;
        if let Some((addr, port)) = scan_attempt {
            println!("{}:{}", addr, port);
        }
    }

    Ok(())
}

async fn scan(addr: IpAddr, port: u16) -> Option<(IpAddr, u16)> {
    let connection_attempt = tokio::net::TcpStream::connect((addr, port)).await;
    if let Ok(_open) = connection_attempt {
        return Some((addr, port));
    }
    None
}

fn args_validate(args: &PortScannerOpts) {
    if args.port_start <= 0 {
        eprintln!("参数错误: port_start 必须大于0");
        exit(2);
    }

    if args.port_end < args.port_start {
        eprintln!("参数错误：port_end 不能小于 port_start");
        exit(2);
    }
}
