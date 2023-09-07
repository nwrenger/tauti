// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![vigenere])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn vigenere(message: &str, key: &str, encode: bool) -> String {
    message
        .chars()
        .zip(key.trim().chars().cycle())
        .map(|x| {
            let (c, k) = x;
            let shift = (k.to_ascii_lowercase() as u8 - b'a') as i32;
            let operation = if encode { shift } else { -shift };

            match c {
                'a'..='z' => {
                    (((c as u8 - b'a') as i32 + operation).rem_euclid(26) as u8 + b'a') as char
                }
                'A'..='Z' => {
                    (((c as u8 - b'A') as i32 + operation).rem_euclid(26) as u8 + b'A') as char
                }
                _ => c,
            }
        })
        .collect()
}
