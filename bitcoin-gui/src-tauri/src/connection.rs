use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio_util::compat::{TokioAsyncReadCompactExt, TokioAsyncWriteCompatExt};

use crate::chain_capnp::chain::Client as ChainClient;