// use tauri::async_runtime::Mutex;
use std::sync::Mutex;
use bitcoincore_rpc::Client;

use crate::CapnpClients;

pub struct RpcClient(pub Mutex<Client>);
pub struct CapnpRpcClient(pub Mutex<CapnpClients>);

unsafe impl Send for CapnpRpcClient {}
unsafe impl Sync for CapnpRpcClient {}