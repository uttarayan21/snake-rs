extern crate dirs;
extern crate serde;
// extern crate serde_derive;
extern crate toml;
use serde::{Deserialize, Serialize};
// use serde_derive::{Deserialize, Serialize};
use crate::game::Difficulty;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::thread::sleep;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub difficulty: Difficulty,
    pub speed: u32,
}

impl Config {
    pub fn new() -> Result<Self, std::io::Error> {
        let config_path = Config::config_path().unwrap();
        if config_path.exists() {
            let mut config_file = File::open(config_path).unwrap();
            let mut config_file_str: String = String::new();
            config_file.read_to_string(&mut config_file_str).unwrap();
            // let config: Config = toml::from_str(&config_file_str).expect("Error in config file");
            let config: Config = match toml::from_str(&config_file_str) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!(
                        "Error in parsing config file {} due to {}",
                        Config::config_path().unwrap().to_str().unwrap(),
                        e,
                    );
                    eprintln!("Falling back to using the default values");
                    sleep(std::time::Duration::from_millis(3000));
                    Config::default()
                }
            };

            Ok(config)
        } else {
            Ok(Config::default())
        }
    }
    fn default() -> Self {
        Self {
            difficulty: Difficulty::Flat, //Default set to ten blocks per second
            speed: 10,
        }
    }
    pub fn write(&self) {
        let config_file_str: String = toml::to_string(self).unwrap();
        let config_path = Config::config_path().unwrap();
        let mut config_file: File =
            File::create(config_path).expect("Couldn't open config file to write");
        config_file
            .write_all(config_file_str.as_bytes())
            .expect("Couldn't write to file");
    }
    fn config_path() -> Result<PathBuf, std::io::Error> {
        let config_path: PathBuf = match dirs::config_dir() {
            Some(mut path) => {
                path.push("snake");
                path.push("snake.toml");
                path
            }
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "snake.toml not found",
                ));
            }
        };
        Ok(config_path)
    }
}
