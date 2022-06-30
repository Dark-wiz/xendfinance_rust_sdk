use ethers::{
    abi::{Abi},
    contract::Contract,
    prelude::{
        k256::ecdsa::SigningKey, Address, Provider, Signer,
        SignerMiddleware, Wallet,
    },
};

use std::{
    fs::File,
    str::FromStr,
};

pub async fn create_contract(
    provider: String,
    abi: &str,
    contract_address: String,
    private_key: &str,
    chain_id: u64,
) -> Contract<SignerMiddleware<Provider<ethers::prelude::Http>, Wallet<SigningKey>>> {
    let contract_provider = create_provider(&provider);

    let address = Address::from_str(&contract_address).unwrap();

    let wallet: Wallet<SigningKey> = private_key.parse().unwrap();
    let wallet = wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(contract_provider, wallet);

    create_contract_instance(address, abi, client.clone())
}

// Create sign_able contract with provider
fn create_contract_instance(
    contract_address: Address,
    abi_path: &str,
    contract_provider: SignerMiddleware<Provider<ethers::prelude::Http>, Wallet<SigningKey>>,
) -> Contract<SignerMiddleware<Provider<ethers::prelude::Http>, Wallet<SigningKey>>> {
    let file = File::open(abi_path).expect("No JSON file");
    let contract_abi: Abi = serde_json::from_reader(file).expect("Wrong JSON format");

    Contract::new(contract_address, contract_abi, contract_provider)
}

pub fn create_provider(node: &str) -> Provider<ethers::prelude::Http> {
    Provider::try_from(node).expect("Wrong node")
}
