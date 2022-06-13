use try_catch::catch;

#[derive(Debug)]
pub struct x_auto {
    chain_id: i32,
    rpc: String,
    assets: Vec<crate::utilities::get_addresses::Data>,
}

const protocol: &str = "xAuto";

impl x_auto {
    fn new(
        //constructor
        chain_id: i32,
        rpc: String,
        assets: Vec<crate::utilities::get_addresses::Data>,
    ) -> x_auto {
        return x_auto {
            chain_id,
            rpc: crate::utilities::helpers::get_chain_id(chain_id).url,
            assets,
        };
    }
    
  
    // async fn approve(token_name: String, amount: String) -> String {
    //     catch! {try{
            
    //     }catch err{
    //         println!("Failed to approve transaction", err);
    //     }};
    // }

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
