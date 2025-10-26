use crate::cmd_opts::date::{DateFormat, TimeSubCommand, TimestampUnit};
use crate::component::stdout;
use crate::{Error, error};
use chrono::TimeZone;

pub fn process_times(sub_command: TimeSubCommand) -> error::Result<()> {
    match sub_command {
        TimeSubCommand::ToDate(date_opt) => {
            process_to_date(date_opt.input, date_opt.timezone, date_opt.unit)?
        }

        TimeSubCommand::ToTimeStamp(timestamp_opt) => {
            println!("{timestamp_opt:?}");
            process_to_timestamp(
                timestamp_opt.input,
                timestamp_opt.format,
                timestamp_opt.timezone,
                timestamp_opt.unit,
            )?
        }
    }

    Ok(())
}

/// 将日期转成时间戳
fn process_to_timestamp(
    input: String,
    format: DateFormat,
    timezone: Option<i32>,
    output: TimestampUnit,
) -> error::Result<()> {
    let naive_dt = chrono::NaiveDateTime::parse_from_str(&input, format.to_format())
        .map_err(|e| Error::DateFormatError { msg: e.to_string() })?;

    let offset = get_time_zome_offset(timezone)?;

    let local_dt = offset
        .from_local_datetime(&naive_dt)
        .single()
        .ok_or(Error::TimezoneError)?;

    let res = match output {
        TimestampUnit::Second => local_dt.with_timezone(&chrono::Utc).timestamp(),
        TimestampUnit::Millisecond => local_dt.with_timezone(&chrono::Utc).timestamp_millis(),
    };

    stdout(res);

    Ok(())
}

/// 将时间戳转成日期格式
fn process_to_date(
    timestamp_opt: Option<i64>,
    timezone: Option<i32>,
    format: TimestampUnit,
) -> error::Result<()> {
    let timestamp = timestamp_opt.unwrap_or_else(|| chrono::Utc::now().timestamp_millis());

    let time_zone = get_time_zome_offset(timezone)?;

    // 计算时区
    let format_date = match format {
        TimestampUnit::Second => chrono::DateTime::from_timestamp(timestamp, 0),
        TimestampUnit::Millisecond => chrono::DateTime::from_timestamp_millis(timestamp),
    }
    .ok_or(Error::TimestampFormatError {
        msg: timestamp.to_string(),
    })?;
    let formated = format_date
        .with_timezone(&time_zone)
        .format("%Y-%m-%d %H:%M:%S%.3f")
        .to_string();

    stdout(formated);

    Ok(())
}

fn get_time_zome_offset(timezone: Option<i32>) -> error::Result<chrono::FixedOffset> {
    let offset = match timezone {
        None => chrono::Local::now().offset().local_minus_utc(),
        Some(hour) => hour * 3600,
    };

    // 计算时区
    let time_zone = if offset > 0 {
        chrono::FixedOffset::east_opt(offset)
    } else if offset < 0 {
        chrono::FixedOffset::west_opt(offset.abs())
    } else {
        chrono::FixedOffset::east_opt(offset)
    }
    .ok_or(Error::TimezoneError)?;

    Ok(time_zone)
}

#[cfg(test)]
mod tests {
    use chrono::Local;

    #[test]
    fn get_current_timezone() {
        let offset_sec = Local::now().offset().local_minus_utc();

        assert_eq!(offset_sec, 28800);
    }
}
