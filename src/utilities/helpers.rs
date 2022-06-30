use super::get_addresses;
use ether_converter;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use ethers::prelude::{ U256, U64};


#[derive(Debug)]
pub struct ProviderType {
    pub name: String,
    pub currency: String,
    pub url: String,
    pub chain: u64,
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


pub const SUCCESS_TX:u64 = 1;

pub const ETHEREUM_MAINNET: u64 = 1;
pub const BSC_MAINNET: u64 = 56;
pub const BSC_TESTNET: u64 = 97;
pub const POLYGON_MAINNET: u64 = 137;
pub const POLYGON_TESTNET: u64 = 80001;
pub const LOCALHOST: u64 = 0;

pub fn get_chain_id(id: u64) -> ProviderType {
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
pub struct TransactionResult {
    pub transaction_hash: String,
    pub status: bool
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
            url: "https://eth-mainnet.alchemyapi.io/v2/2gdCD03uyFCNKcyEryqJiaPNtOGdsNLv".to_string(),
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
        Err(error) => panic!("failed to get address, {:?}", error),
    };
    address
}

pub fn capitalize_first_letter(s: String) -> String {
    s.as_str();
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn format_amount(amount: String, network: u64, asset_name: &str) -> String {
    match network {
        BSC_MAINNET => {
            let result = ether_converter::to_wei(amount.as_str(), "ether");
            result
        }
        POLYGON_MAINNET => {
            if asset_name == "WBTC" {
                let result = ether_converter::to_wei(amount.as_str(), "ether");
                result
            } else if asset_name == "AAVE" {
                let power = f32::powf(10.0, 8.0);
                let f: f32 = amount.parse().unwrap();
                let result: f32 = power * f;
                result.to_string()
            } else {
                let result = ether_converter::to_wei(amount.as_str(), "mwei");
                result
            }
        }
        _ => amount,
    }
}
