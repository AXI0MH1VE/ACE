#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use axiomhive::{
    generate_verified_output,
    model::hybrid_block::{HybridBlock, HybridBlockConfig},
    verification::axiom_checker::{AxiomSet, C0Signature},
};

#[tauri::command]
fn verified(prompt: &str) -> (String, C0Signature) {
    let axiom_set = AxiomSet {
        name: "tauri_local".into(),
        version: "0.1.0".into(),
        rules: vec![],
    };
    tauri::async_runtime::block_on(generate_verified_output(
        prompt,
        &axiom_set,
        256,
    ))
    .unwrap_or_else(|_| {
        (
            "verification failed".into(),
            C0Signature::empty(),
        )
    })
}

#[tauri::command]
fn creative(prompt: &str) -> String {
    let hybrid = HybridBlock::new(HybridBlockConfig::default());
    hybrid.generate_creative_default(prompt, 0.9, 64, vec!["text".into()])
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![verified, creative])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
