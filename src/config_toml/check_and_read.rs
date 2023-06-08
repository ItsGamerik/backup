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

fn create_default() -> Config {
    Config {
        dirs: Keys {
            backup_dir: "".to_string(),
            source_dir: "".to_string(),
        },
    }
}

pub fn check_conf() {
    let config = create_default();

    let conf_path = Path::new("./config");
    let conf_file = Path::new("./config/config.toml");
    if let Err(e) = fs::create_dir_all(conf_path) {
        // TODO: change error message
        eprintln!("error: {}", e);
        return;
    }

    if !conf_file.exists() {
        match File::create(conf_file) {
            Ok(mut file) => {
                let toml = toml::to_string(&config).unwrap();
                if let Err(e) = writeln!(file, "{toml}") {
                    // TODO: change error message
                    eprintln!("error: {}", e);
                }
            }
            Err(e) => eprintln!("could not read file: {}", e),
        }
    }
}

pub fn read_conf() -> Config {
    let config_default = create_default();
    let config_path = Path::new("./config/config.toml");
    let config_str: String = fs::read_to_string(config_path).unwrap();
    let config_content: Config = match toml::from_str(&config_str) {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("error reading from file: {}", e);
            let mut conf_file = File::create(config_path).unwrap();
            if let Err(e) = writeln!(conf_file, "{}", toml::to_string(&config_default).unwrap()) {
                eprintln!("error writing to file: {}", e)
            };
            // config_default
            panic!("error: no config provided");
        }
    };
    // dbg!("{:#?}", &config_content);
    config_content
}
