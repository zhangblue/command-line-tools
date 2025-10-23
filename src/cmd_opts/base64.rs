use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "base64 编码")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "base64 解码")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(
        short,
        long,
        help = "需要编码的文件路径，无此参数则手动输入文本",
        long_help = "需要编码的文件路径，无此参数则手动输入文本"
    )]
    pub input: Option<String>,

    #[arg(
        short,
        long,
        help = "输出文件。无此参数将在终端打印",
        long_help = "输出文件。无此参数将在终端打印"
    )]
    pub output: Option<String>,

    #[arg(
        short,
        long,
        default_value = "standard",
        help = "编码方式",
        long_help = "Standard: 标准模式。UrlSafe: url安全模式"
    )]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(
        short,
        long,
        help = "需要解码的文件路径，如不填则手动输入文本",
        long_help = "需要解码的文件路径，如不填则手动输入文本"
    )]
    pub input: Option<String>,

    #[arg(
        short,
        long,
        help = "输出文件。无此参数将在终端打印",
        long_help = "输出文件。无此参数将在终端打印"
    )]
    pub output: Option<String>,

    #[arg(
        short,
        long,
        default_value = "standard",
        help = "编码方式",
        long_help = "Standard: 标准模式。UrlSafe: url安全模式"
    )]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Base64Format {
    // 标准模式：可能会出现特殊字符，无法在网络上进行传递。
    Standard,
    // url安全模式：对url中的&等特殊符号进行处理
    UrlSafe,
}
