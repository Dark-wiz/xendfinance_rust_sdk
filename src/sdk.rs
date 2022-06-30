use crate::strategies;
use crate::utilities;

use ethers::prelude::Address;
use strategies::xauto::index::XAuto;
use strategies::xvault::index::XVault;
use utilities::{
    balance::get_balance,
    helpers::{ApyData, ApyRoot, APYS_URL},
    *,
};

use utilities::{
    get_addresses::{get_mainnet_protocols, Addresses, ProtocolAddress},
    helpers::{get_chain_id, BSC_MAINNET, ETHEREUM_MAINNET, POLYGON_MAINNET},
    layer2_assets::get_layer2_assets,
    price_per_full_share::get_price_per_full_share,
    privatekey_to_address::retrieve_address,
    protocol_selector::{protocol_selector, AvailableAddress},
};


#[derive(Clone, Debug)]
pub struct XendFinanceSdk {
    pub chain_id: u64,
    pub private_key: String,
    pub options: Options,
    pub provider: String,
    pub protocol: String,
    pub available_protocols: Vec<AvailableAddress>,
    pub addresses: Addresses,
    pub currency: String,
    pub share_currency: String,
    pub x_auto: XAuto,
    pub x_vault: XVault,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub env: String,
    pub protocol_name: String,
    pub protocols: Vec<get_addresses::BscProtocols>,
    pub layer2: [layer2_assets::Layer2Asset; 11], // layer2_assets::get_layer2_assets(),
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
    pub async fn new(chain_id: u64, private_key: String, options: Options) -> Self {
        const MAIN_CHAINIDS: [u64; 3] = [ETHEREUM_MAINNET, BSC_MAINNET, POLYGON_MAINNET];
        let options = Options::new(options.env.as_str()).await;
        let chain = get_chain_id(chain_id);
        let selected_protocol = protocol_selector(options.clone()).await;
        let xauto_init = XAuto::new(
            private_key.clone(),
            chain.chain,
            chain.url.clone(),
            options.layer2.clone(),
        );
        let xvault_init = XVault::new(
            private_key.clone(),
            chain.chain,
            chain.url.clone(),
            options.layer2.clone(),
        );

        if MAIN_CHAINIDS.contains(&chain_id) {
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
                x_auto: xauto_init,
                x_vault: xvault_init,
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

    // pub async fn create_wallet(&self) -> WalletKeys {
    //     let address = create_wallet(self.chain_id.clone(), self.provider.clone()).await;
    //     address
    // }

    pub async fn retrieve_wallet(&self) -> Address {
        let address = retrieve_address(
            self.private_key.as_str(),
            self.chain_id,
            self.provider.clone(),
        );
        address
    }

    pub async fn wallet_balance(&self) -> String {
        let response = get_balance(
            self.provider.clone(),
            self.private_key.as_str(),
            self.addresses.clone(),
            self.chain_id,
        )
        .await;
        response.to_string()
    }

    pub async fn get_ppfs(&self) {
        get_price_per_full_share(
            self.provider.clone(),
            self.addresses.protocol_adapter.clone(),
            self.private_key.as_str().clone(),
            self.chain_id,
        )
        .await;
    }

    pub async fn apys(&self) -> Vec<ApyData> {
        if self.chain_id == BSC_MAINNET {
            let res = reqwest::get(APYS_URL).await.unwrap().text().await;

            let response = match res {
                Ok(val) => val,
                Err(error) => panic!("Problem reading api: {:?}", error),
            };
            let apy: ApyRoot = serde_json::from_str(&response).unwrap();
            apy.data
        } else {
            panic!("Have to be on bsc mainnet");
        }
    }
}