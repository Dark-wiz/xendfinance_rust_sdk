mod abi;
mod utilities;

#[tokio::main]
async fn main() {
    // println!("{} group", abi::GROUPS() );
    // println!("Hello, world!");
    
  print!("{:?}", utilities::getAddresses::get_bsc_mainnet_addresses().await);
}
