#[cfg(test)]
mod account_test {
    use crate::sdk::{XendFinanceSdk, Options};
    use crate::utilities::price_per_full_share::get_price_per_full_share;
    use crate::utilities::privatekey_to_address::retrieve_address;
    use ethers::prelude::{H160, U256};
    use ethers::prelude::k256::elliptic_curve::consts::U25;
    use std::str::FromStr;


    const PK: &str = "30567ea5ec0cfd85a02f58eb7b8c4c3411328a6e8a387f57dc701f1be7a118de";
    const PROVIDER: &str = "https://bsc-dataseed.binance.org/";
    const CHAIN_ID: u64 = 56;
    const CONTRACT_ADDRESS: &str = "0x454d6F10B18f391adD499cE39aCD5bFCD424B601";

    #[test]
    fn get_address_from_pk() {
        let address = retrieve_address(PK, CHAIN_ID, PROVIDER.to_string()); //test private key, do not use in production
        println!("address: {:?}", address);
        let expected = H160::from_str("0x1D148d5419a33f3549030617Ea5191A2bB961160").unwrap();
        assert_eq!(address, expected);
    }
    
    #[tokio::test]
    async fn get_contract_price_per_full_share() {
        let options = Options::new("mainnet").await;
        let xend_finance_sdk = XendFinanceSdk::new(CHAIN_ID, PK.to_string(), options).await;
        let result = xend_finance_sdk.get_ppfs().await;
        let expected:U256 = U256::from_dec_str("0").unwrap();
        assert_ne!(result, expected);
    }
}
