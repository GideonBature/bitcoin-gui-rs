use bitcoin::consensus::Decodable;
use bitcoin::hashes::Hash;
use bitcoin::{Block, BlockHash};
use std::sync::Arc;
use std::sync::Mutex;

use crate::{
    chain_capnp::chain::Client as ChainClient,
    notification::{ChainNotificationHandler, NotificationHandler},
    proxy_capnp::thread::Client as ThreadClient,
    BlockConnectError, Connection,
};

pub struct ChainInterface {
    chain_client: ChainClient,
    thread: ThreadClient,
    notification_handler: Arc<Mutex<ChainNotificationHandler>>,
}

impl ChainInterface {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self {
            chain_client: connection.chain_client().clone(),
            thread: connection.thread().clone(),
            notification_handler: Arc::new(Mutex::new(ChainNotificationHandler::new())),
        }
    }

    pub fn from_client(chain_client: ChainClient, thread: ThreadClient) -> Self {
        Self {
            chain_client,
            thread,
            notification_handler: Arc::new(Mutex::new(ChainNotificationHandler::new())),
        }
    }

    pub async fn register_handler(&self, handler: Arc<dyn NotificationHandler>) {
        let mut notification_handler = self.notification_handler.lock().unwrap();
        notification_handler.register_handler(handler).await;
    }

    pub fn notification_handler(&self) -> Arc<Mutex<ChainNotificationHandler>> {
        self.notification_handler.clone()
    }

    pub async fn subscribe_to_notifications(&self) -> Result<(), BlockConnectError> {
        log::debug!("Subscribing to chain notifications");

        let handler = self.notification_handler.lock().unwrap().clone();
        let notification_client = capnp_rpc::new_client(handler);

        let mut handle_req = self.chain_client.handle_notifications_request();

        handle_req
            .get()
            .get_context()?
            .set_thread(self.thread.clone());

        handle_req.get().set_notifications(notification_client);

        let _ = handle_req.send().promise.await?;

        log::info!("Successfully subscribed to chain notifications");
        Ok(())
    }

    /// Get the current tip block's hash
    pub async fn get_tip_hash(&self) -> Result<BlockHash, BlockConnectError> {
        log::debug!("Fetching current chain tip");
        let height = {
            log::trace!("Sending height request");
            let mut height_req = self.chain_client.get_height_request();
            height_req
                .get()
                .get_context()?
                .set_thread(self.thread.clone());
            let promise = height_req.send().promise;
            let response = promise.await?;
            let height_result = response.get()?;
            height_result.get_result()
        };

        let hash_bytes = {
            let mut hash_req = self.chain_client.get_block_hash_request();
            hash_req
                .get()
                .get_context()?
                .set_thread(self.thread.clone());
            hash_req.get().set_height(height);
            let response = hash_req.send().promise.await?;
            response.get()?.get_result()?.to_vec()
        };

        // Convert bytes to BlockHash
        let hash = self.bytes_to_block_hash(&hash_bytes)?;

        log::trace!("Chain tip hash: {}", hash);
        log::debug!(
            "Retrieved chain tip at height {} with hash {}",
            height,
            hash
        );

        Ok(hash)
    }
    
    /// Get the current tip block's height
    pub async fn get_tip_height(&self) -> Result<i32, BlockConnectError> {
        log::debug!("Fetching current chain tip");
        let height = {
            log::trace!("Sending height request");
            let mut height_req = self.chain_client.get_height_request();
            height_req
                .get()
                .get_context()?
                .set_thread(self.thread.clone());
            let promise = height_req.send().promise;
            let response = promise.await?;
            let height_result = response.get()?;
            height_result.get_result()
        };

        log::trace!("Chain tip height: {}", height);
        log::debug!(
            "Retrieved chain tip height {}", height);

        Ok(height)
    }

    /// Get block hash at specific height
    pub async fn get_block_hash_at_height(
        &self,
        height: i32,
    ) -> Result<Option<BlockHash>, BlockConnectError> {
        log::debug!("Getting block hash at height {}", height);

        let mut hash_req = self.chain_client.get_block_hash_request();

        // Set up thread context and parameters
        hash_req
            .get()
            .get_context()
            .map_err(BlockConnectError::ConnectionError)?
            .set_thread(self.thread.clone());

        hash_req.get().set_height(height);

        // Send request and get result
        let response = hash_req.send().promise.await?;
        let mut result = response
            .get()
            .map_err(BlockConnectError::ConnectionError)?
            .get_result()
            .map_err(BlockConnectError::ConnectionError)?;

        // If block doesn't exist at this height, return None
        if result.is_empty() {
            log::debug!("No block found at height {}", height);
            return Ok(None);
        }

        // Decode block hash directly using consensus_decode
        let hash = BlockHash::consensus_decode(&mut result)
            .map_err(|_| BlockConnectError::InvalidBlockData)?;

        log::debug!("Retrieved block hash at height {}", height);
        log::trace!("Block hash: {}", hash);

        Ok(Some(hash))
    }

    pub async fn get_block(
        &self,
        height: i32,
    ) -> Result<Block, BlockConnectError> {
        log::debug!("Getting ancestor block from height: {}", height);

        let node_tip_hash = self.get_tip_hash();

        let mut find_req = self.chain_client.find_ancestor_by_height_request();

        find_req
            .get()
            .get_context()
            .map_err(BlockConnectError::ConnectionError)?
            .set_thread(self.thread.clone());

        let mut params = find_req.get();
        params.set_block_hash(&node_tip_hash.await?.to_raw_hash().to_byte_array());
        params.set_ancestor_height(height);
        params
            .get_ancestor()
            .map_err(BlockConnectError::ConnectionError)?
            .set_want_data(true);

        let response = find_req.send().promise.await?;
        let mut data = response.get()?.get_ancestor()?.get_data()?;

        Block::consensus_decode(&mut data).map_err(|_| BlockConnectError::InvalidBlockData)
    }

    /// Check if a block is in the best chain
    pub async fn is_in_best_chain(&self, block_hash: &BlockHash) -> Result<bool, BlockConnectError> {
        log::debug!("Checking if block is in best chain");
        let hash_bytes = block_hash.to_raw_hash().to_byte_array();

        let mut find_req = self.chain_client.find_block_request();
        find_req
            .get()
            .get_context()?
            .set_thread(self.thread.clone());
        find_req.get().set_hash(&hash_bytes);
        let response = find_req.send().promise.await?;
        let block_info = response.get()?.get_block()?;
        let is_active = block_info.get_in_active_chain() != 0;

        log::debug!(
            "Block {} is {} in the active chain",
            block_hash,
            if is_active {
                "included"
            } else {
                "not included"
            }
        );
        Ok(is_active)
    }

    /// Find the common ancestor between two blocks
    pub async fn find_common_ancestor(
        &self,
        block1_hash: &BlockHash,
        block2_hash: &BlockHash,
    ) -> Result<Option<BlockHash>, BlockConnectError> {
        log::debug!(
            "Finding common ancestor between blocks {} and {}",
            block1_hash,
            block2_hash
        );
        let hash1_bytes = block1_hash.to_raw_hash().to_byte_array();
        let hash2_bytes = block2_hash.to_raw_hash().to_byte_array();

        let mut find_req = self.chain_client.find_common_ancestor_request();
        find_req
            .get()
            .get_context()?
            .set_thread(self.thread.clone());
        {
            let mut params = find_req.get();
            params.set_block_hash1(&hash1_bytes);
            params.set_block_hash2(&hash2_bytes);
        }
        let response = find_req.send().promise.await?;

        let ancestor_bytes = response.get()?.get_ancestor()?.get_data()?;
        if ancestor_bytes.is_empty() {
            log::debug!("No common ancestor found");
            Ok(None)
        } else {
            let ancestor_hash = self.bytes_to_block_hash(ancestor_bytes)?;
            log::debug!("Common ancestor found: {}", ancestor_hash);
            Ok(Some(ancestor_hash))
        }
    }

    /// Get a full block by its hash
    pub async fn get_block_by_hash(
        &self,
        block_hash: &BlockHash,
    ) -> Result<Option<Block>, BlockConnectError> {
        log::debug!("Getting block with hash {}", block_hash);
        let hash_bytes = block_hash.to_raw_hash().to_byte_array();

        let mut find_req = self.chain_client.find_block_request();
        find_req
            .get()
            .get_context()?
            .set_thread(self.thread.clone());
        find_req.get().set_hash(&hash_bytes);

        let response = find_req.send().promise.await?;

        // Check if the block was found
        let block_info = response.get()?.get_block()?;
        if !block_info.has_data() || block_info.get_data()?.is_empty() {
            log::debug!("No block data found for hash {}", block_hash);
            return Ok(None);
        }

        // Parse the block data
        match bitcoin::consensus::deserialize::<Block>(block_info.get_data()?) {
            Ok(block) => {
                log::debug!("Successfully retrieved block {}", block_hash);
                Ok(Some(block))
            }
            Err(e) => {
                log::error!("Failed to parse block data: {}", e);
                Err(BlockConnectError::InvalidBlockData)
            }
        }
    }

    // Helper method to convert bytes to BlockHash
    fn bytes_to_block_hash(&self, bytes: &[u8]) -> Result<BlockHash, BlockConnectError> {
        if bytes.len() != 32 {
            return Err(BlockConnectError::InvalidBlockData);
        }

        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(bytes);
        Ok(BlockHash::from_raw_hash(
            bitcoin::hashes::Hash::from_byte_array(hash_bytes),
        ))
    }
}
