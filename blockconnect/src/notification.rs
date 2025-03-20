use async_trait::async_trait;
use bitcoin::hashes::Hash;
use bitcoin::{consensus::Decodable, Block, BlockHash, Transaction, Txid};
use capnp::capability::Promise;
use capnp_rpc::pry;
use std::sync::Arc;
use std::sync::Mutex;

use crate::chain_capnp::chain_notifications;
use crate::error::BlockConnectError;

// Public interface
#[derive(Clone, Debug)]
pub enum ChainNotification {
    BlockConnected(Block),
    BlockDisconnected(BlockHash),
    TransactionAddedToMempool(Transaction),
    TransactionRemovedFromMempool(Txid),
    UpdatedBlockTip(BlockHash),
    ChainStateFlushed,
}

#[async_trait]
pub trait NotificationHandler: Send + Sync {
    async fn handle_notification(
        &self,
        notification: ChainNotification,
    ) -> Result<(), BlockConnectError>;
}

#[derive(Clone)]
pub struct ChainNotificationHandler {
    handlers: Arc<Mutex<Vec<Arc<dyn NotificationHandler>>>>,
}

impl ChainNotificationHandler {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn register_handler(&mut self, handler: Arc<dyn NotificationHandler>) {
        // TODO: could use better error handling
        let mut guard = self.handlers.lock().unwrap();
        guard.push(handler);
    }

    async fn dispatch_notification(
        &self,
        notification: ChainNotification,
    ) -> Result<(), BlockConnectError> {
        let handlers = {
            // TODO: could use better error handling
            let guard = self.handlers.lock().unwrap();
            guard.clone()
        };

        for handler in handlers {
            handler.handle_notification(notification.clone()).await?;
        }
        Ok(())
    }
}

impl Default for ChainNotificationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl chain_notifications::Server for ChainNotificationHandler {
    fn block_connected(
        &mut self,
        params: chain_notifications::BlockConnectedParams,
        _: chain_notifications::BlockConnectedResults,
    ) -> ::capnp::capability::Promise<(), ::capnp::Error> {
        let handler = self.clone();

        let future = async move {
            let params_reader = params.get()?;
            let block_info = params_reader.get_block()?;
            let mut block_data = block_info.get_data()?;

            // Decode the block
            let block = bitcoin::Block::consensus_decode(&mut block_data)
                .map_err(|e| ::capnp::Error::failed(format!("Failed to decode block: {}", e)))?;

            // Dispatch notification
            handler
                .dispatch_notification(ChainNotification::BlockConnected(block))
                .await
                .map_err(|e| {
                    ::capnp::Error::failed(format!("Failed to dispatch notification: {}", e))
                })
        };

        Promise::from_future(future)
    }

    fn block_disconnected(
        &mut self,
        params: chain_notifications::BlockDisconnectedParams,
        _: chain_notifications::BlockDisconnectedResults,
    ) -> Promise<(), ::capnp::Error> {
        let handler = self.clone();

        let future = async move {
            let params_reader = params.get()?;
            let block_info = params_reader.get_block()?;
            // let height = block_info.get_height();
            let hash_data = block_info.get_hash()?;

            // Create BlockHash from sha256d hash
            let hash = {
                let hash_obj = bitcoin::hashes::sha256d::Hash::from_slice(hash_data)
                    .map_err(|e| ::capnp::Error::failed(format!("Invalid block hash: {}", e)))?;
                bitcoin::BlockHash::from(hash_obj)
            };

            // Dispatch notification
            handler
                .dispatch_notification(ChainNotification::BlockDisconnected(hash))
                .await
                .map_err(|e| {
                    ::capnp::Error::failed(format!("Failed to dispatch notification: {}", e))
                })
        };

        Promise::from_future(future)
    }

    fn transaction_added_to_mempool(
        &mut self,
        params: chain_notifications::TransactionAddedToMempoolParams,
        _: chain_notifications::TransactionAddedToMempoolResults,
    ) -> Promise<(), ::capnp::Error> {
        let handler = self.clone();

        let tx =
            match bitcoin::Transaction::consensus_decode(&mut pry!(pry!(params.get()).get_tx())) {
                Ok(tx) => tx,
                Err(e) => {
                    return Promise::err(::capnp::Error::failed(format!(
                        "Failed to decode transaction: {}",
                        e
                    )))
                }
            };

        Promise::from_future(async move {
            handler
                .dispatch_notification(ChainNotification::TransactionAddedToMempool(tx))
                .await
                .map_err(|e| {
                    ::capnp::Error::failed(format!("Failed to dispatch notification: {}", e))
                })
        })
    }

    fn transaction_removed_from_mempool(
        &mut self,
        params: chain_notifications::TransactionRemovedFromMempoolParams,
        _: chain_notifications::TransactionRemovedFromMempoolResults,
    ) -> ::capnp::capability::Promise<(), ::capnp::Error> {
        let handler = self.clone();

        let txid = match bitcoin::Txid::consensus_decode(&mut pry!(pry!(params.get()).get_tx())) {
            Ok(txid) => txid,
            Err(e) => {
                return Promise::err(::capnp::Error::failed(format!(
                    "Failed to decode txid: {}",
                    e
                )))
            }
        };

        Promise::from_future(async move {
            handler
                .dispatch_notification(ChainNotification::TransactionRemovedFromMempool(txid))
                .await
                .map_err(|e| {
                    ::capnp::Error::failed(format!("Failed to dispatch notification: {}", e))
                })
        })
    }

    // fn updated_block_tip(
    //     &mut self,
    //     params: chain_notifications::UpdatedBlockTipParams,
    //     _: chain_notifications::UpdatedBlockTipResults,
    // ) -> ::capnp::capability::Promise<(), ::capnp::Error> {
    //     let handler = self.clone();
        
    //     let context_reader = pry!(pry!(params.get()).get_context());
    //     // According to the schema, the context reader has the data we need but differently structured
    //     // We'll need to get the height and hash from appropriate fields based on the context schema
    //     let block_info = pry!(context_reader.get_block());
    //     let hash_data = pry!(block_info.get_hash());
        
    //     let hash = match bitcoin::hashes::sha256d::Hash::from_slice(hash_data) {
    //         Ok(hash_obj) => bitcoin::BlockHash::from(hash_obj),
    //         Err(e) => return Promise::err(::capnp::Error::failed(format!("Invalid block hash: {}", e)))
    //     };
    
    //     // Convert the async dispatch_notification to a Promise
    //     Promise::from_future(async move {
    //         handler.dispatch_notification(ChainNotification::UpdatedBlockTip(hash)).await
    //             .map_err(|e| ::capnp::Error::failed(format!("Failed to dispatch notification: {}", e)))
    //     })
    // }

    fn updated_block_tip(
        &mut self,
        _params: chain_notifications::UpdatedBlockTipParams,
        _: chain_notifications::UpdatedBlockTipResults,
    ) -> ::capnp::capability::Promise<(), ::capnp::Error> {
        let handler = self.clone();

        let future = async move {
            // Simply log that we received the notification
            log::info!("Block tip updated - details skipped");

            // Create a dummy block hash - in a real implementation you'd get this from params
            let dummy_hash = bitcoin::BlockHash::all_zeros();

            // Dispatch notification with dummy data
            handler
                .dispatch_notification(ChainNotification::UpdatedBlockTip(dummy_hash))
                .await
                .map_err(|e| {
                    ::capnp::Error::failed(format!("Failed to dispatch notification: {}", e))
                })
        };

        ::capnp::capability::Promise::from_future(future)
    }

    fn chain_state_flushed(
        &mut self,
        _params: chain_notifications::ChainStateFlushedParams,
        _: chain_notifications::ChainStateFlushedResults,
    ) -> ::capnp::capability::Promise<(), ::capnp::Error> {
        let handler = self.clone();

        let future = async move {
            // Dispatch notification
            handler
                .dispatch_notification(ChainNotification::ChainStateFlushed)
                .await
                .map_err(|e| {
                    ::capnp::Error::failed(format!("Failed to dispatch notification: {}", e))
                })
        };

        ::capnp::capability::Promise::from_future(future)
    }

    fn destroy(
        &mut self,
        _params: chain_notifications::DestroyParams,
        _: chain_notifications::DestroyResults,
    ) -> ::capnp::capability::Promise<(), ::capnp::Error> {
        ::capnp::capability::Promise::ok(())
    }
}
