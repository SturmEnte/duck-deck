// Note:
// This version of the main file might be slow. I am working on a faster version, but have not found a working way yet.

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::net::TcpStream;
use std::io::Write;
use tauri::Manager;

const SERVER: &str = "localhost:3030";

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      
      app.listen_global("button_press", move |event| {
        let mut stream = TcpStream::connect(SERVER).unwrap();
        stream.write(event.payload().unwrap().as_bytes()).unwrap();
        stream.flush().unwrap();
        stream.shutdown(std::net::Shutdown::Both).unwrap();
        println!("Button press id: {}", event.payload().unwrap());
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}