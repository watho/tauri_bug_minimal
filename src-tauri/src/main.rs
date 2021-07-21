#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  Manager,
  WindowBuilder
};
use url::Url;

fn main() {
  tauri::Builder::default().setup(|app| {
    let mut main_window = app.get_window("main").unwrap();
          println!("creating windows async");
          let url_string = format!("http://localhost:{}", "8080");
          let target_url = Url::parse(&url_string).expect("error");
          println!("Creating window pointing to {}", url_string);
            app.create_window("server".to_string(), tauri::WindowUrl::External(target_url), |window_builder, webview_attributes| {
            (window_builder.title("Serverview").maximized(false).resizable(true).visible(true).inner_size(1024.0, 720.0), webview_attributes)
          }).expect("error creating window");
          main_window.hide().unwrap();
    Ok(())
  }
  
  )
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
