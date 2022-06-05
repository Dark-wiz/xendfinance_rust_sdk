use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::utilities::layer2_assets;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Protocols {
    pub data: Vec<Data>,
    pub status: String,
    #[serde(rename = "status_code")]
    pub status_code: i64,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "protocol_name")]
    pub protocol_name: String,
    pub addresses: Vec<Address>,
    pub deprecated: Vec<Deprecated>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub aid: String,
    pub address: String,
    pub name: String,
    pub protocol: String,
    pub from_block: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deprecated {
    pub aid: String,
    pub address: String,
    pub from_block: String,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub async fn get_bsc_mainnet_addresses() -> Vec<Data> {
    let res = reqwest::get("https://api.xend.finance/xend-finance/addresses")
        .await
        .unwrap()
        .text()
        .await;

    let response = match res {
        Ok(val) => val,
        Err(error) => panic!("Problem reading api: {:?}", error),
    };

    let protocols: Protocols = serde_json::from_str(&response).unwrap();
    protocols.data
}