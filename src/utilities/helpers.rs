use super::get_addresses;
use web3::types::{BlockId, BlockNumber, TransactionId, H160, U256, U64};
use serde_derive::Deserialize;
use serde_derive::Serialize;


#[derive(Debug)]
pub struct ProviderType {
    pub name: String,
    pub currency: String,
    pub url: String,
    pub chain: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApyRoot {
    pub data: Vec<ApyData>,
    pub status: String,
    #[serde(rename = "status_code")]
    pub status_code: i64,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApyData {
    pub id: i64,
    pub name: String,
    pub apy: String,
    pub desc: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}


pub const APYS_URL: &str = "https://api.xend.finance/xend-finance/apys";
pub const PROTOCOL_ADDRESS_URL: &str = "https://api.xend.finance/xend-finance/addresses";

pub const ETHEREUM_MAINNET: i32 = 1;
pub const BSC_MAINNET: i32 = 56;
pub const BSC_TESTNET: i32 = 97;
pub const POLYGON_MAINNET: i32 = 137;
pub const POLYGON_TESTNET: i32 = 80001;
pub const LOCALHOST: i32 = 0;

pub fn get_chain_id(id: i32) -> ProviderType {
    match id {
        ETHEREUM_MAINNET => PROVIDERS(String::from("ETHEREUM_MAINNET")),
        BSC_MAINNET => PROVIDERS(String::from("BSC_MAINNET")),
        BSC_TESTNET => PROVIDERS(String::from("BSC_TESTNET")),
        POLYGON_MAINNET => PROVIDERS(String::from("POLYGON_MAINNET")),
        POLYGON_TESTNET => PROVIDERS(String::from("POLYGON_TESTNET")),
        LOCALHOST => PROVIDERS(String::from("LOCALHOST")),
        _ => PROVIDERS(String::from("LOCALHOST")),
    }
}

#[derive(Debug)]
pub enum SavingStrtegyType {
    YEARN_FINANCE,
    DEFI_DOLLARS,
}

pub fn PROVIDERS(state: String) -> ProviderType {
    if state == "ETHEREUM_MAINNET" {
        let ethereum = ProviderType {
            name: "ETHEREUM_MAINNET".to_string(),
            currency: "DAI".to_string(),
            url: "https://eth-mainnet.alchemyapi.io/v2/2gdCD03uyFCNKcyEryqJiaPNtOGdsNLv"
                .to_string(),
            chain: 1,
        };
        ethereum
    } else if state == "BSC_MAINNET" {
        let bsc_mainet = ProviderType {
            name: "BSC_MAINNET".to_string(),
            currency: "BUSD".to_string(),
            url: "https://bsc-dataseed.binance.org/".to_string(),
            chain: 56,
        };
        bsc_mainet
    } else if state == "BSC_TESTNET" {
        let bsc_testnet = ProviderType {
            name: "BSC_TESTNET".to_string(),
            currency: "BUSD".to_string(),
            url: "https://data-seed-prebsc-1-s1.binance.org:8545/".to_string(),
            chain: 97,
        };
        bsc_testnet
    } else if state == "POLYGON_MAINNET" {
        let polygon_mainnet = ProviderType {
            name: "POLYGON_MAINNET".to_string(),
            currency: "MATIC".to_string(),
            url: "https://rpc-mainnet.matic.network/".to_string(),
            chain: 137,
        };
        polygon_mainnet
    } else if state == "POYLGON_TESTNET" {
        let polygon_testnet = ProviderType {
            name: "POYLGON_TESTNET".to_string(),
            currency: "MATIC".to_string(),
            url: "https://rpc-mumbai.matic.today/".to_string(),
            chain: 8001,
        };
        polygon_testnet
    } else if state == "POYLGON_TESTNET" {
        let localhost = ProviderType {
            name: "BSC_MAINNET".to_string(),
            currency: "BUSD".to_string(),
            url: "http://127.0.0.1:8545".to_string(),
            chain: 0,
        };
        localhost
    } else {
        let localhost = ProviderType {
            name: "LOCALHOST".to_string(),
            currency: "BUSD".to_string(),
            url: "http://127.0.0.1:8545".to_string(),
            chain: 0,
        };
        localhost
    }
}

pub fn get_address_by_name(addresses: Vec<get_addresses::ProtocolAddress>, name: String) -> String {
    let mut response = addresses
        .into_iter()
        .filter(|address| address.name == name)
        .last()
        .ok_or("failed to get address");

    let address = match response {
        Ok(val) => val.address,
        Err(error) => String::from("failed to get address"),
    };
    address
}

pub fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    let res = res / 1_000_000_000_000_000_000.0;
    res
}

pub fn capitalize_first_letter(s: String) -> String {
    s.as_str();
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}