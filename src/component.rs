use crate::{Error, error};
use sha2::Digest;
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

/// 计算文件的sha256
pub fn compute_sha256(file_path: &str) -> error::Result<String> {
    let mut file =
        File::open(file_path).map_err(|e| Error::FileNotExistError { msg: e.to_string() })?;
    let mut hasher = sha2::Sha256::new();
    let mut buffer = [0u8; 1024 * 64];

    loop {
        let bytes_read = file
            .read(&mut buffer)
            .map_err(|e| Error::ReadFileError { msg: e.to_string() })?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();

    let hex_str: Vec<String> = result[..]
        .iter()
        .map(|&byte| format!("{:02X}", byte))
        .collect();

    Ok(hex_str.join("").to_lowercase())
}

#[cfg(test)]
mod tests {
    use crate::component::compute_sha256;

    #[test]
    fn test_compute_sha256() {
        let sha256 =
            compute_sha256("/Users/zhangdi/work/workspace/github/myself/rust_cmd/Cargo.toml")
                .unwrap();

        assert_eq!(
            sha256,
            "caa578b585b2ca4e250e86fdfaa12f4c4a3ed8bed5b7021dba288a9a509bc055"
        );
    }
}
