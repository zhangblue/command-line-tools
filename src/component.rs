use crate::{Error, error};
use std::fmt::Display;
use std::fs::File;
use std::io::{Read, Write};

// 读取文件。如果参数是“-”就从命令行交互中读取数据，否则input就是一个文件路径
pub fn get_reader(input: Option<&String>) -> error::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input.is_none() {
        println!("请输入内容: ");
        std::io::stdout()
            .flush()
            .map_err(|e| Error::ReadInputError { msg: e.to_string() })?; // 强制刷新输出。确保提示立即显示
        Box::new(std::io::stdin())
    } else {
        Box::new(
            File::open(input.unwrap().trim()).map_err(|_| Error::FileNotExistError {
                msg: input.unwrap().to_owned(),
            })?,
        )
    };
    Ok(reader)
}

// 在命令行中打印数据
pub fn stdout<T>(content: T)
where
    T: Display,
{
    println!("------结果-------");
    println!("{content}");
}

// 结果写入文件
pub fn write_to_file(output: &str, content: String) -> error::Result<()> {
    std::fs::write(output, content).map_err(|err| Error::WriteOutputError {
        msg: err.to_string(),
    })?;
    Ok(())
}
