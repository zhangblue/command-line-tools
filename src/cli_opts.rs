mod base64;

use crate::cli_opts::base64::Base64SubCommand;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "cli",
    version,
    author,
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
}
