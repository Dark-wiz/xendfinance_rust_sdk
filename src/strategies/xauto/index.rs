use crate::{
    strategies::create_contract::create_contract,
    utilities::{
        helpers::format_amount,
        layer2_assets::{filter_token, layer2_asset},
    },
};
use web3::{
    contract::{Contract, Options},
    types::{H256, U256, TransactionReceipt},
};

#[derive(Debug)]
pub struct x_auto {
    chain_id: i32,
    rpc: String,
    assets: layer2_asset,
}

const PROTOCOL: &str = "xAuto";

impl x_auto {
    fn new(
        //constructor
        chain_id: i32,
        rpc: String,
        assets: layer2_asset,
    ) -> x_auto {
        return x_auto {
            chain_id,
            rpc: crate::utilities::helpers::get_chain_id(chain_id).url,
            assets,
        };
    }

    async fn approve(&self, token_name: String, amount: String) {
        let token = filter_token(token_name.clone(), self.chain_id, PROTOCOL.to_string()).await;
        if token.protocol_address == "" {
            panic!("asset cannot be empty");
        }
        let token_abi_val = token.token_abi.clone();
        let approved_amount = format_amount(amount, self.chain_id, token_name.as_str());
        let contract = create_contract(
            self.rpc.clone(),
            token_abi_val.as_str(),
            token.protocol_address,
        )
        .await;
        let tx:String = match contract
            .query("approve", (approved_amount), None, Options::default(), None).await
        {
            Ok(it) => it,
            Err(err) => panic!("Couldn't get balance"),
        };
    }

    // async fn deposit(token_name: String, amount: String) -> String {
    //     catch! {try{

    //     }catch err{
    //         println!("Failed to approve transaction", err);
    //     }};
    // }

    // async fn deposit_native(token_name: String, amount: String) -> String {
    //     catch! {try{

    //     }catch err{
    //         println!("Failed to approve transaction", err);
    //     }};
    // }

    // async fn withdraw(token_name: String, amount: String) -> String {
    //     catch! {try{

    //     }catch err{
    //         println!("Failed to approve transaction", err);
    //     }};
    // }

    // async fn ppfs(token_name: String, amount: String) -> String {
    //     catch! {try{

    //     }catch err{
    //         println!("Failed to approve transaction", err);
    //     }};
    // }

    // async fn share_balance(token_name: String, amount: String) -> String {
    //     catch! {try{

    //     }catch err{
    //         println!("Failed to approve transaction", err);
    //     }};
    // }
}
