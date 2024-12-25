use starknet::{
    core::types::{ Felt, BlockId, BlockTag, StarknetError, FunctionCall },
    macros::{ felt, selector },
    providers::{ jsonrpc::{ HttpTransport, JsonRpcClient }, Provider, Url },
};
use std::sync::Arc;

#[derive(Debug)]
pub struct TokenAllowanceChecker<P: Provider + Send + Sync> {
    provider: Arc<P>,
}

impl<P: Provider + Send + Sync> TokenAllowanceChecker<P> {
    pub fn new(provider: Arc<P>) -> Self {
        Self { provider }
    }

    pub async fn get_allowance( &self, token_address: Felt, owner: Felt, spender: Felt ) -> Result<Felt, StarknetError> {
        
        if token_address == Felt::ZERO {
            return Err(StarknetError::UnexpectedError("Token Address is not valid".to_string()));
        }
        if owner == Felt::ZERO {
            return Err(StarknetError::UnexpectedError("Owner Address is invalid".to_string()));
        }
        if spender == Felt::ZERO {
            return Err(StarknetError::UnexpectedError("Spender Address is not valid".to_string()));
        }

        let selector = selector!("allowance");

        let calldata = vec![owner, spender];

        let call_result = self.provider.call(
            FunctionCall {
                contract_address: token_address,
                entry_point_selector: selector,
                calldata,
            },
            BlockId::Tag(BlockTag::Latest),
        ).await
        .expect("failed to call allowance");

        Ok(call_result[0])
    }
}