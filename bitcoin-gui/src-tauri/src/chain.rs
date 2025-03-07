use bitcoin::{Block, BlockHash};

use crate::chain_capnp;
use crate::init_capnp;
use crate::proxy_capnp;

// Create a Chain client
pub async fn create_chain_client(
    init_client: &init_capnp::init::Client,
    thread_client: &proxy_capnp::thread::Client,
) -> Result<chain_capnp::chain::Client, Box<dyn std::error::Error>> {
    let mut make_chain_request = init_client.make_chain_request();
    make_chain_request
        .get()
        .get_context()?
        .set_thread(thread_client.clone());

    let chain_client_response = make_chain_request.send().promise.await?;
    println!(
        "received chain_client response: {:?}",
        chain_client_response.get()?
    );

    Ok(chain_client_response.get()?.get_result()?)
}

// Query Chain height
pub async fn query_chain_height(
    chain_client: &chain_capnp::chain::Client,
    thread_client: &proxy_capnp::thread::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_chain_request = chain_client.get_height_request();
    new_chain_request
        .get()
        .get_context()?
        .set_thread(thread_client.clone());
    let new_chain = new_chain_request.send().promise.await?;
    println!("received chain response: {:?}", new_chain.get()?.get_result());
    Ok(())
}

pub async fn query_chain_block_height(
    chain_client: &chain_capnp::chain::Client,
    thread_client: &proxy_capnp::thread::Client,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut new_chain_request = chain_client.get_height_request();
    new_chain_request
        .get()
        .get_context()?
        .set_thread(thread_client.clone());
    let response = new_chain_request.send().promise.await?;
    println!("received chain response: {:?}", response.get()?);
    Ok(response.get()?.get_result())
}

// pub async fn query_chain_best_block_hash(
//     chain_client: &chain_capnp::chain::Client,
//     thread_client: &proxy_capnp::thread::Client,
// ) -> Result<String, Box<dyn std::error::Error>> {
//     let mut new_chain_request = chain_client.get_block_hash_request();
//     new_chain_request
//         .get()
//         .get_context()?
//         .set_thread(thread_client.clone());
//     let response = new_chain_request.send().promise.await?;
//     println!("received chain response: {:?}", response.get()?);
//     let bytes = response.get()?.get_result()?;
//     Ok(String::from_utf8(bytes.to_vec())?)
// }

pub async fn query_chain_block_hash_at_height(
    chain_client: &chain_capnp::chain::Client,
    thread_client: &proxy_capnp::thread::Client,
    height: i32
) -> Result<String, Box<dyn std::error::Error>> {
    let mut new_chain_request = chain_client.get_block_hash_request();
    new_chain_request
        .get()
        .get_context()?
        .set_thread(thread_client.clone());

    new_chain_request.get().set_height(height);

    let response = new_chain_request.send().promise.await?;
    println!("received chain response: {:?}", response.get()?);
    let bytes = response.get()?.get_result()?;

    // Convert bytes to a fixed-size array
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes);
    let hash_bytes = BlockHash::from_raw_hash(bitcoin::hashes::Hash::from_byte_array(array));

    Ok(hash_bytes.to_string())
}

pub async fn query_chain_block(
    chain_client: &chain_capnp::chain::Client,
    thread_client: &proxy_capnp::thread::Client,
    node_tip_hash: &bitcoin::BlockHash,
    height: i32
) -> Result<Block, Box<dyn std::error::Error>> {
    let mut new_chain_request = chain_client.find_ancestor_by_height_request();
    new_chain_request
        .get()
        .get_context()?
        .set_thread(thread_client.clone());

    let mut params = new_chain_request.get();
    params.set_block_hash(node_tip_hash.as_ref());
    params.set_ancestor_height(height);
    params.get_ancestor().unwrap().set_want_data(true);

    let response = new_chain_request.send().promise.await?;
    println!("received chain response: {:?}", response.get()?);
    let bytes = response.get()?.get_ancestor()?.get_data()?;

    let block= bitcoin::consensus::deserialize(&bytes)?;

    Ok(block)
}