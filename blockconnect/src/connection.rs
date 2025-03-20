use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

use crate::chain_capnp::chain::Client as ChainClient;
use crate::init_capnp::init::Client as InitClient;
use crate::proxy_capnp::thread::Client as ThreadClient;
use crate::BlockConnectError;

/// Represents a connection to the Bitcoin node
pub struct Connection {
    rpc_handle: JoinHandle<Result<(), capnp::Error>>,
    disconnector: capnp_rpc::Disconnector<twoparty::VatId>,
    thread: ThreadClient,
    chain_client: ChainClient,
}

impl Connection {
    /// Create a new connection to the Bitcoin node
    pub async fn connect(socket_path: &str) -> Result<Arc<Self>, BlockConnectError> {
        log::info!("Connecting to Bitcoin node at {}", socket_path);

        let stream = tokio::net::UnixStream::connect(socket_path).await?;
        log::debug!("Unix stream connected successfully");
        let (reader, writer) = stream.into_split();

        log::debug!("Setting up RPC network");
        let network = Box::new(twoparty::VatNetwork::new(
            reader.compat(),
            writer.compat_write(),
            rpc_twoparty_capnp::Side::Client,
            Default::default(),
        ));
        let mut rpc = RpcSystem::new(network, None);
        let init_interface: InitClient = rpc.bootstrap(rpc_twoparty_capnp::Side::Server);
        let disconnector = rpc.get_disconnector();

        log::debug!("Spawning RPC task");
        let rpc_handle = tokio::task::spawn_local(rpc);

        // Get thread client
        let mk_init_req = init_interface.construct_request();
        let response = mk_init_req.send().promise.await?;

        let thread_map = response.get()?.get_thread_map()?;

        let mk_thread_req = thread_map.make_thread_request();
        let response = mk_thread_req.send().promise.await?;

        let thread = response.get()?.get_result()?;
        log::debug!("Thread client established");

        // Set up chain client with thread context
        let mut mk_chain_req = init_interface.make_chain_request();
        {
            let mut context = mk_chain_req.get().get_context()?;
            context.set_thread(thread.clone());
        }
        let response = mk_chain_req.send().promise.await?;

        let chain_client = response.get()?.get_result()?;
        log::debug!("Chain client established");

        log::info!("Connection to node established successfully");
        Ok(Arc::new(Self {
            rpc_handle,
            disconnector,
            thread,
            chain_client,
        }))
    }

    /// Disconnect from the node
    pub async fn disconnect(self) -> Result<(), BlockConnectError> {
        log::info!("Disconnecting from node");
        self.disconnector
            .await
            .map_err(BlockConnectError::ConnectionError)?;
        self.rpc_handle
            .await
            .map_err(|e| BlockConnectError::NodeError(e.to_string()))?
            .map_err(BlockConnectError::ConnectionError)?;
        log::info!("Disconnection completed successfully");
        Ok(())
    }

    /// Get a reference to the chain client
    pub fn chain_client(&self) -> &ChainClient {
        &self.chain_client
    }

    /// Get a reference to the thread client
    pub fn thread(&self) -> &ThreadClient {
        &self.thread
    }
}
