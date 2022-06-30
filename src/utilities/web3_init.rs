use ethers::prelude::Address;
use secp256k1::{
    rand::{rngs, SeedableRng}
};
use super::{helpers::get_chain_id, privatekey_to_address::retrieve_address};
enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Clone, Debug)]
pub struct WalletKeys {
    pub secret_key: String,
    pub address: Address,
}

// pub async fn create_wallet(chain_id: u64, provider: String) -> WalletKeys {
//     let secp = secp256k1::Secp256k1::new();
//     let mut rng = rngs::StdRng::seed_from_u64(123);
//     let (secret_key, public_key) = secp.generate_keypair(&mut rng);
//     let sec_key = secret_key.to_string();
//     let addr = retrieve_address(&sec_key.clone(), chain_id, provider);

//     let keys = WalletKeys {
//         secret_key: sec_key,
//         address: addr,
//     };
//     keys
// }

pub async fn retrieve_wallet(private_key: String, chain_id: u64, provider: String) -> WalletKeys {
    let addr = retrieve_address(&private_key.to_string(), chain_id, provider);

    let keys = WalletKeys {
        secret_key: private_key,
        address: addr,
    };
    keys
}
