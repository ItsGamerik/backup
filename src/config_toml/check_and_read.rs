use core::panic;
use std::io::Write;
use std::{
    fs::{self, File},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Config {
    pub dirs: Keys,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct Keys {
    pub backup_dir: String,
    pub source_dir: String,
}

impl Config {
    fn new() -> Config {
        Config {
            dirs: Keys {
                backup_dir: "CHANGE_ME".to_string(),
                source_dir: "CHANGE_ME".to_string(),
            },
        }
    }
}

pub fn check_conf(conf_file: &Path) {
    let config = Config::new();
    let conf_path = Path::new("./config");
    // let conf_file = Path::new("./config/config.toml");
    if let Err(e) = fs::create_dir_all(conf_path) {
        eprintln!("could not create config folder: {}", e);
        return;
    }

    if !conf_file.exists() {
        match File::create(conf_file) {
            Ok(mut file) => {
                let toml = toml::to_string(&config).unwrap();
                if let Err(e) = writeln!(file, "{toml}") {
                    eprintln!("could not write default config: {}", e);
                }
            }
            Err(e) => eprintln!("could not read file: {}", e),
        }
    }
}

pub fn read_conf(conf_file: &Path) -> Config {
    let config_default = Config::new();
    // let config_file = Path::new("./config/config.toml");
    let config_str: String = fs::read_to_string(conf_file).unwrap();
    let config_content: Config = match toml::from_str(&config_str) {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("error reading from file: {}", e);
            // TODO: add cmdline argument for config file
            let mut conf_file = match File::create(conf_file) {
                Ok(file) => file,
                Err(e) => panic!("an error occured while opening the config file: {}", e),
            };
            if let Err(e) = writeln!(conf_file, "{}", toml::to_string(&config_default).unwrap()) {
                eprintln!("error writing to file: {}", e)
            };
            // config_default
            panic!("error: no config provided");
        }
    };
    config_content
}
