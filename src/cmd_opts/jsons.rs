use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct JsonOpts {
    #[arg(
        short,
        long,
        help = "输入文件。无此参数使用命令行输入文本",
        long_help = "输入文件。无此参数使用命令行输入文本"
    )]
    pub input: Option<String>,

    #[arg(
        short,
        long,
        help = "输出文件。无此参数将控制台打印结果",
        long_help = "输出文件。无此参数将控制台打印结果"
    )]
    pub output: Option<String>,

    #[arg(
        short,
        long,
        default_value = "compress",
        long_help = "输出格式。Compress: 压缩在一行。Pretty: 美化",
        help = "输出格式。Compress: 压缩在一行。Pretty: 美化"
    )]
    pub format: OutputFormat,
}

#[derive(Debug, Parser, Copy, Clone, ValueEnum)]
pub enum OutputFormat {
    Compress, // 压缩。
    Pretty,   // 美化
}
