extern crate serde_json;

use std::collections::HashMap;
use std::{net::TcpListener, io::Read};
use enigo::{Enigo, Key, KeyboardControllable};
use serde_json::Value as JsonValue;
use std::fs::File;
use std::path::Path;

const CONFIG_PATH: &str = "config.json";

fn main() {
    let mut enigo = Enigo::new();
    let listener = TcpListener::bind("localhost:3030").unwrap();

    let config = match load_config() {
        Ok(config) => config,
        Err(error) => panic!("{}", error),
    };

    let mut actions: HashMap<String, Action> = HashMap::new();
    
    // The program will crash, when the first layer of the config isn't an array
    let conifg_array = config.as_array().unwrap();
    conifg_array.iter().for_each(|obj| {
        let id = obj["id"].as_str().unwrap().to_string();
        let action_type = obj["action"]["type"].as_str().unwrap();

        match action_type {
            "key-press" => {
                let key = obj["action"]["key"].as_str().unwrap().to_string();
                let alt = obj["action"]["alt"].as_bool().unwrap();
                let ctrl = obj["action"]["ctrl"].as_bool().unwrap();
                let shift = obj["action"]["shift"].as_bool().unwrap();

                actions.insert(id, {Action { key: key, alt: alt, ctrl: ctrl, shift: shift }});
            },
            _ => {
                panic!("Unknown action type");
            }
        }

    });

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        
        let mut buffer = [0; 34];
        let size = stream.read(&mut buffer).unwrap();
        let id = String::from_utf8_lossy(&buffer[..size]);
        println!("Id: {}", id);

        if id == "1" {
            enigo.key_click(Key::Layout('^'));
        }
    }
}

fn load_config() -> Result<JsonValue, String> {
    if !Path::new(CONFIG_PATH).exists() {
        return Err("Cofig does not exist".to_string());
    }
    
    let mut file = match File::open(CONFIG_PATH) {
        Ok(file) => file,
        Err(error) => return Err(error.to_string()), 
    };

    let mut json_str  = String::new();
    file.read_to_string(&mut json_str).unwrap();

    let res = serde_json::from_str(json_str.as_str());

    if res.is_ok() {
        return Ok(res.unwrap());
    } else {
        return Err(res.unwrap_err().to_string());
    }
}

struct Action {
    key: String,
    alt: bool,
    ctrl: bool,
    shift: bool
}