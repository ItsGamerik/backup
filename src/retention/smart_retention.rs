use std::fs;
use std::path::Path;

use chrono::{NaiveDate, NaiveDateTime};

use chrono::{Datelike, Local, Timelike};

use crate::config_toml::check_and_read::read_conf;

pub fn backupctrl(files: Vec<fs::DirEntry>, dry_run: bool) {
    let mut dir_names: Vec<String> = Vec::new();
    for file in files {
        if file.path().is_dir() {
            let filename = file.file_name();
            let file_string = filename.to_str().unwrap();
            dir_names.push(file_string.to_string())
        } else {
            println!("no directories found");
        }
    }
    compare(dir_names, dry_run);
}

fn compare(dirs: Vec<String>, dry_run: bool) {
    let current_time = Local::now();
    let current_time_fmt = NaiveDate::from_ymd_opt(
        current_time.year(),
        current_time.month(),
        current_time.day(),
    )
    .unwrap()
    .and_hms_opt(
        current_time.hour(),
        current_time.minute(),
        current_time.second(),
    )
    .unwrap();
    for file in dirs {
        //
        let name_as_date = match NaiveDateTime::parse_from_str(&file, "%d-%m-%Y_%H-%M-%S") {
            Ok(name) => name,
            Err(e) => {
                eprintln!("error parsing folder name \"{file}\" to timestamp: {e}");
                continue;
            }
        };
        let difference_from_current: i64 = name_as_date
            .signed_duration_since(current_time_fmt)
            .num_days();
        // basically keep 4 backups: 0 days old, one day old, 3 days old, 7 days old
        if matches!(difference_from_current, 0 | -1 | -3 | -7) {
            println!("keep backup {}", file);
            continue;
        } else {
            if dry_run {
                println!("would have deleted backup {}", file);
            } else {
                let backup_path = read_conf().dirs.backup_dir;
                let long_path = backup_path + &file;
                let file_path = Path::new(&long_path);
                if let Err(e) = fs::remove_dir_all(file_path) {
                    eprintln!("could not delete file {file}: {e}");
                };
                println!("deleted backup {}", file);
            }
        }
    }
}
