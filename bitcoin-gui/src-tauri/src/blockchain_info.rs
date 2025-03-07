use crate::{chain::*, structs::*};
use tokio::runtime::Runtime;

use tauri::State;
use bitcoin::{Block, Address};
use bitcoin::address::NetworkUnchecked;
use bitcoincore_rpc::RpcApi;

use crate::{
    chain::query_chain_block_height,
    structs::CapnpRpcClient,
};


#[tauri::command]
pub fn get_block_count(clients: State<CapnpRpcClient>) -> Result<u64, String> {
    let clients = clients.0.lock().map_err(|_| "Failed to acquire CapnpRpcClient lock".to_string())?;
    let runtime = Runtime::new().map_err(|e| e.to_string())?;

    let height = runtime.block_on(async {
        query_chain_block_height(&clients.chain_client, &clients.thread_client).await
    }).unwrap();

    Ok(height as u64)
}

#[tauri::command]
pub fn get_block_hash(clients: State<CapnpRpcClient>, height: i32) -> Result<String, String> {
    let client = clients.0.lock().map_err(|_| "Failed to acquire CapnpRpcClient lock".to_string())?;
    let runtime = Runtime::new().map_err(|e| e.to_string())?;

    let hash = runtime.block_on(async {
        query_chain_block_hash_at_height(&client.chain_client, &client.thread_client, height).await
    }).unwrap();
    Ok(hash)
}

#[tauri::command]
pub fn get_block(clients: State<CapnpRpcClient>, node_tip_hash: &bitcoin::BlockHash, height: i32) -> Result<Block, String> {
    let client = clients.0.lock().map_err(|_| "Failed to acquire CapnpRpcClient lock".to_string())?;
    let runtime = Runtime::new().map_err(|e| e.to_string())?;

    let hash = runtime.block_on(async {
        query_chain_block(&client.chain_client, &client.thread_client, node_tip_hash, height).await
    }).unwrap();
    Ok(hash)
}

// #[tauri::command]
// pub fn get_raw_mempool(client: State<RpcClient>) -> Result<Vec<String>, String> {
//     let client = client.0.lock().unwrap();
//     client.get_raw_mempool().map(|txids| txids.into_iter().map(|txid| txid.to_string()).collect()).map_err(|e| e.to_string())
// }

// #[tauri::command]
// pub fn generate_to_address(client: State<RpcClient>, address: Address<NetworkUnchecked>, num_blocks: u64) -> Result<Vec<String>, String> {
//     let client = client.0.lock().unwrap();
//     client.generate_to_address(num_blocks, &address.assume_checked_ref()).map(|hashes| hashes.into_iter().map(|hash| hash.to_string()).collect()).map_err(|e| e.to_string())
// }

// #[tauri::command]
// pub fn get_uptime(client: State<RpcClient>) -> Result<u64, String> {
//     let client = client.0.lock().unwrap();
//     client.uptime().map_err(|e| e.to_string())
// }

// #[tauri::command]
// pub fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }