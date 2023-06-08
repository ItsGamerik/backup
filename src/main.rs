mod config_toml;
mod retention;

use std::{env, fs, path::Path};

use config_toml::check_and_read::{check_conf, read_conf};
use retention::smart_retention::backupctrl;
use crate::config_toml::check_and_read::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut arg = false;
    if args.iter().any(|arg| arg == "--dry-run") {
        arg = true;
    }
    check_conf();
    let conf_paths: Config = read_conf();
    let backup_dir = Path::new(&conf_paths.dirs.backup_dir);
    let iter = match fs::read_dir(backup_dir) {
        Ok(iter) => iter,
        Err(e) => {
            eprintln!("error while trying to access file: {e}");
            return;
        },
    };
    let mut file_name_vec: Vec<fs::DirEntry> = Vec::new();
    for file in iter {
        if let Some(name) = file.as_ref().unwrap().path().to_str() {
            let file_name = name;
            println!("found file: {}", file_name);
        };
        file_name_vec.push(file.unwrap());
    }
    backupctrl(file_name_vec, arg);
}
