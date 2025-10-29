use crate::cmd_opts::port_scanner::PortScannerOpts;
use crate::error;
use std::net::IpAddr;
use std::process::exit;

pub async fn process_port_scanner(opts: PortScannerOpts) -> error::Result<()> {
    args_validate(&opts);

    let port_range: Vec<u16> = (opts.port_start..=opts.port_end).collect();
    let chunks: Vec<Vec<u16>> = port_range
        .chunks(opts.parallel as usize)
        .map(|s| s.to_vec())
        .collect();

    let mut handles = Vec::with_capacity(chunks.len());
    for chunk in chunks.into_iter() {
        let h = tokio::spawn(async move { scan_ports(opts.addr, chunk).await });
        handles.push(h);
    }

    for h in handles {
        if let Err(e) = h.await {
            println!("PortScanner exited with error: {}", e);
        }
    }

    println!("All finish");

    Ok(())
}

async fn scan_ports(addr: IpAddr, ports: Vec<u16>) {
    for port in ports {
        let scan_attempt = scan(addr, port).await;
        if let Some((addr, port)) = scan_attempt {
            println!("{}:{}", addr, port);
        }
    }
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
