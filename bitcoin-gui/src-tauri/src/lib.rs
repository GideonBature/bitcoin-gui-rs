// pub mod blockchain_info;
// pub mod structs;
// pub mod wallet_actions;

// use crate::structs::*;
// use crate::blockchain_info::*;
// use crate::wallet_actions::*;

// use std::sync::Mutex;
// use bitcoincore_rpc::{Auth, Client};

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     let rpc_url = "http://127.0.0.1:18443"; // Replace with your RPC URL
//     let rpc_auth = Auth::UserPass("bene".to_string(), "bene".to_string()); // Replace with your RPC credentials
//     let client = Client::new(rpc_url, rpc_auth).expect("Failed to connect to Bitcoin Core");
//     println!("Client: {:?}", client);

//     tauri::Builder::default()
//         .manage(RpcClient(Mutex::new(client))) // Store the client in Tauri state
//         .invoke_handler(tauri::generate_handler![
//             greet,
//             get_block_count,
//             get_best_block_hash,
//             get_block_hash,
//             get_block,
//             get_raw_mempool,
//             generate_to_address,
//             get_uptime,
//             create_wallet,
//             load_wallet,
//         ])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }


use std::os::unix::net::UnixStream;
use std::io::{Read, Write};
use std::process::Command;
use std::thread;
use serde_json::{json, Value};

const NODE_SOCK_PATH: &str = "/tmp/bitcoin-node.sock";

#[tauri::command]
fn send_rpc(method: String) -> Result<String, String> {
    let mut stream = match UnixStream::connect(NODE_SOCK_PATH) {
        Ok(stream) => stream,
        Err(_) => {
            Command::new("target/debug/bitcoin-node")
                .spawn()
                .map_err(|e| e.to_string())?;
            thread::sleep(std::time::Duration::from_secs(1));
            UnixStream::connect(NODE_SOCK_PATH).map_err(|e| e.to_string())?
        }
    };
    
    let request = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": [],
        "id": 1,
    });
    stream.write_all(request.to_string().as_bytes()).map_err(|e| e.to_string())?;

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).map_err(|e| e.to_string())?;
    let response: Value = serde_json::from_slice(&buffer[..bytes_read]).map_err(|e| e.to_string())?;

    Ok(response["result"].to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    // Store the client in Tauri state
        .invoke_handler(tauri::generate_handler![
            send_rpc,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}