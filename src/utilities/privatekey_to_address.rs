use secp256k1::{PublicKey, SecretKey};
use tiny_keccak::keccak256;
use web3::types::Address;
pub fn retrieve_address(private_key_value: &str)  -> Address{
    //generate public key 
    let context = secp256k1::Secp256k1::new();
    let private_key: &[u8] = private_key_value.as_bytes();
    let secret_key = SecretKey::from_slice(&hex::decode(private_key).unwrap());
    let response = PublicKey::from_secret_key(&context, &secret_key.unwrap());

    let public_key = response.serialize_uncompressed();
    debug_assert_eq!(public_key[0], 0x04);

    //return address
    let hash = keccak256(&public_key[1..]);
    Address::from_slice(&hash[12..])
}