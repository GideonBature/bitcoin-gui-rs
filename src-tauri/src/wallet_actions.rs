use crate::structs::*;
use tauri::State;
use bitcoincore_rpc::{json::LoadWalletResult, RpcApi};

#[tauri::command]
pub fn create_wallet(client: State<RpcClient>, wallet_name: &str) -> Result<LoadWalletResult, String> {
    let client = client.0.lock().map_err(|_| "Failed to acquire RPC client lock".to_string())?;

    client.create_wallet(&wallet_name, None, None, None, None).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_wallet(client: State<RpcClient>, filename: &str) -> Result<LoadWalletResult, String> {
    let client = client.0.lock().map_err(|_| "Failed to acquire RPC client lock".to_string())?;

    client.load_wallet(&filename).map_err(|e| e.to_string())
}