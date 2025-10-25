pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum Error {
    FileDoesNotExistError { msg: String }, // 文件不存在
    IllegalJsonError { msg: String },      // 非法Json
    ReadInputError { msg: String },        // 读取输入内容错误
    JsonFormatError { msg: String },       // json格式化失败
    WriteOutputError { msg: String },      // 写文件失败
    FormatUtf8Error { msg: String },       // 转成utf8错误
    Base64DecodeError { msg: String },     // base64 解码错误
}

impl Error {
    pub fn print_error_msg(&self) {
        match self {
            Error::FileDoesNotExistError { msg } => println!("文件不存在: {msg}"),
            Error::IllegalJsonError { msg } => println!("非法json格式: {msg}"),
            Error::ReadInputError { msg } => println!("读取输入文件失败: {msg}"),
            Error::JsonFormatError { msg } => println!("Json格式化失败: {msg}"),
            Error::WriteOutputError { msg } => println!("生成输出文件失败: {msg}"),
            Error::FormatUtf8Error { msg } => println!("转成UTF8格式失败: {msg}"),
            Error::Base64DecodeError { msg } => println!("base64解码失败: {msg}"),
        }
    }
}
