// use std::{default, env, str::FromStr};

use crate::api_error::ApiError;
use crate::config::env_var;

use starknet::{
    accounts::Account,
    core::{
        chain_id,
        types::{BlockId, BlockTag, Call, EventFilter, Felt, FunctionCall},
        utils::get_selector_from_name,
    },
    macros::{felt, selector},
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Url,
    },
};

use starknet::accounts::{ExecutionEncoding, SingleOwnerAccount};

use starknet::signers::{LocalWallet, SigningKey};

pub async fn swap() -> Result<(), ApiError> {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&env_var("STARKNET_RPC_URL")).unwrap(),
    ));

    let private_key =
        Felt::from_hex(env_var("PRIVATE_KEY").trim()).expect("Invalid PRIVATE_KEY format");

    // println!("Priv: {}", private_key);

    // let private_key: Felt = felt!(priv_key.trim_start_matches("0x").as_str());

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(private_key));
    let account_address = Felt::from_hex(env_var("OWNER_ADDRESS").as_str()).unwrap();

    let id: Felt = match env_var("APP_ENVIRONMENT").as_str() {
        "production" => chain_id::MAINNET,

        _ => chain_id::SEPOLIA,
    };

    let account = SingleOwnerAccount::new(
        provider.clone(),
        signer,
        account_address,
        id,
        ExecutionEncoding::New,
    );

    let contract_address = Felt::from_hex(env_var("AUTOSWAP_CONTRACT_ADDRESS").as_str()).unwrap();

    let erc20_event_selector = get_selector_from_name("ERC20Event").expect("Invalid event name");

    let filter: EventFilter = EventFilter {
        from_block: None,
        to_block: None,
        address: Some(contract_address),
        keys: Some(vec![vec![erc20_event_selector]]),
    };
    let continuation_token: Option<String> = None;
    let chunk_size: u64 = 10;

    let stream = provider
        .get_events(filter.clone(), continuation_token.clone(), chunk_size)
        .await
        .expect("Failed to get events");

    println!("Listening for events...");
    loop {
        let response = provider
            .get_events(filter.clone(), continuation_token.clone(), chunk_size)
            .await
            .expect("Failed to fetch events");

        let events = response.events;
        for event in events {
            println!("Event received: {:#?}", event);
        }

        // Check for pagination
        // continuation_token = response.continuation_token;
        // if continuation_token.is_none() {
        //     continue;
        // }
    }
}
