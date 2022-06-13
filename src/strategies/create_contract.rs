use ethers_core::{
    abi::Abi,
    types::{Address, H256},
};
use ethers_contract::Contract;
use ethers_providers::{Provider, Http};

enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub async fn create_contract(provider: String, abi: &str, contractAddress: String)->ethers_contract::Contract<Provider<Http>>{
    let abi:Abi = serde_json::from_str(abi).unwrap();
    
    let client =Provider::<Http>::try_from(provider).unwrap();
    let address = contractAddress.parse::<Address>();

    let correct_address = match address {
        Ok(val) => val,
        Err(error) => panic!("Problem reading address: {:?}", error),
    };

    let contract = Contract::new(correct_address, abi, client);
    contract
}
