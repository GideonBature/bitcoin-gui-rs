use std::sync::Arc;

mod chain;
mod connection;
mod error;
mod generated;
mod notification;

use std::time::Duration;
use tokio::time;
use bitcoin::{Block, BlockHash};

pub use chain::ChainInterface;
pub use connection::Connection;
pub use error::BlockConnectError;
pub use generated::*;
pub use notification::ChainNotification;
pub use notification::NotificationHandler;

#[derive(Clone)]
/// Main entry point for blockchain interaction
pub struct BlockConnect {
    connection: Arc<Connection>,
    chain: Arc<ChainInterface>,
}

impl BlockConnect {
    /// Initialize BlockConnect by connecting to a Bitcoin node through the specified socket
    pub async fn init(socket_path: &str) -> Result<Self, BlockConnectError> {
        log::info!("Initializing BlockConnect with socket path: {}", socket_path);
        let connection = Connection::connect(socket_path).await?;
        let chain = Arc::new(ChainInterface::new(connection.clone()));
        log::info!("BlockConnect initialized successfully");

        Ok(Self { connection, chain })
    }

    /// Get a reference to the chain interface
    pub fn chain(&self) -> &Arc<ChainInterface> {
        &self.chain
    }

    /// Disconnect from the node
    pub async fn disconnect(self) -> Result<(), BlockConnectError> {
        // Check if we're the last owner of the connection
        match Arc::try_unwrap(self.connection) {
            Ok(conn) => conn.disconnect().await,
            Err(_) => {
                // There are other references to the connection still alive
                Ok(())
            }
        }
    }
}


pub async fn query_chain_tip_height(socket_path: &str) -> Result<i32, String> {
    let bc = BlockConnect::init(socket_path).await.map_err(|e| format!("Connection error: {}", e))?;
    let chain = bc.chain();

    match time::timeout(Duration::from_secs(5), chain.get_tip_height()).await {
        Ok(Ok(height)) => Ok(height),
        Ok(Err(e)) => Err(format!("Error fetching tip: {}", e)),
        Err(_) => Err("Request timed out".into()),
    }
}

pub async fn query_chain_tip_hash(socket_path: &str) -> Result<BlockHash, String> {
    let bc = BlockConnect::init(socket_path).await.map_err(|e| format!("Connection error: {}", e))?;
    let chain = bc.chain();

    match time::timeout(Duration::from_secs(5), chain.get_tip_hash()).await {
        Ok(Ok(hash)) => Ok(hash),
        Ok(Err(e)) => Err(format!("Error fetching tip: {}", e)),
        Err(_) => Err("Request timed out".into()),
    }
}

pub async fn query_chain_hash_by_height(socket_path: &str, height: i32) -> Result<BlockHash, String> {
    let bc = BlockConnect::init(socket_path).await.map_err(|e| format!("Connection error: {}", e))?;
    let chain = bc.chain();

    match time::timeout(Duration::from_secs(5), chain.get_block_hash_at_height(height)).await {
        Ok(Ok(hash)) => Ok(hash.expect("Block hash not found for given height")),
        Ok(Err(e)) => Err(format!("Error fetching tip: {}", e)),
        Err(_) => Err("Request timed out".into()),
    }
}

pub async fn query_chain_block_by_height(socket_path: &str, height: i32) -> Result<Block, String> {
    let bc = BlockConnect::init(socket_path).await.map_err(|e| format!("Connection error: {}", e))?;
    let chain = bc.chain();

    match time::timeout(Duration::from_secs(5), chain.get_block(height)).await {
        Ok(Ok(block)) => Ok(block),
        Ok(Err(e)) => Err(format!("Error fetching tip: {}", e)),
        Err(_) => Err("Request timed out".into()),
    }
}