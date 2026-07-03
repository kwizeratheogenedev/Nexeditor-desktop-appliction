#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::{process::{Command, Child}, sync::Mutex};
use tauri::{Manager, RunEvent};

fn spawn_node_backend() -> Option<Child> {
  // Attempt to spawn `node backend/server.js` relative to the app executable cwd.
  // In development the project root will be the workspace; in production the backend
  // should be packaged or the app should point to the installed backend location.
  let node_cmd = if cfg!(target_os = "windows") { "node.exe" } else { "node" };
  let backend_path = "../Nexteditor/backend/server.js"; // relative to `src-tauri/..` during dev

  match Command::new(node_cmd)
    .arg(backend_path)
    .env("NODE_ENV", "production")
    .spawn()
  {
    Ok(child) => {
      println!("Spawned Node backend (pid={})", child.id());
      Some(child)
    }
    Err(e) => {
      eprintln!("Failed to spawn Node backend: {}", e);
      None
    }
  }
}

fn main() {
  // Keep a handle to the child process so we can kill it on exit.
  let backend_child: Mutex<Option<Child>> = Mutex::new(None);

  tauri::Builder::default()
    .setup(move |app| {
      println!("Nexeditor Tauri backend setup starting...");

      // Spawn the node backend. If it fails, app will continue but features depending on it will not work.
      let child = spawn_node_backend();
      *backend_child.lock().unwrap() = child;

      // Expose the API base URL via an environment variable that the web side reads.
      // Backend auto-falls back to port 3001 if 3000 is occupied during local development.
      let api_url = "http://127.0.0.1:3001";
      std::env::set_var("VITE_API_URL", api_url);

      // Also inject a runtime global into the webview window so the app can read it at runtime.
      if let Some(win) = app.get_window("main") {
        let _ = win.eval(&format!("window.__API_URL = '{0}';", api_url));
      }

      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while building tauri application")
    .run(|app_handle, event| match event {
      RunEvent::ExitRequested { api, .. } => {
        // Allow exit
        api.prevent_exit();
        // Kill child then exit
        if let Some(mut child) = backend_child.lock().unwrap().take() {
          let _ = child.kill();
          let _ = child.wait();
        }
        std::process::exit(0);
      }
      _ => {}
    });
}
