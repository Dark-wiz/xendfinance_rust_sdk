use super::get_addresses::Addresses;
use super::privatekey_to_address::retrieve_address;
use crate::{abi, strategies::create_contract::create_contract};
use web3::contract::{Contract, Options};
use web3::types::{BlockId, BlockNumber, TransactionId, H160, U256, U64};

pub async fn get_balance(provider: String, private_key: &str, addresses: Addresses)-> String{
    let address: H160 = retrieve_address(private_key);
    let contract = create_contract(provider, abi::token().as_str(), addresses.token).await;

    let balance_of = match contract.method::<_, String>("balanceOf", ()) {
        Ok(res) => res,
        Err(_) => panic!("Couldn't get balance"),
    };
    let value = balance_of.clone().call().await.unwrap();
    value
    // web3_instance.eth().balance(address, block)
}
