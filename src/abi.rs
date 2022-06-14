use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

pub fn groups() -> String {
    let mut file = File::open("src/abis/groups.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn esusu_service() -> String {
    let mut file = File::open("src/abis/EsusuService.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn esusu_adapter() -> String {
    let mut file = File::open("src/abis/EsusuAdapter.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn esusu_storage() -> String {
    let mut file = File::open("src/abis/EsusuStorage.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn cooperative() -> String {
    let mut file = File::open("src/abis/XendFinanceGroup.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn fadapter() -> String {
    let mut file = File::open("src/abis/FortubeAdapterTestnet.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn token() -> String {
    let path: String = "src/abis/DaiContract.json".to_string();
    path
    // let mut file = File::open("src/abis/DaiContract.json").expect("File not found");
    // let mut data = String::new();
    // file.read_to_string(&mut data).expect("Error while reading file");
    // data
}
pub fn personal() -> String {
    let mut file = File::open("src/abis/XendFinanceIndividual_Yearn_V1.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn client_record() -> String {
    let mut file = File::open("src/abis/ClientRecord.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn cycles() -> String {
    let mut file = File::open("src/abis/Cycles.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn protocol_adapter() -> String {
    let mut file = File::open("src/abis/ProtocolAdapter.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn xauto() -> String {
    let mut file = File::open("src/abis/xAuto.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn xvault() -> String {
    let mut file = File::open("src/abis/xVault.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read file");
    data
}
pub fn erc_20() -> String {
    let mut file = File::open("src/abis/ER20.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn busd() -> String {
    let mut file = File::open("src/abis/busd.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
// //v2 XVault BSC
pub fn xv_vault_usdcv2() -> String {
    let mut file = File::open("src/abis/V2XVault/XVaultUSDCV2.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn xv_vault_usdtv2() -> String {
    let mut file = File::open("src/abis/V2XVault/XVaultUSDTV2.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn xv_vault_busdv2() -> String {
    let mut file = File::open("src/abis/V2XVault/XVaultBUSDV2.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}

// //v2 XAuto BSC
pub fn xv_auto_bscbusdv2() -> String {
    let mut file = File::open("src/abis/V2XAuto/xvAutoBUSDV2.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn xv_auto_bscusdcv2() -> String {
    let mut file = File::open("src/abis/V2XAuto/xvAutoUSDCV2.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn xv_auto_bscusdtv2() -> String {
    let mut file = File::open("src/abis/V2XAuto/xvAutoUSDTV2.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn xv_auto_bscbnbv2() -> String {
    let mut file = File::open("src/abis/V2XAuto/xvAutoBNBV2.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}

// //v2 XAuto Matic
pub fn xv_auto_usdcv2_matic() -> String {
    let mut file = File::open("src/abis/V2XAutoMatic/xvAutoUSDCV2.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn xv_auto_usdtv2_matic() -> String {
    let mut file = File::open("src/abis/V2XAutoMatic/xvAutoUSDTV2.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn xv_auto_aavev2_matic() -> String {
    let mut file = File::open("src/abis/V2XAutoMatic/xvAutoAAVEV2.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
pub fn xv_auto_wbtcv2_matic() -> String {
    let mut file = File::open("src/abis/V2XAutoMatic/xvAutoWBTCV2.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    data
}
