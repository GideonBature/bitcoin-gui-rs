pub mod blockchain_info;
pub mod structs;
pub mod wallet_actions;

use crate::structs::*;
use crate::blockchain_info::*;
use crate::wallet_actions::*;

use std::sync::Mutex;
use bitcoincore_rpc::{Auth, Client};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let rpc_url = "http://127.0.0.1:18443"; // Replace with your RPC URL
    let rpc_auth = Auth::UserPass("bene".to_string(), "bene".to_string()); // Replace with your RPC credentials
    let client = Client::new(rpc_url, rpc_auth).expect("Failed to connect to Bitcoin Core");
    println!("Client: {:?}", client);

    tauri::Builder::default()
        .manage(RpcClient(Mutex::new(client))) // Store the client in Tauri state
        .invoke_handler(tauri::generate_handler![
            greet,
            get_block_count,
            get_best_block_hash,
            get_block_hash,
            get_block,
            get_raw_mempool,
            generate_to_address,
            get_uptime,
            create_wallet,
            load_wallet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}