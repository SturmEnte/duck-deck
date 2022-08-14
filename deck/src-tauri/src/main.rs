#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
  tauri::Builder::default()
  .setup(|app| {

    app.listen_global("button-press", |event| {
      let id = event.payload().unwrap().parse::<i32>().unwrap();
      println!("got button press {}", id);
    });
    
    Ok(())
  })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
