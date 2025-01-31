use starknet::accounts::Account;
use starknet::core::codec::{Decode, Encode};
use starknet::core::types::{BlockId, BlockTag, Call, Felt, U256};
use starknet::macros::selector;

use super::starknet::{contract_address_felt, signer_account};

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
pub struct RouteParams {
    pub token_in: Felt,
    pub token_out: Felt,
    pub amount_in: U256,
    pub min_received: U256,
    pub destination: Felt,
}

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
pub struct SwapParams {
    pub token_in: Felt,
    pub token_out: Felt,
    pub rate: u32,
    pub protocol_id: u32,
    pub pool_address: Felt,
    pub extra_data: Felt,
}

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
pub struct CallData {
    route_params: RouteParams,
    swap_params: SwapParams,
    contract_address: Felt,
}

type FibrousResponse = Result<
    starknet::core::types::InvokeTransactionResult,
    starknet::accounts::AccountError<
        starknet::accounts::single_owner::SignError<starknet::signers::local_wallet::SignError>,
    >,
>;

pub async fn fibrous_swap(
    route_params: RouteParams,
    swap_params: SwapParams,
    contract_address: Felt,
) -> FibrousResponse {
    let mut account = signer_account();
    let contract_address_main = contract_address_felt();

    let call_data = CallData {
        route_params,
        swap_params,
        contract_address,
    };
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let mut serialized = vec![];
    call_data.encode(&mut serialized).unwrap();

    let swap_call = Call {
        to: contract_address_main,
        selector: selector!("swap"),
        calldata: serialized,
    };

    account.execute_v3(vec![swap_call]).send().await
}
