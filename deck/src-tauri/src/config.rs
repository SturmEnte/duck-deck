use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub struct Config {
    file_path: String,
    pub fullscreen: bool,
    pub receiver_url: String
}

impl Config {
    pub fn new(file_path: &str) -> Config {
        // Defualt config
        let mut config = Config {
            file_path: file_path.to_string(),
            fullscreen: false,
            receiver_url: "localhost:3000".to_string(),
        };

        // Load config from file (if exists) and overwrite all config values, that are inside the config
        if Path::new(file_path).exists() {
            let file = File::open(file_path).unwrap();
            let reader = BufReader::new(file);

            // Read the file line by line
            for (_index, line) in reader.lines().enumerate() {
                let line = line.unwrap();
                let split = line.split(":");
                
                let mut i = 0;
                let mut key = "";
                let mut value = String::from("");

                for s in split {
                    if i == 0 {
                        key = s;
                    }else {
                        value.push_str(s);
                        if i > 1 {
                            value.push_str(":");
                        } 
                    }

                    i += 1;
                }

                match key {
                    "fullscreen" => {
                        if value == "true" {
                            config.fullscreen = true;
                        } else if value == "false" {
                            config.fullscreen = false;
                        } else {
                            println!("Invalid value of fullscreen: {}", value);
                        }
                    },
                    "receiver_url" => {
                        config.receiver_url = value;
                    },
                    &_ => println!("Unknown config key: {}", key) 
                };
            }
        }

        config
    }

    pub fn save(&mut self) {

        let mut file_string = String::from("");

        file_string.push_str("receiver_url:");
        file_string.push_str(&self.receiver_url);
        file_string.push_str("\nfullscreen:");
        file_string.push_str(&self.fullscreen.to_string().as_str());

        if Path::new(self.file_path.as_str()).exists() {
            let mut f = std::fs::OpenOptions::new().write(true).truncate(true).open(self.file_path.as_str()).unwrap();
            f.write_all(file_string.as_bytes()).unwrap();
            f.flush().unwrap();
        } else {
            let mut file = File::create(self.file_path.as_str()).unwrap();
            file.write_all(file_string.as_bytes()).unwrap();
        }
    }
}

