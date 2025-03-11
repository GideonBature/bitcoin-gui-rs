pub mod chain_capnp {
    include!(concat!(env!("OUT_DIR"), "/chain_capnp.rs"));
}
pub mod common_capnp {
    include!(concat!(env!("OUT_DIR"), "/common_capnp.rs"));
}
pub mod echo_capnp {
    include!(concat!(env!("OUT_DIR"), "/echo_capnp.rs"));
}
pub mod handler_capnp {
    include!(concat!(env!("OUT_DIR"), "/handler_capnp.rs"));
}
pub mod init_capnp {
    include!(concat!(env!("OUT_DIR"), "/init_capnp.rs"));
}
pub mod node_capnp {
    include!(concat!(env!("OUT_DIR"), "/node_capnp.rs"));
}
pub mod proxy_capnp {
    include!(concat!(env!("OUT_DIR"), "/proxy_capnp.rs"));
}
pub mod wallet_capnp {
    include!(concat!(env!("OUT_DIR"), "/wallet_capnp.rs"));
}
pub mod chain;
pub mod init;
pub mod node;
pub mod wallet;


pub mod blockchain_info;
pub mod structs;
pub mod wallet_actions;

// use crate::structs::*;
use std::path::Path;
use structs::CapnpRpcClient;

// use crate::wallet_actions::*;
use crate::blockchain_info::*;

use std::sync::Mutex;
// use futures_util::lock::Mutex;
// use tauri::async_runtime::Mutex;
// use tokio::sync::Mutex;
// use bitcoincore_rpc::{Auth, Client};

pub struct CapnpClients {
    pub init_client: init_capnp::init::Client,
    pub chain_client: chain_capnp::chain::Client,
    pub node_client: node_capnp::node::Client,
    // pub wallet_loader: wallet_capnp::wallet::Client,
    pub thread_client: proxy_capnp::thread::Client,
}

// Initialize Cap'n Proto clients during application startup
async fn initialize_clients(socket_path: &str) -> Result<CapnpClients, String> {
    // Set up connection to Bitcoin Core's Cap'n Proto interface
    let (init_client, thread_client) = init::setup_connection(Path::new(socket_path)).await.map_err(|e| format!("Failed to create chain client: {}", e))?;

    // Create Chain Client
    let chain_client = chain::create_chain_client(&init_client, &thread_client).await.map_err(|e| format!("Failed to create chain client: {}", e))?;
    
    // Create Node Client
    let node_client = node::create_node_client(&init_client, &thread_client).await.map_err(|e| format!("Failed to create node client: {}", e))?;
    
    // Create Wallet Loader
    // let wallet_loader = wallet::create_wallet_loader_client(&node_client, &thread_client).await.map_err(|e| format!("Failed to create wallet loader: {}", e))?;
    
    Ok(CapnpClients {
        init_client,
        chain_client,
        node_client,
        // wallet_loader,
        thread_client,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // let rpc_url = "http://127.0.0.1:18443"; // Replace with your RPC URL
    // let rpc_auth = Auth::UserPass("bene".to_string(), "bene".to_string()); // Replace with your RPC credentials
    // let client = Client::new(rpc_url, rpc_auth).expect("Failed to connect to Bitcoin Core");
    // println!("Client: {:?}", client);
    // /Users/bene/Desktop/bitcoin/datadir_bdk_wallet/regtest/node.sock
    // /Users/bene/Desktop/bitcoin/datadir/regtest/node.sock
    let socket_path = "/Users/bene/Desktop/bitcoin/datadir_bdk_wallet/regtest/node.sock";
    // let runtime = tokio::runtime::Runtime::new().unwrap();
    // let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    
    let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();

    let local_set = tokio::task::LocalSet::new();

    local_set.block_on(&runtime, async move {
        let clients = initialize_clients(socket_path).await.expect("Failed to initialize Cap'n Proto clients");

        tauri::Builder::default()
            .manage(CapnpRpcClient(Mutex::new(clients)))
            .invoke_handler(tauri::generate_handler![
                get_block_count,
                get_block_hash,
                get_block,
            ])
            .run(tauri::generate_context!())
            .expect("Error while running tauri application");
    })
    
    // let clients = runtime.block_on(initialize_clients(socket_path)).expect("Failed to initialize Cap'n Proto clients");

    // tauri::Builder::default()
    //     .manage(CapnpRpcClient(Mutex::new(clients))) // Store the client in Tauri state
    //     .invoke_handler(tauri::generate_handler![
    //         get_block_count,
    //         get_block_hash,
    //         get_block,
    //         // get_best_block_hash,
    //         // get_block,
    //         // get_raw_mempool,
    //         // generate_to_address,
    //         // get_uptime,
    //         // create_wallet,
    //         // load_wallet,
    //     ])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");

}