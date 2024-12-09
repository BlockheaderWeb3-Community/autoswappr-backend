use crate::api_error::ApiError;

use starknet::{
    contract, core::types::{BlockId, BlockTag, Felt, FunctionCall}, macros::{felt, selector}, providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Url,
    }
};

// pub async fn call_contract() {
//     let provider = JsonRpcClient::new(HttpTransport::new(
//         Url::parse("https://free-rpc.nethermind.io/sepolia-juno/").unwrap(),
//     ));

//     let contract_address =
//         felt!("0x55abee888d949203f3973ad9ad725fd7c57c8d5da79a6756ef0473af86af863");

//     let storage_key = felt!("0x0389b73ec450d58472d9fbf78e97bcd7fe290cc445373ae2c2018c8a87f113c3");
//     let block_id = BlockId::Tag(BlockTag::Latest);

//     // Fetch storage value
//     match provider.get_storage_at(contract_address, storage_key, block_id).await {
//         Ok(call_result) => {
//             println!("Call result (raw): {:?}", call_result);

//             // Optional: Convert `call_result` to `i64` if meaningful
//             if let Some(decoded_result) = call_result.to_bytes_be().try_into().ok() {
//                 let decoded_value = i64::from_be_bytes(decoded_result);
//                 println!("Decoded call result: {}", decoded_value);
//             } else {
//                 println!("Failed to decode call result.");
//             }
//         }
//         Err(err) => {
//             eprintln!("Failed to fetch storage: {:?}", err);
//         }
//     }
// }


pub async fn call_contract(provider_url: &str, contract_address: &str ) {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://free-rpc.nethermind.io/sepolia-juno/").unwrap(),
    ));

    let contract_address =
        felt!("0x55abee888d949203f3973ad9ad725fd7c57c8d5da79a6756ef0473af86af863");

    let block_id = BlockId::Tag(BlockTag::Latest);
    let call_result =
        provider
            .get_storage_at(contract_address, felt!("0x0389b73ec450d58472d9fbf78e97bcd7fe290cc445373ae2c2018c8a87f113c3"), block_id)
            .await
            .expect("failed to call contract");
            

            // .call(
                
            //     FunctionCall {
            //         contract_address: tst_token_address,
            //         entry_point_selector: selector!("name"),
            //         calldata: vec![
            //             Felt::from_hex("0x6b94abf5540e1e0602150a650749ddad92bb784a517b28f7aa836ad7fd3c4bc").unwrap()
            //         ],
            //     },
            //     BlockId::Tag(BlockTag::Latest),
            // )
            // .await
            // .expect("failed to call contract");

    // let result: i64 = call_result;
    println!(
        "Call result: {}", call_result
        // hex::decode(call_result.to_hex()).unwrap().try_into().unwrap()
    );
    // dbg!(call_result);
}

// use starknet::core::types::FieldElement;
// use starknet::providers::{Provider, StarknetProvider};
// use starknet::signers::{LocalWallet, Signer};
// use starknet::accounts::{Account, Execution};

// async fn invoke_starknet_contract(app_state: &AppState, contract_address: &str, method_name: &str, params: Vec<FieldElement>) -> Result<(), ApiError> {
//     // Initialize the provider
//     let provider = StarknetProvider::new(&app_state.starknet_rpc_url);

//     // Initialize the wallet
//     let wallet = LocalWallet::from_private_key(app_state.private_key);

//     // Initialize the account
//     let account = Account::new(provider, wallet);

//     // Prepare the execution
//     let execution = Execution::new(contract_address)
//         .method(method_name)
//         .args(params);

//     // Execute the transaction
//     account.execute(execution).await.map_err(|_| ApiError::ContractExecutionError)?;

//     Ok(())
// }


pub fn validate_address(address: &str) -> Result<bool, ApiError> {
    if !address.starts_with("0x") && address.len() != 42 {
        return Err(ApiError::InvalidRequest(
            "Invalid token address format".to_string(),
        ));
    }

    Ok(true)
}
