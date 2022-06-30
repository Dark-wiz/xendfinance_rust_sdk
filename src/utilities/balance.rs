use super::get_addresses::Addresses;
use super::privatekey_to_address::retrieve_address;
use crate::{abi, strategies::create_contract::create_contract};
use ethers::prelude::{Address, U256};

pub async fn get_balance(
    provider: String,
    private_key: &str,
    addresses: Addresses,
    chain_id: u64,
) -> U256 {
    let address: Address = retrieve_address(private_key, chain_id, provider.clone());
    let contract = create_contract(
        provider.clone(), 
        abi::token().as_str(),
        addresses.token,
        private_key,
        chain_id,
    )
    .await;

    let val = match contract
        .method::<_, U256>("balanceOf", address)
        .unwrap()
        .call()
        .await
    {
        Ok(it) => it,
        Err(err) => panic!("Couldn't get balance: {:?}", err),
    };
    val
}
