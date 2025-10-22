pub(crate) mod base64;
pub(crate) mod times;

use crate::cmd_opts::base64::Base64SubCommand;
use crate::cmd_opts::times::TimeSubCommand;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "rust-command-line",
    version,
    author = "张三 <zhangsan@example.com>",
    about = "rust编写的命令行工具",
    long_about = "rust编写的命令行工具"
)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(
        subcommand,
        about = "对文本或者文件进行 base64 编解码",
        long_about = "对文本或文件进行 base64 编解码"
    )]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "对时间进行处理", long_about = "对时间进行处理")]
    Time(TimeSubCommand),
}
