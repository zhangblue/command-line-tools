use crate::cmd_opts::port_scanner::PortScannerOpts;
use crate::error;
use std::net::IpAddr;
use std::process::exit;

pub async fn process_port_scanner(opts: PortScannerOpts) -> error::Result<()> {
    args_validate(&opts);

    let mut handles = Vec::with_capacity((opts.port_end - opts.port_start + 1) as usize);
    for port in opts.port_start..=opts.port_end {
        let join_handle = tokio::spawn(scan(opts.addr, port));
        handles.push(join_handle);
    }
    for h in handles {
        if let Some((ip, port)) = h.await.unwrap() {
            println!("{}:{}", ip, port);
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
    if args.port_start == 0 {
        eprintln!("参数错误: port_start 必须大于0");
        exit(2);
    }

    if args.port_end < args.port_start {
        eprintln!("参数错误：port_end 不能小于 port_start");
        exit(2);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_port_group() {
        let begin = 1;
        let end = 1024;
        let ports: Vec<i32> = (begin..=end).collect();
        let chunks = ports.chunks(10);
    }
}
