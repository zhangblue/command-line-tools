use crate::cmd_opts::base64::Base64Format;
use crate::{Base64SubCommand, get_reader};
use base64::Engine;
use base64::prelude::{BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD};
use std::io::Read;

/// 处理base64的编解码逻辑。解码方式必须与编码方式相同，否则会解不开
pub fn process_base64(base64sub_command: Base64SubCommand) -> anyhow::Result<()> {
    match base64sub_command {
        Base64SubCommand::Encode(opts) => process_encode(opts.input.as_str(), opts.format)?,
        Base64SubCommand::Decode(opts) => process_decode(opts.input.as_str(), opts.format)?,
    }
    Ok(())
}

// 对输入内容编码
fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input.trim())?;

    let mut buffer = Vec::new();
    let _ = reader.read_to_end(&mut buffer)?;

    let encode = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(buffer),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(buffer),
    };
    println!();
    println!("-------------编码为结果---------------");
    println!("{encode}");
    Ok(())
}

// 对输入的内容进行解码
fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input.trim())?;

    let mut buff = String::new();
    let _ = reader.read_to_string(&mut buff)?;
    let buffer = buff.trim();

    let decode = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(buffer),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.decode(buffer),
    }?;

    let decode = String::from_utf8(decode)?;

    println!();
    println!("-------------解码结果---------------");
    println!("{decode}");
    Ok(())
}
