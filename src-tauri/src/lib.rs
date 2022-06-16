use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use stm::stm_core::{FromMap, ToMap};
use stm::{FromMap, ToMap};

#[cfg(any(target_os = "linux", target_os = "macos"))]
const HOME: &str = "HOME";
#[cfg(target_os = "windows")]
const HOME: &str = "UserProfile";

#[derive(ToMap, FromMap)]
pub struct Settings {
    music_folder: String,
    download_format: String,
    find_playlist_start: bool,
    auto_playlist_download: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            music_folder: "".to_string(),
            download_format: "{playlist_index} {title}.{ext}".to_string(),
            find_playlist_start: false,
            auto_playlist_download: false,
        }
    }
}

pub fn testing() {
    let s = Settings::get();
    let set = Settings {
        music_folder: "kanker".to_string(),
        ..Default::default()
    };
    Settings::set(set);
}

impl Settings {
    // TODO return Result
    pub fn get() -> Settings {
        let config_file = Settings::get_config_path();
        let file_settings = fs::read_to_string(&config_file).unwrap();
        let mut settings: HashMap<String, Value> = HashMap::new();

        let lines: Vec<&str> = file_settings.split("\n").collect();

        
        for line in lines {
            let mut line = line.split("=");
            let key = line.next().unwrap().to_string();

            if key.len() > 0 {
                let value = line.next().unwrap().trim();
                let val = match value {
                    "true" => Value::new(true),
                    "false" => Value::new(false),
                    _ => Value::new(value.to_owned()),
                };

                settings.insert(key, val);
            }
        }

        Settings::from_map(settings)
    }

    pub fn set(settings: Settings) {
        let config_file = Settings::get_config_path();
        let mut config_file = fs::File::open(&config_file).unwrap();
        let settings: String = Settings::to_map(settings)
            .iter()
            .map(|(k, v)| format!("{}={}\n", k, v))
            .collect();
       
        config_file.write(settings.as_bytes()).unwrap();
    }

    fn get_config_path() -> PathBuf {
        if let Ok(home_folder) = env::var(HOME) {
            let mut config_path = PathBuf::from(home_folder);

            // Config folder
            config_path.push(".config");
            config_path.push("ypd");

            if !config_path.exists() {
                fs::create_dir_all(&config_path).unwrap();

                // Config file
                config_path.push("config");

                let mut config = fs::File::create(&config_path).unwrap();
                let setting_string: String = Settings::to_map(Settings::default())
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect();

                config.write_all(setting_string.as_bytes()).unwrap();
            } else {
                config_path.push("config");
            }
            config_path
        } else {
            panic!("Home folder could not be found!");
        }
    }
}
