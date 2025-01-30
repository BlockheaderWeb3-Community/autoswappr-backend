use ethers::abi::{Address, Abi};
use ethers::{
    prelude::*,
    providers::Provider,
    contract::Contract,
};
use std::sync::Arc;
use std::str::FromStr;

pub async fn get_token_allowance(
    token_address: &str,
    owner: &str,
    spender: &str,
    rpc_url: &str,
) -> Result<U256, Box<dyn std::error::Error>> {
    // Create provider
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(provider);

    // ERC20 allowance function ABI
    const ALLOWANCE_ABI: &str = r#"[{
        "constant": true,
        "inputs": [
            {"name": "owner", "type": "address"},
            {"name": "spender", "type": "address"}
        ],
        "name": "allowance",
        "outputs": [{"name": "", "type": "uint256"}],
        "type": "function"
    }]"#;

    // Create contract instance
    let token_address = Address::from_str(token_address)?;
    let abi: Abi = serde_json::from_str(ALLOWANCE_ABI)?;
    let contract = Contract::new(token_address, abi, provider);

    // Call allowance function
    let owner_address = Address::from_str(owner)?;
    let spender_address = Address::from_str(spender)?;
    
    let allowance: U256 = contract
        .method("allowance", (owner_address, spender_address))?
        .call()
        .await?;

    Ok(allowance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_real_token_allowance() {
        // Load environment variables
        dotenv().ok();
        
        // Get RPC URL 
        let rpc_url = env::var("RPC_URL")
            .expect("RPC_URL must be set in environment");

        // Test addresses 
        let token_address = "0x7169D38820dfd117C3FA1f22a697dBA58d90BA06"; // Example USDT on Sepolia
        let owner = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";  // Example test wallet
        let spender = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"; // Example test wallet

        let result = get_token_allowance(
            token_address,
            owner,
            spender,
            &rpc_url
        ).await;

        match result {
            Ok(allowance) => {
                println!("Successfully retrieved allowance: {}", allowance);
                assert!(true);
            },
            Err(e) => {
                println!("Error getting allowance: {:?}", e);
                assert!(false);
            }
        }
    }
}