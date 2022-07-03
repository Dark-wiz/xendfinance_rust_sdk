The rust implementation for the xend finance sdk 

##Add the following to your cargo dependencies:
```
[dependencies]
xendfinance_rust_sdk = { git = "https://github.com/Dark-wiz/xendfinance_rust_sdk/" }
```

include at the top of project to import sdk into project 
```
use xendfinance_rust_sdk::sdk::{Options, XendFinanceSdk};
```

##check /examples for more implementations
Implementing xAuto contract
```
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
```

```
use xendfinance_rust_sdk::sdk::{Options, XendFinanceSdk};
let pk: String = "xxxxxxx".to_string();
    let option_val = Options::new("mainnet").await;
    let xf = XendFinanceSdk::new(56, pk, option_val).await;

    let result = xf.retrieve_wallet().await; //retrieve wallet from private key
    println!("{:?}", result)
```
