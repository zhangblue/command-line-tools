pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum Error {
    FileNotExistError { msg: String }, // 文件不存在
    IllegalJsonError { msg: String },  // 非法Json
    ReadInputError { msg: String },    // 读取输入内容错误
    ReadFileError { msg: String },
    JsonFormatError { msg: String },      // json格式化失败
    WriteOutputError { msg: String },     // 写文件失败
    FormatUtf8Error { msg: String },      // 转成utf8错误
    Base64DecodeError { msg: String },    // base64 解码错误
    TimestampError { msg: String },       // 获取时间戳错误
    TimestampFormatError { msg: String }, // 时间戳转换失败
    TimezoneError,
    DateFormatError { msg: String },
    ByteToStringError { msg: String },
    GetShootingTimeError,
    CreateForderError { msg: String },
    CreateFileError { msg: String },
    MoveFileError { msg: String },
    CopyFileError { msg: String },
    OtherError { msg: String },
    UnsupportedPhoto { msg: String },
}

impl Error {
    pub fn print_error_msg(&self) {
        match self {
            Error::FileNotExistError { msg } => println!("文件不存在: {msg}"),
            Error::IllegalJsonError { msg } => println!("非法json格式: {msg}"),
            Error::ReadInputError { msg } => println!("读取输入内容失败: {msg}"),
            Error::JsonFormatError { msg } => println!("Json格式化失败: {msg}"),
            Error::WriteOutputError { msg } => println!("生成输出文件失败: {msg}"),
            Error::FormatUtf8Error { msg } => println!("转成UTF8格式失败: {msg}"),
            Error::Base64DecodeError { msg } => println!("base64解码失败: {msg}"),
            Error::TimestampError { msg } => println!("获取时间戳失败: {msg}"),
            Error::TimestampFormatError { msg } => println!("时间戳转换失败: {msg}"),
            Error::DateFormatError { msg } => println!("时间格式不正确: {msg}"),
            Error::ReadFileError { msg } => println!("读取文件内容失败：{msg}"),
            Error::TimezoneError => println!("时区设置错误"),
            Error::ByteToStringError { msg } => println!("字节转字符串失败: {msg}"),
            Error::GetShootingTimeError => println!("获取拍摄时间失败"),
            Error::OtherError { msg } => println!("其他异常：{msg}"),
            Error::CreateForderError { msg } => println!("创建目录失败: {msg}"),
            Error::CreateFileError { msg } => println!("创建文件失败: {msg}"),
            Error::MoveFileError { msg } => println!("移动文件失败: {msg}"),
            Error::CopyFileError { msg } => println!("拷贝文件失败: {msg}"),
            Error::UnsupportedPhoto { msg } => println!("不支持的照片格式. 目前只支持：{msg}"),
        }
    }
}
