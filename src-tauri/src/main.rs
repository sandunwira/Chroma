// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use window_shadows::set_shadow;
use std::env;
use std::fs;

#[derive(Clone, serde::Serialize)]
struct Payload {
  args: Vec<String>,
  cwd: String,
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();

      // SET SHADOWS =========================================================================== //
      set_shadow(&main_window, true).unwrap();

      // DISABLE RELOAD ======================================================================== //
      main_window.eval("window.addEventListener('keydown', function(e) {if (e.keyCode == 116) { e.preventDefault(); }});").unwrap(); // F5  |  Main
      main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.keyCode == 116) { e.preventDefault(); }});").unwrap(); // CTRL + F5  |  Main
      main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.keyCode == 82) { e.preventDefault(); }});").unwrap(); // CTRL + R  |  Main
      main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.shiftKey && e.keyCode == 82) { e.preventDefault(); }});").unwrap(); // CTRL + SHIFT + R  |  Main
      // DISABLE VIEWING SOURCE ================================================================ //
      main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.keyCode == 85) { e.preventDefault(); }});").unwrap(); // CTRL + U  |  Main
      // DISABLE PRINT ========================================================================= //
      main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.keyCode == 80) { e.preventDefault(); }});").unwrap(); // CTRL + P  |  Main
      main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.shiftKey && e.keyCode == 80) { e.preventDefault(); }});").unwrap(); // CTRL + SHIFT + P  |  Main
      // DISABLE SCREENSHOT ==================================================================== //
      main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.shiftKey && e.keyCode == 83) { e.preventDefault(); }});").unwrap(); // CTRL + SHIFT + S  |  Main
      main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.shiftKey && e.keyCode == 88) { e.preventDefault(); }});").unwrap(); // CTRL + SHIFT + X  |  Main
      // DISABLE DEVELOPER OPTIONS ============================================================= //
      main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.shiftKey && e.keyCode == 73) { e.preventDefault(); }});").unwrap(); // CTRL + SHIFT + I  |  Main
      // DISABLE FIND IN PAGE ================================================================== //
      main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.keyCode == 70) { e.preventDefault(); }});").unwrap(); // CTRL + F  |  Main
      main_window.eval("window.addEventListener('keydown', function(e) {if (e.ctrlKey && e.keyCode == 71) { e.preventDefault(); }});").unwrap(); // CTRL + G  |  Main
      main_window.eval("window.addEventListener('keydown', function(e) {if (e.keyCode == 114) { e.preventDefault(); }});").unwrap(); // F3  |  Main
      // DISABLE CARET BROWSING ================================================================ //
      main_window.eval("window.addEventListener('keydown', function(e) {if (e.keyCode == 118) { e.preventDefault(); }});").unwrap(); // F7  |  Main
      // DISABLE MIDDLE-CLICK TO OPEN LINKS IN NEW WINDOWS ===================================== //
      main_window.eval("window.addEventListener('auxclick', function(e) {if (e.button == 1) { e.preventDefault(); }});").unwrap(); //  |  Main
      Ok(())
    })
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
      println!("{}, {argv:?}, {cwd}", app.package_info().name);
      app.emit_all("single-instance", Payload { args: argv, cwd }).unwrap();
    }))
    .invoke_handler(tauri::generate_handler![open_chromium, uninstall_chromium, get_chromium_version, download_chromium])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
async fn download_chromium(url: String, destination: String) -> Result<(), String> {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use reqwest::Client;
    use zip::ZipArchive;

    // Define the Chromium directory path
    let chromium_path = PathBuf::from("C:\\Chromium Project");

    // Check if the Chromium directory exists and contains files
    if chromium_path.exists() && chromium_path.read_dir().map_or(false, |mut i| i.next().is_some()) {
        // Delete the Chromium directory
        fs::remove_dir_all(&chromium_path).map_err(|e| e.to_string())?;
    }

    // Ensure the destination directory exists
    if !Path::new(&destination).exists() {
        fs::create_dir_all(&destination).map_err(|e| e.to_string())?;
    }

    let client = Client::new();
    let temp_file_path = Path::new(&destination).join("chromium.zip");
    let mut temp_file = File::create(&temp_file_path).map_err(|e| e.to_string())?;

    let mut resp = client.get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?;

    while let Some(chunk) = resp.chunk().await.map_err(|e| e.to_string())? {
        temp_file.write_all(&chunk).map_err(|e| e.to_string())?;
    }

    let file = File::open(&temp_file_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

    // Temporary extraction path
    let temp_extraction_path = Path::new(&destination).join("temp_extraction");
    fs::create_dir_all(&temp_extraction_path).map_err(|e| e.to_string())?;

    archive.extract(&temp_extraction_path).map_err(|e| e.to_string())?;

    // Assuming the zip contains a single top-level folder
    let extracted_folder = fs::read_dir(&temp_extraction_path)
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("Extraction failed: No files found")?
        .map_err(|e| e.to_string())?
        .path();

    // Rename the extracted folder to "Chromium"
    let chromium_path = Path::new(&destination).join("Chromium");
    fs::rename(extracted_folder, &chromium_path).map_err(|e| e.to_string())?;

    // Delete the temporary extraction directory and the zip file
    fs::remove_dir_all(&temp_extraction_path).map_err(|e| e.to_string())?;
    fs::remove_file(&temp_file_path).map_err(|e| e.to_string())?;

    use mslnk::ShellLink;

    let user_profile = env::var("USERPROFILE").map_err(|e| e.to_string())?;
    let desktop_lnk_target = format!(r"{}\Desktop\Chromium.lnk", user_profile);
    let start_lnk_target = format!(r"{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Chromium.lnk", user_profile);
    let exe_target = r"C:\Chromium Project\Chromium\chrome.exe";
    let desktop_sl = ShellLink::new(exe_target).map_err(|e| e.to_string())?;
    let start_sl = ShellLink::new(exe_target).map_err(|e| e.to_string())?;
    desktop_sl.create_lnk(desktop_lnk_target).map_err(|e| e.to_string())?;
    start_sl.create_lnk(start_lnk_target).map_err(|e| e.to_string())?;

    Ok(())
}


#[tauri::command]
fn open_chromium(destination: String) -> Result<(), String> {
    use std::process::Command;
    use std::path::Path;

    // Use the provided destination path directly
    let chromium_path = Path::new(&destination);

    // Check if the chrome.exe path exists
    if !chromium_path.exists() {
        return Err(format!("The specified path does not exist: {:?}", chromium_path));
    }

    // Attempt to launch Chromium
    Command::new(chromium_path)
        .spawn()
        .map_err(|e| format!("Failed to open Chromium: {}", e))?;

    Ok(())
}



#[tauri::command]
async fn get_chromium_version(chromium_path: String) -> Result<String, String> {
    use std::fs;
    use std::path::Path;

    let path = Path::new(&chromium_path);
    if path.is_dir() {
        let entries = fs::read_dir(path)
            .map_err(|e| format!("Failed to read directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("manifest") {
                return Ok(path.file_stem().unwrap().to_str().unwrap().to_string());
            }
        }
        Err("No .manifest file found".into())
    } else {
        Err("Provided path is not a directory".into())
    }
}



#[tauri::command]
async fn uninstall_chromium(chromium_path: String) -> Result<(), String> {
    use std::path::Path;

    // Attempt to remove the directory at the provided path
    if Path::new(&chromium_path).exists() {
        fs::remove_dir_all(&chromium_path).map_err(|e| e.to_string())?;
    } else {
        println!("Chromium directory at {} does not exist.", chromium_path);
    }

    // Construct the path to the Chromium directory in the user's AppData
    let user_profile = env::var("USERPROFILE").map_err(|e| e.to_string())?;
    let chromium_appdata_path = format!(r"{}\AppData\Local\Chromium", user_profile);

    // Attempt to remove the Chromium directory from AppData
    if Path::new(&chromium_appdata_path).exists() {
        fs::remove_dir_all(&chromium_appdata_path).map_err(|e| e.to_string())?;
    } else {
        println!("Chromium AppData directory does not exist.");
    }

    let desktop_shortcut = format!(r"{}\Desktop\Chromium.lnk", user_profile);
    if Path::new(&desktop_shortcut).exists() {
        fs::remove_file(desktop_shortcut).map_err(|e| e.to_string())?;
    } else {
        println!("Desktop shortcut does not exist.");
    }

    let start_menu_shortcut = format!(r"{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Chromium.lnk", user_profile);
    if Path::new(&start_menu_shortcut).exists() {
        fs::remove_file(start_menu_shortcut).map_err(|e| e.to_string())?;
    } else {
        println!("Start Menu shortcut does not exist.");
    }

    Ok(())
}