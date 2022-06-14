use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
use web3;
use web3::transports::Http;
use web3::types::Address;
use web3::Web3;

use super::{helpers::get_chain_id, privatekey_to_address::retrieve_address};
enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Clone, Debug)]
pub struct WalletKeys {
    pub secret_key: SecretKey,
    pub address: Address,
}

pub fn instantiate_web3(provider: &str) -> Web3<Http> {
    let transport = web3::transports::Http::new(provider);
    let transport_val = match transport {
        Ok(val) => val,
        Err(error) => panic!("Failed to instantiate web3"),
    };
    let web3 = web3::Web3::new(transport_val);
    print!("{:?}", web3);
    web3
}

pub async fn create_wallet(chain_id: i32) -> WalletKeys {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(111);
    let (secret_key, public_key)= secp.generate_keypair(&mut rng);
    let addr = retrieve_address(&secret_key.to_string());

    let keys = WalletKeys {
        secret_key: secret_key,
        address: addr
    };
    keys
    // let provider = get_chain_id(chain_id).url;
    // let web3_instance = instantiate_web3(provider.as_str());
    // let wallet = web3_instance.accounts(). personal().new_account(DEFAULT_PASSWORD).await;
    // let address = match wallet {
    //     Ok(val) => val,
    //     Err(error) => panic!("Failed to create account"),
    // };
    // address
}
