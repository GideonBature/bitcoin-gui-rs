extern crate dotenv;
use dotenv::dotenv;
use std::env;

use blockconnect;

#[tauri::command]
pub async fn get_chain_tip_height() -> Result<i32, String> {
    dotenv().ok();

    let result = tauri::async_runtime::spawn_blocking(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| e.to_string())?;

        let local = tokio::task::LocalSet::new();

        local.block_on(&rt, async {
            // let socket_path = "/Users/bene/Desktop/bitcoin/datadir_bdk_wallet/regtest/node.sock";
            let socket_path = env::var("SOCKET_PATH")
                .map_err(|e| e.to_string())?;
            blockconnect::query_chain_tip_height(&socket_path).await
        })  
    })
    .await
    .map_err(|e| e.to_string())?;

    match result {
        Ok(height) => Ok(height),
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn get_chain_tip_hash() -> Result<String, String> {
    dotenv().ok();

    let result = tauri::async_runtime::spawn_blocking(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| e.to_string())?;

        let local = tokio::task::LocalSet::new();

        local.block_on(&rt, async {
            let socket_path = env::var("SOCKET_PATH")
                .map_err(|e| e.to_string())?;
            blockconnect::query_chain_tip_hash(&socket_path).await
        })  
    })
    .await
    .map_err(|e| e.to_string())?;

    match result {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn get_chain_hash_by_height(height: i32) -> Result<String, String> {
    dotenv().ok();

    let result = tauri::async_runtime::spawn_blocking(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| e.to_string())?;

        let local = tokio::task::LocalSet::new();

        local.block_on(&rt, async {
            let socket_path = env::var("SOCKET_PATH")
                .map_err(|e| e.to_string())?;
            blockconnect::query_chain_hash_by_height(&socket_path, height).await
        })  
    })
    .await
    .map_err(|e| e.to_string())?;

    match result {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn get_block_by_height(height: i32) -> Result<String, String> {
    dotenv().ok();
    
    let result = tauri::async_runtime::spawn_blocking(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| e.to_string())?;

        let local = tokio::task::LocalSet::new();

        local.block_on(&rt, async {
            let socket_path = env::var("SOCKET_PATH")
                .map_err(|e| e.to_string())?;
            blockconnect::query_chain_block_by_height(&socket_path, height).await
        })  
    })
    .await
    .map_err(|e| e.to_string())?;

    match result {
        Ok(block) => Ok(serde_json::to_string_pretty(&block).map_err(|e| e.to_string())?),
        Err(e) => Err(e),
    }
}