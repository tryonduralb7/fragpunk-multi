use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    aimbot_enabled: bool,
    esp_enabled: bool,
    speedhack_multiplier: f32,
}

impl Config {
    fn new() -> Self {
        Config {
            aimbot_enabled: false,
            esp_enabled: false,
            speedhack_multiplier: 1.0,
        }
    }

    fn load() -> io::Result<Self> {
        let data = fs::read_to_string("config.json")?;
        let config: Config = serde_json::from_str(&data)?;
        Ok(config)
    }

    fn save(&self) -> io::Result<()> {
        let data = serde_json::to_string(self)?;
        let mut file = fs::File::create("config.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

fn main() {
    let mut config = Config::new();
    if let Ok(loaded_config) = Config::load() {
        config = loaded_config;
    }

    if config.aimbot_enabled {
        println!("Aimbot is enabled.");
    }

    if config.esp_enabled {
        println!("ESP is enabled.");
    }

    println!("Speedhack multiplier: {}", config.speedhack_multiplier);
    config.save().unwrap();
}