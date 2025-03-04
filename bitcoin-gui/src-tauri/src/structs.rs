use std::sync::Mutex;
use bitcoincore_rpc::Client;

pub struct RpcClient(pub Mutex<Client>);