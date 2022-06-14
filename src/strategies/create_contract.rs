use ethers_core::{
    abi::Abi,
    types::{Address, H256},
    // contract::{Contract, Options},
};
use web3::contract::{Contract, Options};
// use ethers_contract::Contract;
use ethers_providers::{Http, Provider};

enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub async fn create_contract(
    provider: String,
    abi: &str,
    contract_address: String,
) -> Contract<web3::transports::Http> {
    let address = contract_address.parse::<Address>();

    let correct_address = match address {
        Ok(val) => val,
        Err(error) => panic!("Problem reading address: {:?}", error),
    };
    let transport = web3::transports::Http::new(provider.as_str());
    let main_transport = match transport {
        Ok(result) => result,
        Err(error) => panic!("couldn't get contract {:?}", error),
    };

    let abi = std::fs::read(abi).expect("Can't read ABI from file");
    let web3 = web3::Web3::new(main_transport);
    let contract = Contract::from_json(web3.eth(), correct_address, &abi);

    let main_contract = match contract {
        Ok(result) => result,
        Err(error) => panic!("couldn't get contract {:?}", error),
    };
    main_contract
}
