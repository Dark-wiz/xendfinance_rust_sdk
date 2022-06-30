use super::super::abi;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer2Asset {
    pub name: String,
    pub logo: String,
    pub token_address: String,
    pub token_abi: String,
    pub protocol_name: String,
    pub protocol_address: String,
    pub protocol_abi: String,
    pub network: u64,
    pub decimals: i32,
    pub widthdraw_decimals: u32,
    pub ppfs_method: String,
}

//bsc
const BUSD_BSC_XVAULT: &str = "0x3de1Fe0039EC99773DBEE5608823FECDeFB8D9D0";
const USDC_BSC_XVAULT: &str = "0x50c9fBf77CBC8FF1b23a8ED61725C325bedC3C86";
const USDT_BSC_XVAULT: &str = "0x454d6F10B18f391adD499cE39aCD5bFCD424B601";

const BUSD_BSC_XAUTO: &str = "0x0f28698FD6A0771CB099482305BeEd0EeCB458D5";
const USDC_BSC_XAUTO: &str = "0xa3003c67C0C8fF2280b282F0A821e95fEBA47293";
const USDT_BSC_XAUTO: &str = "0x9607be08acFeB47Ea7e66b494Dd5dAb88Dda59cf";
const USDT_BNB_XAUTO: &str = "0x8C709c792700d73e37D8B0A4CD3bcc995d03f084";

//matic
const USDT_MATIC_XAUTO: &str = "0x143afc138978Ad681f7C7571858FAAA9D426CecE";
const USDC_MATIC_XAUTO: &str = "0xd01a0971F03D0ddC8D621048d92A1632b2dB7356";
const AAVE_MATIC_XAUTO: &str = "0xDD3afc5D5476FC327812B84ae2ccf66C011e6d67";
const WBTC_MATIC_XAUTO: &str = "0x0b26E76D8617b20Ec9fe0811BE2dCbF3438cc27F";

//Token Addresses
const BUSD_BSC: &str = "0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56";
const USDC_BSC: &str = "0x8AC76a51cc950d9822D68b83fE1Ad97B32Cd580d";
const USDT_BSC: &str = "0x55d398326f99059fF775485246999027B3197955";

const AAVE_MATIC: &str = "0xd6df932a45c0f255f85145f286ea0b292b21c90b";
const WBTC_MATIC: &str = "0x1bfd67037b42cf73acf2047067bd4f2c47d9bfd6";
const USDT_MATIC: &str = "0xc2132D05D31c914a87C6611C10748AEb04B58e8F";
const USDC_MATIC: &str = "0x2791bca1f2de4661ed88a30c99a7a9449aa84174";

const ERROR: &str = "";

pub fn get_layer2_assets() -> [Layer2Asset; 11] {
    let layer2Assets = [
        Layer2Asset {
            name: "USDC".to_string(),
            logo: "".to_string(),
            token_address: USDC_BSC.to_string(),
            token_abi: abi::erc_20(),
            protocol_name: "xVault".to_string(),
            protocol_address: USDC_BSC_XVAULT.to_string(),
            protocol_abi: abi::xv_vault_usdcv2(),
            network: 56,
            decimals: 18,
            widthdraw_decimals: 36,
            ppfs_method: "pricePerShare".to_string(),
        },
        Layer2Asset {
            name: "USDT".to_string(),
            logo: "".to_string(),
            token_address: USDT_BSC.to_string(),
            token_abi: abi::erc_20(),
            protocol_name: "xVault".to_string(),
            protocol_address: USDT_BSC_XVAULT.to_string(),
            protocol_abi: abi::xv_vault_usdtv2(),
            network: 56,
            decimals: 18,
            widthdraw_decimals: 36,
            ppfs_method: "pricePerShare".to_string(),
        },
        Layer2Asset {
            name: "BUSD".to_string(),
            logo: "".to_string(),
            token_address: BUSD_BSC.to_string(),
            token_abi: abi::erc_20(),
            protocol_name: "xVault".to_string(),
            protocol_address: BUSD_BSC_XVAULT.to_string(),
            protocol_abi: abi::xv_vault_busdv2(),
            network: 56,
            decimals: 18,
            widthdraw_decimals: 36,
            ppfs_method: "pricePerShare".to_string(),
        },
        Layer2Asset {
            name: "BUSD".to_string(),
            logo: "".to_string(),
            token_address: BUSD_BSC.to_string(),
            token_abi: abi::busd(),
            protocol_name: "xAuto".to_string(),
            protocol_address: BUSD_BSC_XAUTO.to_string(),
            protocol_abi: abi::xv_auto_bscbusdv2(),
            network: 56,
            decimals: 18,
            widthdraw_decimals: 36,
            ppfs_method: "getPricePerFullShare".to_string(),
        },
        Layer2Asset {
            name: "USDT".to_string(),
            logo: "".to_string(),
            token_address: USDT_BSC.to_string(),
            token_abi: abi::erc_20(),
            protocol_name: "xAuto".to_string(),
            protocol_address: USDT_BSC_XAUTO.to_string(),
            protocol_abi: abi::xv_auto_bscusdtv2(),
            network: 56,
            decimals: 18,
            widthdraw_decimals: 36,
            ppfs_method: "getPricePerFullShare".to_string(),
        },
        Layer2Asset {
            name: "BNB".to_string(),
            logo: "".to_string(),
            token_address: "".to_string(),
            token_abi: abi::erc_20(),
            protocol_name: "xAuto".to_string(),
            protocol_address: USDT_BNB_XAUTO.to_string(),
            protocol_abi: abi::xv_auto_bscbnbv2(),
            network: 56,
            decimals: 18,
            widthdraw_decimals: 36,
            ppfs_method: "getPricePerFullShare".to_string(),
        },
        Layer2Asset {
            name: "USDC".to_string(),
            logo: "".to_string(),
            token_address: USDC_BSC.to_string(),
            token_abi: abi::erc_20(),
            protocol_name: "xAuto".to_string(),
            protocol_address: USDC_BSC_XAUTO.to_string(),
            protocol_abi: abi::xv_auto_bscusdcv2(),
            network: 56,
            decimals: 18,
            widthdraw_decimals: 36,
            ppfs_method: "getPricePerFullShare".to_string(),
        },
        Layer2Asset {
            name: "USDC".to_string(),
            logo: "".to_string(),
            token_address: USDC_MATIC.to_string(),
            token_abi: abi::erc_20(),
            protocol_name: "xAuto".to_string(),
            protocol_address: USDC_MATIC_XAUTO.to_string(),
            protocol_abi: abi::xv_auto_usdcv2_matic(),
            network: 137,
            decimals: 18,
            widthdraw_decimals: 24,
            ppfs_method: "getPricePerFullShare".to_string(),
        },
        Layer2Asset {
            name: "USDT".to_string(),
            logo: "".to_string(),
            token_address: USDT_MATIC.to_string(),
            token_abi: abi::erc_20(),
            protocol_name: "xAuto".to_string(),
            protocol_address: USDT_MATIC_XAUTO.to_string(),
            protocol_abi: abi::xv_auto_usdtv2_matic(),
            network: 137,
            decimals: 18,
            widthdraw_decimals: 24,
            ppfs_method: "getPricePerFullShare".to_string(),
        },
        Layer2Asset {
            name: "AAVE".to_string(),
            logo: "".to_string(),
            token_address: AAVE_MATIC.to_string(),
            token_abi: abi::erc_20(),
            protocol_name: "xAuto".to_string(),
            protocol_address: AAVE_MATIC_XAUTO.to_string(),
            protocol_abi: abi::xv_auto_aavev2_matic(),
            network: 137,
            decimals: 18,
            widthdraw_decimals: 36,
            ppfs_method: "getPricePerFullShare".to_string(),
        },
        Layer2Asset {
            name: "WBTC".to_string(),
            logo: "".to_string(),
            token_address: WBTC_MATIC.to_string(),
            token_abi: abi::erc_20(),
            protocol_name: "xAuto".to_string(),
            protocol_address: WBTC_MATIC_XAUTO.to_string(),
            protocol_abi: abi::xv_auto_wbtcv2_matic(),
            network: 137,
            decimals: 18,
            widthdraw_decimals: 26,
            ppfs_method: "getPricePerFullShare".to_string(),
        },
    ];
    layer2Assets
}


pub async fn filter_token(token_name: String, network: u64, protocol: String) -> Layer2Asset {
    let result: [Layer2Asset; 11] = get_layer2_assets();
    let asset_result = result
        .into_iter()
        .filter(|pro| {
            pro.name == token_name && pro.network == network && pro.protocol_name == protocol
        })
        .last();
    let asset:Layer2Asset = match asset_result {
        None => Layer2Asset{
            name: "error".to_string(),
            logo: "".to_string(),
            token_address: ERROR.to_string(),
            token_abi: abi::erc_20(),
            protocol_name: "error".to_string(),
            protocol_address: ERROR.to_string(),
            protocol_abi: "error".to_string(),
            network: 0,
            decimals: 18,
            widthdraw_decimals: 0,
            ppfs_method: "error".to_string(),
        },
        Some(asset_result)=>asset_result,
    };
    asset
}
