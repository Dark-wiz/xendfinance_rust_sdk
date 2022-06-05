pub struct ProviderType {
    pub name: String,
    pub currency: String,
    pub url: String,
    pub chain: u32,
}
struct ChainId;
impl ChainId {
    pub const ETHEREUM_MAINNET: i32 = 1;
    pub const BSC_MAINNET: i32 = 56;
    pub const BSC_TESTNET: i32 = 97;
    pub const POLYGON_MAINNET: i32 = 137;
    pub const POLYGON_TESTNET: i32 = 80001;
    pub const LOCALHOST: i32 = 0;
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
    } else if state == "POYLGON_TESTNET"{
        let localhost = ProviderType {
            name: "BSC_MAINNET".to_string(),
            currency: "BUSD".to_string(),
            url: "http://127.0.0.1:8545".to_string(),
            chain: 0,
        };
        localhost
    }else {
        let localhost = ProviderType {
            name: "LOCALHOST".to_string(),
            currency: "BUSD".to_string(),
            url: "http://127.0.0.1:8545".to_string(),
            chain: 0,
        };
        localhost
    }
}
