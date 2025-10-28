use clap::Parser;

#[derive(Debug, Parser)]
pub enum FilesSubCommand {
    #[command(
        name = "repeat",
        about = "寻找重复的文件",
        long_about = "通过计算文件的sha256，找到相同的文件"
    )]
    Repeat(RepeatOpts),
}

#[derive(Debug, Parser)]
pub struct RepeatOpts {
    #[arg(short, long, help = "扫描的目录", long_help = "扫描的目录")]
    pub scan_dir: String,

    #[arg(
        short,
        long,
        help = "输出结果的文件",
        long_help = "将结果输出到一个文件中"
    )]
    pub output: String,
}
