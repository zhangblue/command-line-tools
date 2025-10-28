pub(crate) mod file;
pub(crate) mod base64;
pub(crate) mod date;
pub(crate) mod jsons;
pub(crate) mod photo;

use crate::cmd_opts::file::FilesSubCommand;
use crate::cmd_opts::base64::Base64SubCommand;
use crate::cmd_opts::date::TimeSubCommand;
use crate::cmd_opts::jsons::JsonOpts;
use crate::cmd_opts::photo::PhotoSubCommand;
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
    Date(TimeSubCommand),

    #[command(name = "json", about = "json进行格式化", long_about = "json格式化")]
    Json(JsonOpts),

    #[command(
        subcommand,
        about = "照片操作",
        long_about = "操作照片 \n- 目前支持的照片格式：[TIFF, RAW, HEIF, JPEG, WEBP, PNG, JPG, HEIC]"
    )]
    Photo(PhotoSubCommand),

    #[command(subcommand, about = "文件操作", long_about = "文件操作")]
    Files(FilesSubCommand),
}
