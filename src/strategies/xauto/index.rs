use std::{ops::Div, str::FromStr};

use crate::{
    strategies::create_contract::create_contract,
    utilities::{
        helpers::{format_amount, TransactionResult, SUCCESS_TX},
        layer2_assets::{filter_token, layer2_asset},
        privatekey_to_address::retrieve_address,
    },
};
use ethers::{
    abi::Address,
    prelude::{TransactionReceipt, U256},
};
use eyre::Result;

#[derive(Clone, Debug)]
pub struct x_auto {
    private_key: String,
    chain_id: u64,
    rpc: String,
    assets: [layer2_asset; 11],
}

const PROTOCOL: &str = "xAuto";

impl x_auto {
    pub fn new(
        //constructor
        private_key: String,
        chain_id: u64,
        rpc: String,
        assets: [layer2_asset; 11],
    ) -> x_auto {
        return x_auto {
            private_key,
            chain_id,
            rpc: crate::utilities::helpers::get_chain_id(chain_id).url,
            assets,
        };
    }

    pub async fn approve(&self, token_name: String, amount: String) -> Result<TransactionResult> {
        let token = filter_token(token_name.clone(), self.chain_id, PROTOCOL.to_string()).await;
        if token.protocol_address == "" {
            panic!("asset cannot be empty");
        }
        let token_abi_val = token.token_abi.clone();
        let contract = create_contract(
            self.rpc.clone(),
            token_abi_val.as_str(),
            token.token_address.clone(),
            self.private_key.as_str().clone(),
            self.chain_id.clone(),
        )
        .await;

        let convert_amount = ether_converter::to_wei(amount.as_str(), "ether");
        let amount_in_wei = U256::from_dec_str(&convert_amount).unwrap();
        println!("new_amount {:?}", amount_in_wei);
        let token_addr = Address::from_str(token.protocol_address.clone().as_str()).unwrap();

        let tx = contract
            .method::<_, bool>("approve", (token_addr, amount_in_wei).to_owned())?
            .legacy();

        let receipt: TransactionReceipt = tx.send().await?.await?.unwrap(); // Send transaction
        let result = TransactionResult {
            transaction_hash: receipt.transaction_hash.to_string(),
            status: true,
        };
        Ok(result)
    }

    pub async fn deposit(&self, token_name: String, amount: String) -> Result<TransactionResult> {
        let token = filter_token(token_name.clone(), self.chain_id, PROTOCOL.to_string()).await;
        if token.protocol_address == "" {
            panic!("asset cannot be empty");
        }
        let contract = create_contract(
            self.rpc.clone(),
            token.protocol_abi.as_str(),
            token.protocol_address.clone(),
            self.private_key.as_str().clone(),
            self.chain_id.clone(),
        )
        .await;

        let convert_amount = ether_converter::to_wei(amount.as_str(), "ether");
        let amount_in_wei = U256::from_dec_str(&convert_amount).unwrap();
        println!("new_amount {:?}", amount_in_wei);

        let tx = contract
            .method::<_, U256>("deposit", (amount_in_wei).to_owned())?
            .legacy();

        let receipt: TransactionReceipt = tx.send().await?.await?.unwrap(); // Send transaction
        println!("Tx Hash: {:#x}", receipt.transaction_hash);

        let result = TransactionResult {
            transaction_hash: receipt.transaction_hash.to_string(),
            status: true,
        };
        Ok(result)
    }

    pub async fn withdraw(&self, token_name: String, amount: String) -> Result<TransactionResult> {
        let token = filter_token(token_name.clone(), self.chain_id, PROTOCOL.to_string()).await;
        if token.protocol_address == "" {
            panic!("asset cannot be empty");
        }
        let contract = create_contract(
            self.rpc.clone(),
            token.protocol_abi.as_str(),
            token.protocol_address.clone(),
            self.private_key.as_str().clone(),
            self.chain_id.clone(),
        )
        .await;

        let client_address =
            retrieve_address(&self.private_key, self.chain_id, PROTOCOL.to_string());

        let share = contract
            .method::<_, U256>("balanceOf", client_address)?
            .call()
            .await?; //get users shares

        let ppfs = contract
            .method::<_, U256>("getPricePerFullShare", ())?
            .call()
            .await?; //get price per full shares

        println!("share-{:?}, ppfs-{:?}", share, ppfs);

        let withdrawal_decimal = 36;
        let divisor = u128::pow(10, withdrawal_decimal) as f64;
        // let new_divsor = U256::from_dec_str(&divisor).unwrap();
        println!("divisor-{:?}", divisor);
        let share_ppfs = share.checked_mul(ppfs).unwrap(); //total shares
        let new_share_ppfs = U256::as_u128(&share_ppfs) as f64;
        let total_deposit = new_share_ppfs.div(divisor);
        println!(
            "share_ppfs-{:?}, total_deposit-{:?}",
            new_share_ppfs, total_deposit
        );
        let convert_amount_to_wei = ether_converter::to_wei(amount.as_str(), "ether");
        let new_amount_u256 = U256::from_dec_str(convert_amount_to_wei.as_str()).unwrap();
        let new_amount = U256::as_u128(&new_amount_u256) as f64;

        let user_shares = new_share_ppfs * new_amount; //returns user share amoount
        println!("user_shares-{:?}", user_shares);

        let withdrawal_amount = (user_shares / total_deposit).floor(); //return withdrawable amount
                                                                       // let new_float = withdrawal_amount.parse().unwrap();
        println!("withdrawal_amount - {:?}", withdrawal_amount);
        let final_val = withdrawal_amount.div(1e54).to_string();
        println!("withdrawal_amount formatted -{:?}", final_val);

        let convert_amount = ether_converter::to_wei(&final_val, "ether");
        let amount_in_wei = U256::from_dec_str(&convert_amount).unwrap();
        println!("new_amount {:?}", amount_in_wei);

        let tx = contract
            .method::<_, U256>("withdraw", (amount_in_wei).to_owned())?
            .legacy();

        let receipt: TransactionReceipt = tx.send().await?.await?.unwrap(); // Send transaction
        println!("Tx Hash: {:#x}", receipt.transaction_hash);

        let result = TransactionResult {
            transaction_hash: receipt.transaction_hash.to_string(),
            status: true,
        };
        Ok(result)
    }

    pub async fn ppfs(&self, token_name: String) -> U256 {
        let token = filter_token(token_name.clone(), self.chain_id, PROTOCOL.to_string()).await;
        if token.protocol_address == "" {
            panic!("asset cannot be empty");
        }

        let contract = create_contract(
            self.rpc.clone(),
            token.protocol_abi.as_str(),
            token.protocol_address.clone(),
            self.private_key.as_str().clone(),
            self.chain_id.clone(),
        )
        .await;

        let val = match contract
            .method::<_, U256>(&token.ppfs_method, ())
            .unwrap()
            .call()
            .await
        {
            Ok(it) => it,
            Err(err) => panic!("Couldn't get balance: {:?}", err),
        };
        val
    }

    pub async fn share_balance(&self, token_name: String) -> U256 {
        let token = filter_token(token_name.clone(), self.chain_id, PROTOCOL.to_string()).await;
        if token.protocol_address == "" {
            panic!("asset cannot be empty");
        }

        let contract = create_contract(
            self.rpc.clone(),
            token.protocol_abi.as_str(),
            token.protocol_address.clone(),
            self.private_key.as_str().clone(),
            self.chain_id.clone(),
        )
        .await;

        let address = retrieve_address(&self.private_key, self.chain_id, self.rpc.clone());

        let val = match contract
            .method::<_, U256>("balanceOf", (address))
            .unwrap()
            .call()
            .await
        {
            Ok(it) => it,
            Err(err) => panic!("Couldn't get balance: {:?}", err),
        };
        val
    }
}
