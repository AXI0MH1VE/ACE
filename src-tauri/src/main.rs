#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use axiomhive::{verification::axiom_checker::C0Signature, wasm_generate};

#[tauri::command]
fn verified(prompt: &str) -> (String, C0Signature) {
    wasm_generate(prompt)
}

#[tauri::command]
fn creative(prompt: &str) -> String {
    format!("[tauri creative preview] {}", prompt)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![verified, creative])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
