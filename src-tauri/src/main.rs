// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn adder(num1: u16, num2: u16) -> String {
    println!("HIHIHIHIHIH");
    let num3 = num1 + num2;
    format!("this is you number {}", num3)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![adder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
