use crate::structs::*;

use serde_json::Value;
use tauri::State;
use bitcoin::Address;
use bitcoin::address::NetworkUnchecked;
use bitcoincore_rpc::RpcApi;


#[tauri::command]
pub fn get_block_count(client: State<RpcClient>) -> Result<u64, String> {
    let client = client.0.lock().unwrap();
    let count = client.get_block_count().map_err(|e| e.to_string()).unwrap();
    Ok(count)
}

#[tauri::command]
pub fn get_best_block_hash(client: State<RpcClient>) -> Result<String, String> {
    let client = client.0.lock().unwrap();
    client.get_best_block_hash().map(|hash| hash.to_string()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_block_hash(client: State<RpcClient>, height: u64) -> Result<String, String> {
    let client = client.0.lock().unwrap();
    client.get_block_hash(height).map(|hash| hash.to_string()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_block(client: State<RpcClient>, height: u64) -> Result<Value, String> {
    let client = client.0.lock().unwrap();
    let hash = client.get_block_hash(height).map_err(|e| e.to_string())?;
    client.get_block(&hash).map(|block| serde_json::to_value(block).unwrap()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_raw_mempool(client: State<RpcClient>) -> Result<Vec<String>, String> {
    let client = client.0.lock().unwrap();
    client.get_raw_mempool().map(|txids| txids.into_iter().map(|txid| txid.to_string()).collect()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn generate_to_address(client: State<RpcClient>, address: Address<NetworkUnchecked>, num_blocks: u64) -> Result<Vec<String>, String> {
    let client = client.0.lock().unwrap();
    client.generate_to_address(num_blocks, &address.assume_checked_ref()).map(|hashes| hashes.into_iter().map(|hash| hash.to_string()).collect()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_uptime(client: State<RpcClient>) -> Result<u64, String> {
    let client = client.0.lock().unwrap();
    client.uptime().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}