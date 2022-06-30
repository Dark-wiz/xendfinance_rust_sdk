use ethers::{
    prelude::{
        k256::ecdsa::SigningKey, Address, Signer,
        SignerMiddleware, Wallet,
    }
};
use crate::strategies::create_contract::create_provider;

pub fn retrieve_address(private_key: &str, chain_id: u64, provider: String)  -> Address{
    let contract_provider = create_provider(&provider);

    let wallet: Wallet<SigningKey> = private_key.parse().unwrap();
    let wallet = wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(contract_provider, wallet);
    client.address()

    //generate public key 
    // let context = secp256k1::Secp256k1::new();
    // let private_key: &[u8] = private_key_value.as_bytes();
    // let secret_key = SecretKey::from_slice(&hex::decode(private_key).unwrap());
    // let response = PublicKey::from_secret_key(&context, &secret_key.unwrap());

    // let public_key = response.serialize_uncompressed();
    // debug_assert_eq!(public_key[0], 0x04);

    // //return address
    // let hash = keccak256(&public_key[1..]);
    // Address::from_slice(&hash[12..])
}