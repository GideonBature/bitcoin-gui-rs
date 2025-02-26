use bitcoincore_rpc::{Auth, Client, RpcApi};
use serde_json::Value;
use tauri::State;
use std::sync::Mutex;
use bitcoin::Address;
use bitcoin::address::NetworkUnchecked;

// Define a struct to hold the RPC client
struct RpcClient(Mutex<Client>);

#[tauri::command]
fn get_block_count(client: State<RpcClient>) -> Result<u64, String> {
    let client = client.0.lock().unwrap();
    println!("client (get_block_count): {:?}", client);
    let count = client.get_block_count().map_err(|e| e.to_string()).unwrap();
    println!("Count: {}", count);
    Ok(count)
}

#[tauri::command]
fn get_best_block_hash(client: State<RpcClient>) -> Result<String, String> {
    let client = client.0.lock().unwrap();
    client.get_best_block_hash().map(|hash| hash.to_string()).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_block_hash(client: State<RpcClient>, height: u64) -> Result<String, String> {
    let client = client.0.lock().unwrap();
    client.get_block_hash(height).map(|hash| hash.to_string()).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_block(client: State<RpcClient>, height: u64) -> Result<Value, String> {
    let client = client.0.lock().unwrap();
    let hash = client.get_block_hash(height).map_err(|e| e.to_string())?;
    client.get_block(&hash).map(|block| serde_json::to_value(block).unwrap()).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_raw_mempool(client: State<RpcClient>) -> Result<Vec<String>, String> {
    let client = client.0.lock().unwrap();
    client.get_raw_mempool().map(|txids| txids.into_iter().map(|txid| txid.to_string()).collect()).map_err(|e| e.to_string())
}

#[tauri::command]
fn generate_to_address(client: State<RpcClient>, address: Address<NetworkUnchecked>, num_blocks: u64) -> Result<Vec<String>, String> {
    let client = client.0.lock().unwrap();
    client.generate_to_address(num_blocks, &address.assume_checked_ref()).map(|hashes| hashes.into_iter().map(|hash| hash.to_string()).collect()).map_err(|e| e.to_string())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let rpc_url = "http://127.0.0.1:18443"; // Replace with your RPC URL
    let rpc_auth = Auth::CookieFile("/Users/bene/Library/Application Support/Bitcoin/regtest/.cookie".into()); // Replace with your RPC credentials
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
            generate_to_address
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
