mod config_toml;
mod retention;

use std::{env, fs, path::Path};

use crate::config_toml::check_and_read::Config;
use config_toml::check_and_read::{check_conf, read_conf};
use retention::smart_retention::backupctrl;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file_arg = match args.get(1) {
        Some(arg) => {
            println!("using config path from command line: {}", arg);
            arg
        },
        None => "./config/config.toml",
    };
    let config_file_path = Path::new(config_file_arg);
    let mut arg = false;
    if args.iter().any(|arg| arg == "--dry-run") {
        arg = true;
    }
    check_conf(config_file_path);
    let conf_paths: Config = read_conf(config_file_path);
    let backup_dir = Path::new(&conf_paths.dirs.backup_dir);
    let iter = match fs::read_dir(backup_dir) {
        Ok(iter) => iter,
        Err(e) => {
            eprintln!("error while trying to access file: {e}");
            return;
        }
    };
    let mut file_name_vec: Vec<fs::DirEntry> = Vec::new();
    for file in iter {
        if let Some(name) = file.as_ref().unwrap().path().to_str() {
            let file_name = name;
            println!("found file: {}", file_name);
        };
        file_name_vec.push(file.unwrap());
    }
    backupctrl(file_name_vec, arg, config_file_path);
}
