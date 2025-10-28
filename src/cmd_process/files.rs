use crate::cmd_opts::file::FilesSubCommand;
use crate::component::{compute_sha256, write_to_file};
use crate::{Error, error};
use std::collections::HashMap;
use walkdir::WalkDir;

pub fn process_files(sub_command: FilesSubCommand) -> error::Result<()> {
    match sub_command {
        FilesSubCommand::Repeat(opts) => process_repeat(&opts.scan_dir, &opts.output),
    }
}

/// 检查重复文件
fn process_repeat(scan_dir: &str, output: &str) -> error::Result<()> {
    let mut file_meta: HashMap<String, Vec<String>> = HashMap::with_capacity(100000);

    let walk_dir = WalkDir::new(scan_dir);

    for entry in walk_dir.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let file_path = path.to_str().unwrap();
            // 计算sha256
            let sha256 = compute_sha256(file_path)?;

            if !file_meta.contains_key(&sha256) {
                file_meta.insert(sha256.clone(), Vec::with_capacity(10));
            }
            file_meta
                .get_mut(&sha256)
                .unwrap()
                .push(String::from(file_path));
        }
    }

    let result_map: HashMap<String, Vec<String>> =
        file_meta.into_iter().filter(|(_, v)| v.len() > 1).collect();

    let json = serde_json::to_string(&result_map)
        .map_err(|e| Error::IllegalJsonError { msg: e.to_string() })?;

    write_to_file(output, json)?;

    Ok(())
}
