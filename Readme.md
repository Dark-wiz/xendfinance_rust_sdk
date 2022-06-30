The rust implementation for the xend finance sdk 

##setup
[dependencies]
xendfinance_rust_sdk = { git = "https://github.com/Dark-wiz/xendfinance_rust_sdk/" }


##XVault implementation
#[tokio::main]
async fn main() {
    let pk: String = "xxxxxxxx".to_string();//private_key
    let option_val = Options::new("mainnet").await;
    let xf = XendFinanceSdk::new(56, pk, option_val).await;

    let token_name = "USDT".to_string();
    let amount = "0.005".to_string();
    let response = match xf.x_vault.approve(token_name, amount).await {
         Ok(val) => val,
        Err(error) => panic!("Problem reading transaction: {:?}", error),
    };
    let response = match xf.x_vault.deposit(token_name, amount).await {
         Ok(val) => val,
        Err(error) => panic!("Problem reading transaction: {:?}", error),
    };
    let response = match xf.x_vault.withdraw(token_name, amount).await {
         Ok(val) => val,
        Err(error) => panic!("Problem reading transaction: {:?}", error),
    };
    println!("{:#?}", response);
    print!("{:?}",  xf.x_vault.share_balance(token_name).await);
    print!("{:?}",  xf.retrieve_wallet().await);
}


##XAuto implementation
#[tokio::main]
async fn main() {
    let pk: String = "xxxxxxxx".to_string();//private_key
    let option_val = Options::new("mainnet").await;
    let xf = XendFinanceSdk::new(56, pk, option_val).await;

    let token_name = "USDT".to_string();
    let amount = "0.005".to_string();
    let response = match xf.x_auto.approve(token_name, amount).await {
         Ok(val) => val,
        Err(error) => panic!("Problem reading transaction: {:?}", error),
    };
    let response = match xf.x_auto.deposit(token_name, amount).await {
         Ok(val) => val,
        Err(error) => panic!("Problem reading transaction: {:?}", error),
    };
    let response = match xf.x_auto.withdraw(token_name, amount).await {
         Ok(val) => val,
        Err(error) => panic!("Problem reading transaction: {:?}", error),
    };
    println!("{:#?}", response);
    print!("{:?}",  xf.x_auto.share_balance(token_name).await);
    print!("{:?}",  xf.retrieve_wallet().await);
}