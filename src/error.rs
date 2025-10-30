pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum Error {
    FileNotExistError { msg: String }, // 文件不存在
    OpenFileError(String),
    ReadInputError { msg: String }, // 读取输入内容错误
    ReadFileError { msg: String },
    WriteOutputError { msg: String },  // 写文件失败
    Base64DecodeError { msg: String }, // base64 解码错误
    DateFormatError(String),
    CreateForderError(String),
    CreateFileError { msg: String },
    MoveFileError(String),
    CopyFileError(String),
    OtherError(String),
}

impl Error {
    pub fn print_error_msg(&self) {
        match self {
            Error::CopyFileError(value) => eprintln!("拷贝文件失败: {value}"),
            Error::OtherError(value) => eprintln!("{value}"),

            Error::FileNotExistError { msg } => println!("文件不存在: {msg}"),
            Error::ReadInputError { msg } => println!("读取输入内容失败: {msg}"),
            Error::WriteOutputError { msg } => println!("生成输出文件失败: {msg}"),
            Error::Base64DecodeError { msg } => println!("base64解码失败: {msg}"),
            Error::ReadFileError { msg } => println!("读取文件内容失败：{msg}"),
            Error::CreateForderError(msg) => println!("创建目录失败: {msg}"),
            Error::CreateFileError { msg } => println!("创建文件失败: {msg}"),
            Error::MoveFileError(value) => println!("移动文件失败: {value}"),

            Error::OpenFileError(msg) => eprintln!("打开文件失败: {msg}"),
            Error::DateFormatError(msg) => eprintln!("日期格式错误：{msg}"),
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::OtherError(value.to_string())
    }
}
