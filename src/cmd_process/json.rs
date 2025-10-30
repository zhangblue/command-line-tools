use crate::cmd_opts::jsons::{JsonOpts, OutputFormat};
use crate::component::{get_reader, stdout, write_to_file};
use crate::error;
use crate::error::Error;
use serde_json::Value;

pub fn process_json(json_opts: JsonOpts) -> error::Result<()> {
    println!("{json_opts:?}");

    json_format(
        json_opts.input.as_ref(),
        json_opts.output.as_ref(),
        json_opts.format,
    )?;

    Ok(())
}

fn json_format(
    input: Option<&String>,
    output: Option<&String>,
    format: OutputFormat,
) -> error::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buffer = Vec::new();
    let _ = reader
        .read_to_end(&mut buffer)
        .map_err(|err| Error::ReadInputError {
            msg: err.to_string(),
        })?;

    let input_value =
        String::from_utf8(buffer).map_err(|e| Error::ReadInputError { msg: e.to_string() })?;

    let value: Value = serde_json::from_str(&input_value).map_err(|_| "非法的json格式")?;

    let json_value = match format {
        OutputFormat::Compress => serde_json::to_string(&value),
        OutputFormat::Pretty => serde_json::to_string_pretty(&value),
    }
    .map_err(|_e| "转成Json格式失败")?;

    match output {
        None => {
            stdout(json_value);
        }
        Some(file) => {
            write_to_file(file, json_value)?;
        }
    }

    Ok(())
}
