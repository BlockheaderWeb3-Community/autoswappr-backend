use starknet::accounts::Account;
use starknet::core::codec::{Decode, Encode};
use starknet::core::types::{BlockId, BlockTag, Call, Felt};
use starknet::macros::selector;

use super::starknet::{contract_address_felt, signer_account};

#[derive(Debug, Clone, Copy)]
pub struct TokenFrom {
    address: Felt,
    amount: u128,
}

#[derive(Debug, Clone, Copy)]
pub struct TokenTo {
    address: Felt,
    amount: u128,
    min_amount: u128,
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct SwapData {
    pub token_from_address: Felt,
    pub token_from_amount: u128,
    pub token_to_address: Felt,
    pub token_to_amount: u128,
    pub token_to_min_amount: u128,
    pub beneficiary: Felt,
    pub integrator_fee_amount_bps: u128,
    pub integrator_fee_recipient: Felt,
    pub routes: Felt,
}

impl SwapData {
    pub fn new(
        token_from: TokenFrom,
        token_to: TokenTo,
        beneficiary: Felt,
        integrator_fee_amount_bps: u128,
        integrator_fee_recipient: Felt,
        routes: Felt,
    ) -> Self {
        SwapData {
            token_from_address: token_from.address,
            token_from_amount: token_from.amount,
            token_to_address: token_to.address,
            token_to_amount: token_to.amount,
            token_to_min_amount: token_to.min_amount,
            beneficiary,
            integrator_fee_amount_bps,
            integrator_fee_recipient,
            routes,
        }
    }
}

type AnvuResponse = Result<
    starknet::core::types::InvokeTransactionResult,
    starknet::accounts::AccountError<
        starknet::accounts::single_owner::SignError<starknet::signers::local_wallet::SignError>,
    >,
>;

pub async fn anvu_swap(
    token_from: TokenFrom,
    token_to: TokenTo,
    beneficiary: Felt,
    integrator_fee_amount_bps: u128,
    integrator_fee_recipient: Felt,
    routes: Felt,
) -> AnvuResponse {
    let mut account = signer_account();
    let contract_address = contract_address_felt();
    let swap_data = SwapData::new(
        token_from,
        token_to,
        beneficiary,
        integrator_fee_amount_bps,
        integrator_fee_recipient,
        routes,
    );

    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let mut serialized = vec![];
    swap_data.encode(&mut serialized).unwrap();

    let approve_call = Call {
        to: token_from.address,
        selector: selector!("approve"),
        calldata: vec![contract_address, Felt::from(token_from.amount)],
    };

    let swap_call = Call {
        to: contract_address,
        selector: selector!("anvu_swap"),
        calldata: serialized,
    };

    account
        .execute_v3(vec![approve_call, swap_call])
        .send()
        .await
}
