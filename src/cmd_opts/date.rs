use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub enum TimeSubCommand {
    #[command(
        name = "to_date",
        about = "时间戳转日期",
        long_about = "将输入的时间戳转换成日期格式",
        author
    )]
    ToDate(DateOpt),
    #[command(name = "to_timestamp", about = "将日期转成时间戳", author)]
    ToTimeStamp(TimeStampOpt),
}

#[derive(Debug, Parser)]
pub struct DateOpt {
    #[arg(
        short,
        long,
        help = "时间戳",
        long_help = "时间戳。\n- 如不填则自动获取当前时间戳"
    )]
    pub input: Option<i64>,

    #[arg(
        short,
        long,
        help = "输入时间戳的单位",
        long_help = "时间戳单位. 次参数必须与input参数匹配。 \n- 可选：second (秒)，millisecond (毫秒)",
        default_value = "millisecond"
    )]
    pub unit: TimestampUnit,

    #[arg(
        short,
        long,
        help = "输出日期的时区",
        long_help = "输出日期的时区。\n- 负数表示西区，正数表示东区，0表示UTC。默认使用当前时区 \n- 例子：东8区: -t=8 \n- 例子：西2区: -t=-2"
    )]
    pub timezone: Option<i32>,
}

#[derive(Debug, Parser)]
pub struct TimeStampOpt {
    #[arg(short, long, help = "日期", long_help = "日期")]
    pub input: String,

    #[arg(short, long, help = "日期格式", long_help = "日期格式")]
    pub format: DateFormat,

    #[arg(
        short,
        long,
        help = "时区。负数表示西区，正数表示东区，0表示UTC。默认使用当前时区",
        long_help = "时区。负数表示西区，正数表示东区，0表示UTC。默认使用当前时区",
        alias = "tz"
    )]
    pub timezone: Option<i32>,

    #[arg(short, long, help = "输出时间戳单位", long_help = "输出时间戳单位")]
    pub unit: TimestampUnit,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TimestampUnit {
    Second,
    Millisecond,
}

#[derive(Copy, Clone, Debug, PartialEq, ValueEnum)]
pub enum DateFormat {
    #[clap(name = "yyyy-MM-dd")]
    Ymd,
    #[clap(name = "yyyy-MM-dd_HH")]
    YmdH,
    #[clap(name = "yyyy-MM-dd_HH:mm")]
    YmdHm,
    #[clap(name = "yyyy-MM-dd_HH:mm::ss")]
    YmdHms,
    #[clap(name = "yyyy-MM-dd_HH:mm::ss.SSS")]
    YmdHmsS,
}

impl DateFormat {
    pub fn to_format(&self) -> &'static str {
        match self {
            DateFormat::Ymd => "%Y-%m-%d",
            DateFormat::YmdH => "%Y-%m-%d_%H",
            DateFormat::YmdHm => "%Y-%m-%d_%H:%M",
            DateFormat::YmdHms => "%Y-%m-%d_%H:%M:%S",
            DateFormat::YmdHmsS => "%Y-%m-%d_%H:%M:%S%.3f",
        }
    }
}
