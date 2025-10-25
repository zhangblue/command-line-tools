use crate::cmd_opts::base64::Base64Format;
use crate::component::{get_reader, stdout, write_to_file};
use crate::{Base64SubCommand, Error, error};
use base64::Engine;
use base64::prelude::{BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD};
use std::io::Read;

/// 处理base64的编解码逻辑。解码方式必须与编码方式相同，否则会解不开
pub fn process_base64(base64sub_command: Base64SubCommand) -> error::Result<()> {
    match base64sub_command {
        Base64SubCommand::Encode(opts) => {
            process_encode(opts.input.as_ref(), opts.output.as_ref(), opts.format)?
        }
        Base64SubCommand::Decode(opts) => {
            process_decode(opts.input.as_ref(), opts.output.as_ref(), opts.format)?
        }
    }
    Ok(())
}

// 对输入内容编码
fn process_encode(
    input: Option<&String>,
    output: Option<&String>,
    format: Base64Format,
) -> error::Result<()> {
    let mut reader = get_reader(input)?;

    let mut buffer = Vec::new();
    let _ = reader.read_to_end(&mut buffer).map_err(|e|Error::ReadInputError {msg: e.to_string() })?;

    let encode = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(buffer),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(buffer),
    };

    match output {
        None => stdout(encode),
        Some(file) => write_to_file(file, encode)?,
    }

    Ok(())
}

// 对输入的内容进行解码
fn process_decode(
    input: Option<&String>,
    output: Option<&String>,
    format: Base64Format,
) -> error::Result<()> {
    let mut reader = get_reader(input)?;

    let mut buff = String::new();
    let _ = reader
        .read_to_string(&mut buff)
        .map_err(|e| Error::ReadInputError { msg: e.to_string() })?;
    let buffer = buff.trim();

    let decode = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(buffer),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.decode(buffer),
    }
    .map_err(|e| Error::Base64DecodeError { msg: e.to_string() })?;

    let decode =
        String::from_utf8(decode).map_err(|e| Error::FormatUtf8Error { msg: e.to_string() })?;

    match output {
        None => stdout(decode),
        Some(file) => write_to_file(file, decode)?,
    }
    Ok(())
}
