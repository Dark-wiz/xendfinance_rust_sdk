use web3;
use web3::types::{Address};
use web3::transports::Http;
use web3::Web3;

use super::helpers::get_chain_id;
enum Result<T, E> {
    Ok(T),
    Err(E),
}
const DEFAULT_PASSWORD: &str = "";

pub fn instantiate_web3(provider: &str) -> Web3<Http> {
    let transport = web3::transports::Http::new(provider);
    let transport_val = match transport {
        Ok(val) => val,
        Err(error) => panic!("Failed to instantiate web3"),
    };
    let web3 = web3::Web3::new(transport_val);
    web3
}

pub async fn create_wallet(chain_id: i32)->Address {
    let provider = get_chain_id(chain_id).url;
    let web3_instance = instantiate_web3(provider.as_str());
    let wallet = web3_instance.personal().new_account(DEFAULT_PASSWORD).await;
    let address = match wallet {
        Ok(val) => val,
        Err(error) => panic!("Failed to create account"),
    };
    address
}

