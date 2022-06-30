use crate::{environments::testnet::get_testnet_protocols, sdk::Options};

use super::get_addresses::{Addresses, BscProtocols};

#[derive(Clone, Debug)]
pub struct AvailableAddress {
    pub code: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct ProtocolSelector {
    pub name: String,
    pub addresses: Addresses,
    pub available: Vec<AvailableAddress>,
}

pub fn get_protocol_essentials(
    protocols: Vec<BscProtocols>,
    protocol_name: String,
) -> ProtocolSelector {
    let mut available: Vec<AvailableAddress> = Vec::new();
    let addresses: Addresses;
    let mut name: String = "".to_string();

    if !protocol_name.is_empty() {
        let requested_protocol: Vec<BscProtocols> = protocols.clone()
            .into_iter()
            .filter(|protocol| protocol.code == protocol_name)
            .collect();
        if requested_protocol.len() > 0 {
            let proto_obj: BscProtocols = requested_protocol[0].clone();
            addresses = proto_obj.addresses;
            name = proto_obj.name;
        } else {
            let proto_obj: BscProtocols = protocols[0].clone();
            addresses = proto_obj.addresses;
            name = proto_obj.name;
        }
    } else {
        let proto_obj: BscProtocols = protocols[0].clone();
        addresses = proto_obj.addresses;
        name = proto_obj.name;
    }
    for protocol in protocols {
        let data = AvailableAddress {
            code: protocol.code,
            name: protocol.name,
        };
        available.push(data);
    }
    let selector = ProtocolSelector {
        name: name,
        addresses: addresses,
        available: available,
    };
    selector
}

pub async fn protocol_selector(options: Options)-> ProtocolSelector{
    let protocol_name = options.protocol_name;
    let environment = options.env;
    let local_protocol = options.protocols;

    let protocol = match environment.as_str() {
        "testnet" => {
            let testnet_protocols = get_testnet_protocols();
            let protocol = get_protocol_essentials(testnet_protocols, protocol_name);
            protocol
        }
        "local" => {
            if !local_protocol.is_empty() {
                let protocol = get_protocol_essentials(local_protocol, protocol_name);
                protocol
            } else {
                panic!("Provide the protocols to be used");
            }
        }
        "mainnet" => {
            if !local_protocol.is_empty() {
                let protocol = get_protocol_essentials(local_protocol, protocol_name);
                protocol
            } else {
                panic!("Failed to initialize mainnet addresses");
            }
        }
        &_ => panic!("Unknown environment selected"),
    };
    protocol
}
