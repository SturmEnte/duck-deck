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

      let app_handle = app.app_handle();

      app.listen_global("settings", move |event| {
        match event.payload().unwrap() {
          // Toggle fullscreen
          "1" => {
            println!("Toggle fullscreen");
            app_handle.windows().iter().for_each(|obj| {
              obj.1.set_fullscreen(!obj.1.is_fullscreen().unwrap()).unwrap();
            });
          },
          &_ => println!("Receibed unknown setting id: {}", event.payload().unwrap())
        };
      });

      app.listen_global("button_press", move |event| {
        let mut stream = TcpStream::connect(SERVER).unwrap();
        let success:bool = match stream.write(event.payload().unwrap().as_bytes()) {
          Ok(_res) => true,
          Err(_err) => false
        };

        if success {
          println!("Sent the button press to the receiver");
        } else {
          println!("Error while sending the button press to the receiver");
        }

        stream.flush().unwrap();
        stream.shutdown(std::net::Shutdown::Both).unwrap();
        println!("Button press id: {}", event.payload().unwrap());
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}