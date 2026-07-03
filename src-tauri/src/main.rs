#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let handle = app.handle();
      // Placeholder: we can spawn a Node backend or other services here.
      // Example: spawn a child process that runs the Node backend bundled with the app.
      println!("Nexeditor Tauri backend setup");
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
