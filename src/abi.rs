use std::fs;

pub fn GROUPS() -> String {
    let groups_data =
        fs::read_to_string("src/strategies/abis/groups.json").expect("Unable to read file");
    // let json = serde_json::from_str(&groups_data).expect("JSON does not have correct format.");
    groups_data
}
pub fn ESUSU_SERVICE() -> String {
    let esusu_data =
        fs::read_to_string("src/strategies/abis/EsusuService.json").expect("Unable to read file");
    esusu_data
}
pub fn ESUSU_ADAPTER() -> String {
    let esusu_adapter =
        fs::read_to_string("src/strategies/abis/EsusuAdapter.json").expect("Unable to read file");
    esusu_adapter
}
pub fn ESUSU_STORAGE() -> String {
    let esusu_storage =
        fs::read_to_string("src/strategies/abis/EsusuStorage.json").expect("Unable to read file");
    esusu_storage
}
pub fn COOPERATIVE() -> String {
    let cooperative = fs::read_to_string("src/strategies/abis/XendFinanceGroup.json")
        .expect("Unable to read file");
    cooperative
}
pub fn FADAPTER() -> String {
    let fadapter = fs::read_to_string("src/strategies/abis/FortubeAdapterTestnet.json")
        .expect("Unable to read file");
    fadapter
}
pub fn TOKEN() -> String {
    let token =
        fs::read_to_string("src/strategies/abis/DaiContract.json").expect("Unable to read file");
    token
}
pub fn PERSONAL() -> String {
    let personal = fs::read_to_string("src/strategies/abis/XendFinanceIndividual_Yearn_V1.json")
        .expect("Unable to read file");
    personal
}
pub fn CLIENT_RECORD() -> String {
    let client_record =
        fs::read_to_string("src/strategies/abis/ClientRecord.json").expect("Unable to read file");
    client_record
}
pub fn CYCLES() -> String {
    let cycles =
        fs::read_to_string("src/strategies/abis/Cycles.json").expect("Unable to read file");
    cycles
}
pub fn PROTOCOL_ADAPTER() -> String {
    let protocol_adapter = fs::read_to_string("src/strategies/abis/ProtocolAdapter.json")
        .expect("Unable to read file");
    protocol_adapter
}
pub fn XAUTO() -> String {
    let xauto = fs::read_to_string("src/strategies/abis/xAuto.json").expect("Unable to read file");
    xauto
}
pub fn XVAULT() -> String {
    let xvault =
        fs::read_to_string("src/strategies/abis/xVault.json").expect("Unable to read file");
    xvault
}
pub fn ERC20() -> String {
    let erc20 = fs::read_to_string("src/strategies/abis/ER20.json").expect("Unable to read file");
    erc20
}
pub fn BUSD() -> String {
    let busd = fs::read_to_string("src/strategies/abis/busd.json").expect("Unable to read file");
    busd
}
// //v2 XVault BSC
pub fn xvVaultUSDCV2() -> String {
    let xvusdc = fs::read_to_string("src/strategies/abis/V2XVault/XVaultUSDCV2.json")
        .expect("Unable to read file");
    xvusdc
}
pub fn xvVaultUSDTV2() -> String {
    let xvusdt = fs::read_to_string("src/strategies/abis/V2XVault/XVaultUSDTV2.json")
        .expect("Unable to read file");
    xvusdt
}
pub fn xvVaultBUSDV2() -> String {
    let xvbusd = fs::read_to_string("src/strategies/abis/V2XVault/XVaultBUSDV2.json")
        .expect("Unable to read file");
    xvbusd
}

// //v2 XAuto BSC
pub fn xvAutoBSCBUSDV2() -> String {
    let xvbscbusd = fs::read_to_string("src/strategies/abis/V2XAuto/xvAutoBUSDV2.json")
        .expect("Unable to read file");
    xvbscbusd
}
pub fn xvAutoBSCUSDCV2() -> String {
    let xvbscusdc = fs::read_to_string("src/strategies/abis/V2XAuto/xvAutoUSDCV2.json")
        .expect("Unable to read file");
    xvbscusdc
}
pub fn xvAutoBSCUSDTV2() -> String {
    let xvbscusdt = fs::read_to_string("src/strategies/abis/V2XAuto/xvAutoUSDTV2.json")
        .expect("Unable to read file");
    xvbscusdt
}
pub fn xvAutoBSCBNBV2() -> String {
    let xvbscbnb = fs::read_to_string("src/strategies/abis/V2XAuto/xvAutoBNBV2.json")
        .expect("Unable to read file");
    xvbscbnb
}

// //v2 XAuto Matic
pub fn xvAutoUSDCV2Matic() -> String {
    let xvusdc_matic = fs::read_to_string("src/strategies/abis/V2XAutoMatic/xvAutoUSDCV2.json")
        .expect("Unable to read file");
    xvusdc_matic
}
pub fn xvAutoUSDTV2Matic() -> String {
    let xvusdt_matic = fs::read_to_string("src/strategies/abis/V2XAutoMatic/xvAutoUSDTV2.json")
        .expect("Unable to read file");
    xvusdt_matic
}
pub fn xvAutoAAVEV2Matic() -> String {
    let xvaave_matic = fs::read_to_string("src/strategies/abis/V2XAutoMatic/xvAutoAAVEV2.json")
        .expect("Unable to read file");
    xvaave_matic
}
pub fn xvAutoWBTCV2Matic() -> String {
    let xvwbtc_matic = fs::read_to_string("src/strategies/abis/V2XAutoMatic/xvAutoWBTCV2.json")
        .expect("Unable to read file");
    xvwbtc_matic
}
