use crate::node_capnp;
use crate::proxy_capnp;
use crate::wallet_capnp;

// Manage wallets using the Node client
pub async fn create_wallet_loader_client(
    node_client: &node_capnp::node::Client,
    thread_client: &proxy_capnp::thread::Client,
) -> Result<wallet_capnp::wallet_loader::Client, Box<dyn std::error::Error>> {
    let mut custom_wallet_loader_request = node_client.custom_wallet_loader_request();
    custom_wallet_loader_request
        .get()
        .get_context()?
        .set_thread(thread_client.clone());
    let cwl_response = custom_wallet_loader_request.send().promise.await?;
    println!(
        "received create_wallet_loader_client response: {:?}",
        cwl_response.get()?
    );
    Ok(cwl_response.get()?.get_result()?)
}

// pub async fn query_wallet_address(
//     wallet_client: &wallet_capnp::wallet::Client,
//     thread_client: &proxy_capnp::thread::Client,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let mut new_chain_request = wallet_client.get_addresses_request();
//     new_chain_request
//         .get()
//         .get_context()?
//         .set_thread(thread_client.clone());
//     let new_chain = new_chain_request.send().promise.await?;
//     let response = new_chain.get()?.get_result()?;
//     println!("received chain response: {:?}", new_chain.get()?.get_result());
//     Ok(response)
// }