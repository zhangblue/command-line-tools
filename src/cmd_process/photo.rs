use crate::cmd_opts::photo::{ActionType, PhotoSubCommand};
use crate::{Error, error};
use bloomfilter::Bloom;
use chrono::NaiveDateTime;
use exif::{In, Tag};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use walkdir::WalkDir;

pub fn process_photo(sub_command: PhotoSubCommand) -> error::Result<()> {
    // 得到支持的文件扩展名
    let photo_support_extension: HashSet<&str> =
        HashSet::from(["TIFF", "RAW", "HEIF", "JPEG", "WEBP", "PNG", "JPG", "HEIC"]);

    match sub_command {
        PhotoSubCommand::Group(opts) => process_group(
            &opts.scan_dir,
            &opts.output_dir,
            &opts.action,
            &photo_support_extension,
        )?,
        PhotoSubCommand::ShootingTime(opts) => {
            process_shooting_time(&opts.photo_dir, &photo_support_extension)?
        }
    }
    Ok(())
}

/// 得到照片的拍摄时间
fn process_shooting_time(input: &str, support_ext: &HashSet<&str>) -> error::Result<()> {
    let input_path = Path::new(input);
    if input_path.exists() && input_path.is_file() {
        if let Some(os_str) = input_path.extension()
            && let Some(ext) = os_str.to_str()
            && support_ext.contains(ext.to_uppercase().as_str())
        {
            println!("{}", get_file_shooting_time(input_path)?);
        } else {
            let support_ext_str = format!("{:?}", support_ext);
            return Err(Error::UnsupportedPhoto {
                msg: support_ext_str,
            });
        }
    }
    Ok(())
}

/// 将照片进行分组
fn process_group(
    scan_dir: &str,
    output_dir: &str,
    action: &ActionType,
    support_ext: &HashSet<&str>,
) -> error::Result<()> {
    // 重名文件检查器
    let mut file_name_checker: HashMap<String, Bloom<String>> = HashMap::with_capacity(1000);

    let walk_dir = WalkDir::new(scan_dir);
    for entry in walk_dir.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file()
            && let Some(os_str) = path.extension()
            && let Some(ext) = os_str.to_str()
        {
            // 检查扩展名是否支持
            if support_ext.contains(ext.to_uppercase().as_str()) {
                // 处理单个照片
                operation_single_photo(path, ext, &mut file_name_checker, action, output_dir)?;
            }
        }
    }
    Ok(())
}

/// 操作单个文件
fn operation_single_photo(
    path: &Path,
    extension: &str,
    file_name_checker: &mut HashMap<String, Bloom<String>>,
    action: &ActionType,
    output_dir: &str,
) -> error::Result<()> {
    // 获取照片的拍摄时间
    let shooting_time = get_file_shooting_time(path)?;
    // 通过拍摄时间得到应该放在哪一天的目录
    let target_forder = get_target_forder(&shooting_time)?;

    if !file_name_checker.contains_key(&target_forder) {
        let existing_file_names: Bloom<String> = Bloom::new_for_fp_rate(10000, 0.001)
            .map_err(|e| Error::OtherError { msg: e.to_string() })?;
        file_name_checker.insert(target_forder.clone(), existing_file_names);
    }

    if let Some(os_str) = path.file_name()
        && let Some(file_name) = os_str.to_str()
    {
        let bloom = file_name_checker.get_mut(&target_forder).unwrap();

        let mut file_name_current = String::from(file_name);
        let mut count = 1;
        while bloom.check(&file_name_current) {
            // 计算新名字
            file_name_current = format!("{file_name}-{count}.{extension}");
            count += 1;
        }
        bloom.set(&file_name_current);

        // 计算得到输出文件的目录
        let target_dir = Path::new(output_dir).join(&target_forder);

        if !target_dir.exists() {
            fs::create_dir_all(&target_dir)
                .map_err(|e| Error::CreateForderError { msg: e.to_string() })?;
        }

        match action {
            ActionType::Move => {
                fs::rename(path, target_dir.join(file_name_current))
                    .map_err(|e| Error::MoveFileError { msg: e.to_string() })?;
            }
            ActionType::Copy => {
                fs::copy(path, target_dir.join(file_name_current))
                    .map_err(|e| Error::CopyFileError { msg: e.to_string() })?;
            }
        }
    }

    Ok(())
}

/// 得到照片的拍摄时间
fn get_file_shooting_time(path: &Path) -> error::Result<String> {
    let file = File::open(path).map_err(|e| Error::FileNotExistError { msg: e.to_string() })?;
    let mut buffer = BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    let exif = exif_reader
        .read_from_container(&mut buffer)
        .map_err(|e| Error::ReadFileError { msg: e.to_string() })?;

    let shooting_time = exif
        .get_field(Tag::DateTimeOriginal, In::PRIMARY)
        .ok_or(Error::GetShootingTimeError)?
        .display_value()
        .to_string();

    Ok(shooting_time)
}

/// 根据拍摄时间得到日期目录
fn get_target_forder(shooting_time: &str) -> error::Result<String> {
    let date_forder = NaiveDateTime::parse_from_str(shooting_time, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| Error::DateFormatError { msg: e.to_string() })?
        .and_local_timezone(chrono::Local)
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    Ok(date_forder)
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::cmd_process::photo::get_file_shooting_time;
    use exif::{In, Tag};
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    #[test]
    fn test_get_file_shooting_time() {
        let time = get_file_shooting_time(Path::new("/Users/zhangdi/Downloads/IMG_2239.HEIC"));

        println!("{:?}", time);
    }

    #[test]
    fn test_get_photo_tag() {
        let path = Path::new("/Users/zhangdi/Downloads/test/output/2025-08-11/IMG_2102.HEIC");

        let file = File::open(path).unwrap();
        let mut buffer = BufReader::new(&file);
        let exif_reader = exif::Reader::new();
        let exif = exif_reader.read_from_container(&mut buffer).unwrap();

        let x = exif.get_field(Tag:: ImageWidth, In::PRIMARY);
        let y = exif.get_field(Tag:: ImageLength, In::PRIMARY);
        println!("{x:?}");
        println!("{y:?}");
    }
}
