use tauri::Manager;
use std::env;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // Make the app portable: store WebView2 data next to the executable
  if let Ok(exe_path) = env::current_exe() {
    if let Some(exe_dir) = exe_path.parent() {
      let data_dir = exe_dir.join("Aether_Data");
      env::set_var("WEBVIEW2_USER_DATA_FOLDER", data_dir);
    }
  }

  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_libmpv::init())
    .invoke_handler(tauri::generate_handler![close_splashscreen])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn close_splashscreen(app: tauri::AppHandle) {
  if let Some(window) = app.get_webview_window("main") {
    let _ = window.show();
    let _ = window.set_focus();
  }
  if let Some(splash) = app.get_webview_window("splashscreen") {
    let _ = splash.close();
  }
}
