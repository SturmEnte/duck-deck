// Note:
// This version of the main file might be slow. I am working on a faster version, but have not found a working way yet.
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod config;
mod logger;

use std::net::TcpStream;
use std::sync::Mutex;
use std::io::{Read, Write};
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;
use std::fs::create_dir_all;

use tauri::{Manager, AppHandle, CustomMenuItem, Menu, Submenu};
use once_cell::sync::Lazy;

use config::Config;
use logger::Logger;

const SERVER: &str = "localhost:3030";
const CONFIG_PATH: &str = "config";
const MAIN_WINDOW_LABEL: &str = "main"; // I'm not sure if this is always the case, but at this point I don't have enough mental capacity left to find this out

static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
  let config = Config::new(CONFIG_PATH);
  Mutex::new(config)
});

static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| {

  {
    let path = Path::new("logs");
    if !path.exists() {
      create_dir_all("logs").unwrap();
    }
  }

  let logger = Logger::new(format!("logs/{}.log", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()).as_str());
  Mutex::new(logger)
});

fn main() {

  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let fullscreen = CustomMenuItem::new("fullscreen".to_string(), "Fullscreen");
  let settings_submenu = Submenu::new("Settings", Menu::new().add_item(fullscreen));

  let menu = Menu::new()
    .add_item(quit)
    .add_submenu(settings_submenu);

  tauri::Builder::default()
    .setup(|app| {
      let app_handle = app.app_handle();

      let window = app.get_window(MAIN_WINDOW_LABEL).unwrap();
      if CONFIG.lock().unwrap().fullscreen {
        window.set_fullscreen(true).unwrap();
        window.menu_handle().hide().unwrap();
      }
      window.set_focus().unwrap();

      app.listen_global("settings", move |event| {
        match event.payload().unwrap() {
          "1" => {
            toggle_fullscreen(&app_handle);
          },
          &_ => LOGGER.lock().unwrap().log(format!("Receibed unknown setting id: {}", event.payload().unwrap()).as_str())
        };
      });

      app.listen_global("button_press", move |event| {
        LOGGER.lock().unwrap().log(format!("Button press id: {}", event.payload().unwrap()).as_str());

        let mut stream = TcpStream::connect(SERVER).unwrap();
        let success:bool = match stream.write(event.payload().unwrap().as_bytes()) {
          Ok(_res) => true,
          Err(_err) => false
        };

        if success {
          LOGGER.lock().unwrap().log("Sent the button press to the receiver");
        } else {
          LOGGER.lock().unwrap().log("Error while sending the button press to the receiver");
        }

        let mut buffer = [0; 1];
        let response: String;
        match stream.read(&mut buffer) {
          Ok(_) => {
            response = String::from_utf8_lossy(&buffer).to_string();
          },
          Err(err) => {
            LOGGER.lock().unwrap().log(format!("Error while receiving response. Error: \n{err}").as_str());
            return;
          }
        }

        match response.as_str() {
          "0" => {
            LOGGER.lock().unwrap().log("Successfully executed");
          },
          _ => {
            LOGGER.lock().unwrap().log(format!("Unknown response from receiver: {response}").as_str());
          }
        }

        stream.flush().unwrap();
        stream.shutdown(std::net::Shutdown::Both).unwrap();
      });

      Ok(())
    })
    .menu(menu)
    .on_menu_event(|event| {
      match event.menu_item_id() {
        "quit" => {
          std::process::exit(0);
        }
        "fullscreen" => {
          toggle_fullscreen(&event.window().app_handle());
        }
        _ => {}
      }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn toggle_fullscreen(app_handle: &AppHandle) {
  LOGGER.lock().unwrap().log("Toggle fullscreen");
  app_handle.windows().iter().for_each(|obj| {
    obj.1.menu_handle().toggle().unwrap();
    obj.1.set_fullscreen(!obj.1.is_fullscreen().unwrap()).unwrap();
    CONFIG.lock().unwrap().fullscreen = obj.1.is_fullscreen().unwrap();
    CONFIG.lock().unwrap().save();
  });
}