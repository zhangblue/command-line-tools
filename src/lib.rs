mod cmd_opts;
mod cmd_process;

pub use cmd_opts::Opts;
pub use cmd_opts::SubCommand;
pub use cmd_opts::base64::Base64SubCommand;

pub use cmd_process::process_base64;
pub use cmd_process::process_times;


use std::fs::File;
use std::io::{Read, Write};

// 读取文件。如果参数是“-”就从命令行交互中读取数据，否则input就是一个文件路径
pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        println!("请输入内容: ");
        std::io::stdout().flush()?; // 强制刷新输出。确保提示立即显示
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    Ok(reader)
}
