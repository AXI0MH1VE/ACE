#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use axiomhive::{
    model::hybrid_block::{HybridBlock, HybridBlockConfig},
    verification::axiom_checker::{AxiomChecker, C0Signature},
    wasm_generate,
};

#[tauri::command]
fn verified(prompt: &str) -> (String, C0Signature) {
    wasm_generate(prompt)
}

#[tauri::command]
fn creative(prompt: &str) -> String {
    let hybrid = HybridBlock::new(HybridBlockConfig::default());
    hybrid.generate_creative(prompt, 0.9, 64, vec!["text".into()])
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![verified, creative])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
