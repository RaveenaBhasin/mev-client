use starknet::core::types::Felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Url};
use starknet_mev_client::amm::factory::AutomatedMarketMakerFactory;
use starknet_mev_client::amm::pool::AutomatedMarketMaker;
use starknet_mev_client::amm::tenKSwap::factory::TenKFactory;
use starknet_mev_client::amm::tenKSwap::get_data;
use std::sync::Arc;
use tokio;
// use starknet_mev_client::amm::AutomatedMarketMaker;

#[allow(unused)]
fn create_rpc_provider(
    rpc_url: &str,
) -> Result<Arc<JsonRpcClient<HttpTransport>>, Box<dyn std::error::Error>> {
    let url = Url::parse(rpc_url)?;
    let transport = HttpTransport::new(url);
    let provider = JsonRpcClient::new(transport);
    Ok(Arc::new(provider))
}

#[tokio::main]
async fn main() {
    let mut factory = TenKFactory::new(
        Felt::from_hex("0x1c0a36e26a8f822e0d81f20a5a562b16a8f8a3dfd99801367dd2aea8f1a87a2")
            .unwrap(),
    );

    print!("Initialise factory {:?}", factory);

    let rpc_url = "https://starknet-mainnet.public.blastapi.io/rpc/v0_7";
    let provider = create_rpc_provider(rpc_url).unwrap();
    let pools = factory.fetch_all_pools(provider.clone()).await.unwrap();
    let pool  = &pools[0];
    println!("Fetched pools: {:?}", pool);

    pool.simulate_swap(
        Felt::from_hex("0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7").unwrap(), 
        Felt::from_hex("0x53c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8").unwrap(), 
        Felt::from(100u32),
        provider
    );

    // get_data::get_v2_pool_data_batch_request(pools[0], provider)
    //     .await
    //     .unwrap();
    // let amm = AMM::JediswapPool`(pool);
    // println!("Jediswap DAI/ETH pool address: {:?}", amm.address());
    //
    // let rpc_url = "https://starknet-mainnet.public.blastapi.io/rpc/v0_7";
    // let provider = create_rpc_provider(rpc_url).unwrap();
    //
    // let block_number = provider.block_number().await.unwrap();
    // println!("Current block number: {}", block_number);
    //
    // let reserves = amm.get_reserves(provider).await.unwrap();
    // println!("reserves {:?}", reserves);
}
