mod abi;
pub mod environments;
mod strategies;
mod utilities;
use tokio::net::TcpStream;
use utilities::{balance::get_balance, helpers::{APYS_URL, ApyRoot, ApyData}, *};
use web3::types::Address;

use crate::utilities::{
    get_addresses::{ get_mainnet_protocols, Addresses, ProtocolAddress},
    helpers::{get_chain_id, BSC_MAINNET, ETHEREUM_MAINNET, POLYGON_MAINNET},
    layer2_assets::get_layer2_assets,
    price_per_full_share::get_price_per_full_share,
    privatekey_to_address::retrieve_address,
    protocol_selector::{protocol_selector, AvailableAddress},
    web3_init::{create_wallet, WalletKeys},
};

#[derive(Clone, Debug)]
pub struct XendFinanceSdk {
    pub chain_id: i32,
    pub private_key: String,
    pub options: Options,
    pub provider: String,
    pub protocol: String,
    pub available_protocols: Vec<AvailableAddress>,
    pub addresses: Addresses,
    pub currency: String,
    pub share_currency: String,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub env: String,
    pub protocol_name: String,
    pub protocols: Vec<get_addresses::BscProtocols>,
    pub layer2: [layer2_assets::layer2_asset; 11], // layer2_assets::get_layer2_assets(),
}

impl Options {
    pub async fn new(env: &str) -> Self {
        let environment = match env {
            "local" => "local",
            "testnet" => "testnet",
            "mainnet" => "mainnet",
            &_ => panic!("Unknown environment"),
        };
        Options {
            env: environment.to_string(),
            protocols: get_mainnet_protocols().await,
            layer2: get_layer2_assets(),
            protocol_name: "".to_string(),
        }
    }
    pub async fn local(env: &str, protocols: get_addresses::Protocols, protocol_name: String) {}
}

impl XendFinanceSdk {
    pub async fn new(chain_id: i32, private_key: String, options: Options) -> Self {
        const main_chainids: [i32; 3] = [ETHEREUM_MAINNET, BSC_MAINNET, POLYGON_MAINNET];
        let options = Options::new(options.env.as_str()).await;
        let chain = get_chain_id(chain_id);
        let selected_protocol = protocol_selector(options.clone()).await;

        if main_chainids.contains(&chain_id) {
            Self {
                chain_id,
                private_key,
                options: options.clone(),
                provider: chain.url,
                protocol: selected_protocol.clone().name,
                available_protocols: selected_protocol.clone().available,
                addresses: selected_protocol.clone().addresses.clone(),
                currency: chain.currency,
                share_currency: selected_protocol
                    .clone()
                    .addresses
                    .clone()
                    .protocol_currency,
            }
        } else {
            panic!("Please pass mainnet chain id");
        }
    }

    // pub async fn local(chain_id: i32, private_key: String, options: Options) -> Self {
    //     let options = Options::new(options.env.as_str()).await;
    //     Self {
    //         chain_id,
    //         private_key,
    //         options,
    //     }
    // }

    pub async fn create_wallet(&self) -> WalletKeys {
        let address = create_wallet(self.chain_id).await;
        address
    }

    pub async fn retrieve_wallet(&self) -> Address {
        let address = retrieve_address(self.private_key.as_str());
        address
    }

    pub async fn wallet_balance(&self) -> String {
        let response = get_balance(
            self.provider.clone(),
            self.private_key.as_str(),
            self.addresses.clone(),
        )
        .await;
        response.to_string()
    }

    pub async fn get_ppfs(&self) {
        get_price_per_full_share(
            self.provider.clone(),
            self.addresses.protocol_adapter.clone(),
        )
        .await;
    }

    pub async fn apys(&self) -> Vec<ApyData>{
        if self.chain_id == BSC_MAINNET {
            let res = reqwest::get(APYS_URL).await.unwrap().text().await;

            let response = match res {
                Ok(val) => val,
                Err(error) => panic!("Problem reading api: {:?}", error),
            };
            let apy: ApyRoot = serde_json::from_str(&response).unwrap();
            apy.data
        }else{
            panic!("Have to be on bsc mainnet");
        }
    }
}
#[tokio::main]
async fn main() {
    let pk: String = "".to_string();
    let option_val = Options::new("mainnet").await;
    let xf = XendFinanceSdk::new(56, pk,option_val ).await;
    print!("{:?}",  xf.wallet_balance().await);
}
