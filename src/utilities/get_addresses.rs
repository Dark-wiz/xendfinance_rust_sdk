use crate::utilities::layer2_assets;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::borrow::Borrow;
use std::fmt;

use super::helpers;
use super::helpers::PROTOCOL_ADDRESS_URL;
use super::helpers::get_address_by_name;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Protocols {
    pub data: Vec<Data>,
    pub status: String,
    #[serde(rename = "status_code")]
    pub status_code: i64,
    pub message: String,
}

#[derive(Default, fmt::Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "protocol_name")]
    pub protocol_name: String,
    pub addresses: Vec<ProtocolAddress>,
    pub deprecated: Vec<Deprecated>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolAddress {
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

//formated addresses
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BscProtocols {
    pub name: String,
    pub code: String,
    pub addresses: Addresses,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Addresses {
    pub protocol_adapter: String,
    pub protocol_service: String,
    pub groups: String,
    pub cycles: String,
    pub esusu_service: String,
    pub esusu_storage: String,
    pub esusu_adapter: String,
    pub cooperative: String,
    pub personal: String,
    pub client_record: String,
    pub xend_token: String,
    pub token: String,
    pub protocol_currency: String,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub async fn get_bsc_mainnet_addresses() -> Vec<Data> {
    let res = reqwest::get(PROTOCOL_ADDRESS_URL)
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

pub async fn get_mainnet_protocols() -> Vec<BscProtocols> {
    let data = get_bsc_mainnet_addresses().await;
    let mut new_vec: Vec<BscProtocols> = Vec::new();
    if !data.is_empty() {
        for protocol in data {
            let value = BscProtocols {
                name: helpers::capitalize_first_letter(protocol.protocol_name.clone()),
                code: protocol.protocol_name.clone(),
                addresses: Addresses {
                    protocol_adapter: get_address_by_name(
                        protocol.addresses.clone(),
                        String::from("protocol_adapter"),
                    ),
                    protocol_service: get_address_by_name(
                        protocol.addresses.clone(),
                        String::from("protocol_service"),
                    ),
                    groups: get_address_by_name(protocol.addresses.clone(), String::from("groups")),
                    cycles: get_address_by_name(protocol.addresses.clone(), String::from("cycles")),
                    esusu_service: get_address_by_name(
                        protocol.addresses.clone(),
                        String::from("esusu_service"),
                    ),
                    esusu_storage: get_address_by_name(
                        protocol.addresses.clone(),
                        String::from("esusu_storage"),
                    ),
                    esusu_adapter: get_address_by_name(
                        protocol.addresses.clone(),
                        String::from("esusu_adapter"),
                    ),
                    cooperative: get_address_by_name(
                        protocol.addresses.clone(),
                        String::from("cooperative"),
                    ),
                    personal: get_address_by_name(
                        protocol.addresses.clone(),
                        String::from("individual"),
                    ),
                    client_record: get_address_by_name(
                        protocol.addresses.clone(),
                        String::from("client_record"),
                    ),
                    xend_token: get_address_by_name(
                        protocol.addresses.clone(),
                        String::from("xend_token"),
                    ),
                    token: get_address_by_name(protocol.addresses.clone(), String::from("token")),
                    protocol_currency: get_address_by_name(
                        protocol.addresses.clone(),
                        String::from("busd"),
                    ),
                },
            };
            new_vec.push(value);
        }
        new_vec
    } else {
        panic!("Failed to return mainnet protocols");
    }
}
