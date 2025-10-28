use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub enum PhotoSubCommand {
    #[command(
        name = "group",
        about = "照片按照拍摄时间分类",
        long_about = "得到照片的拍摄日期，按照拍摄时间分类"
    )]
    Group(GroupOpts),

    #[command(
        name = "time",
        about = "得到照片的拍摄时间",
        long_about = "得到照片的拍摄时间"
    )]
    ShootingTime(ShootingTimeOpts),
}

#[derive(Debug, Parser)]
pub struct GroupOpts {
    #[arg(short, long, help = "扫描的目录", long_help = "扫描的目录")]
    pub scan_dir: String,
    #[arg(short, long, help = "输出目录", long_help = "输出目录")]
    pub output_dir: String,
    #[arg(
        short,
        long,
        help = "操作方式",
        long_help = "分组时采用何种方式操作文件",
        default_value = "move"
    )]
    pub action: ActionType,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ActionType {
    Move,
    Copy,
}

#[derive(Debug, Parser)]
pub struct ShootingTimeOpts {
    #[arg(short, long, help = "照片路径", long_help = "照片路径")]
    pub photo_dir: String,
}
