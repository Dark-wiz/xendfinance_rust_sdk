use crate::utilities::get_addresses::{Addresses, BscProtocols};

pub fn get_testnet_protocols() -> Vec<BscProtocols> {
    let mut testnet_protocols: Vec<BscProtocols> = Vec::new();
    let venusAddress = Addresses {
        protocol_adapter: "0xb69BF00bB8F724Bc3BfbC66A7EE423c80c303c8c".to_string(),
        protocol_service: "0x4ff202306C877841eed4d999543A916Cbde476E4".to_string(),
        groups: "0x14D765A51D8765EC6eaD1A7902F314817A9f07d0".to_string(),
        cycles: "0x66894f6a6721c154d58DaC1FCB1ae7eD786fd5aD".to_string(),
        esusu_service: "0x5401F660Dd7ED429b47E3ECe4e11801229E791Ce".to_string(),
        esusu_storage: "0xFCDb938471a16dA685c97D1f3EDd9f2C164291Bb".to_string(),
        esusu_adapter: "0xB5fd40DC3c89361FC9E6E8dF002d2d1bC8a31fDc".to_string(),
        cooperative: "0xFb348399E414489fF7d024Aa8AfB6EB757116660".to_string(),
        personal: "0xd13dE5eD810402b26B81647f490DC47F534708cE".to_string(),
        client_record: "0x0617A43dc30BaA4CF39Ad94cc0646aeD7eF23f10".to_string(),
        xend_token: "0x4644FcaE42D96A28b8C7Ca0e7D13252e4848b817".to_string(),
        token: "0x3B7Be9D18d53cb59DCbebC6E0582DED5DF07E000".to_string(),
        protocol_currency: "VBUSD".to_string(),
    };

    let venus = BscProtocols {
        name: "Venus".to_string(),
        code: "venus".to_string(),
        addresses: venusAddress,
    };

    
    let fortube_address = Addresses {
        protocol_adapter: "0x4bc6730a5e77a7f7a6dF5fbA7Df4446280657e66".to_string(),
        protocol_service: "0x7E53329e76146Fd278b2e41C12511f7AD486C8D2".to_string(),
        groups: "0x405cFF755EBB3A47726f50ad30b481d1f8A02Cf4".to_string(),
        cycles: "0xF975dd3636c3603c097CC6EED1dff25302ad4bAa".to_string(),
        esusu_service: "0x42359db1d396e8b160C450C5277cA81237AfAd0d".to_string(),
        esusu_storage: "0x9fA05476F262AF03064C45495544dCDe653E6Bb4".to_string(),
        esusu_adapter: "0x511789A809900ac0f9f6DbcB9e194410a3B886D8".to_string(),
        cooperative: "0x37EE013a327Df57668f3A860f20A51e06E0028C8".to_string(),
        personal: "0x92Da3E991A22b415Fc8EeDD91B97cA92232421Dc".to_string(),
        client_record: "0x213a8e1Fe5A74B451352d0b840f73Bb95b590C6b".to_string(),
        xend_token: "0xa3f0c50B860E79Cef044F6aF2e8ca1ABa53D6B27".to_string(),
        token: "0x3b1F033dD955f3BE8649Cc9825A2e3E194765a3F".to_string(),
        protocol_currency: "FBUSD".to_string(),
    };

    let fortube = BscProtocols {
        name: "Fortube".to_string(),
        code: "fortube".to_string(),
        addresses: fortube_address,
    };

    testnet_protocols.push(venus);
    testnet_protocols.push(fortube);
    testnet_protocols
}
