pub mod commands;
use commands::*;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_chain_tip_height,
            get_chain_tip_hash,
            get_chain_hash_by_height,
            get_block_by_height,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}