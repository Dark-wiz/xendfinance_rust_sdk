use crate::abi;
use crate::strategies::create_contract::create_contract;
use ethers_providers::Provider;
use try_catch::catch;

pub async fn get_price_per_full_share(provider: String, contract_address: String) -> String {
    // let contract =
    //     create_contract(provider, abi::protocol_adapter().as_str(), contract_address).await;
    // let get_ppfs =  match contract
    //     .method::<_, String>("GetPricePerFullShare", ()) {
    //         Ok(res)=>res,
    //         Err(_)=>panic!("Couldn't get price per full share")
    //     };
    // let value = get_ppfs.clone().call().await.unwrap();
    // value
    let val:String = String::from("test");
    val
}
