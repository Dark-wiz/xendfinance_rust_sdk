use ethers::prelude::U256;
use crate::{abi::protocol_adapter, strategies::create_contract::create_contract};

pub async fn get_price_per_full_share(
    provider: String,
    contract_address: String,
    private_key: &str,
    chain_id: u64,
) -> U256 {
    let protocol = protocol_adapter();
    let abi = protocol.as_str();

    let contract = create_contract(
        provider.clone(),
        abi,
        contract_address.clone(),
        private_key,
        chain_id,
    )
    .await;
    let get_ppfs = match contract
        .method::<_, U256>("GetPricePerFullShare", ())
        .unwrap()
        .call()
        .await
    {
        Ok(res) => res,
        Err(_) => panic!("Couldn't get price per full share"),
    };
    get_ppfs
}
