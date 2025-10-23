pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum Error {
    FileDoesNotExistError { msg: String }, // 文件不存在
    NotCorrectJson { msg: String }, // 不是正确的Json
}
