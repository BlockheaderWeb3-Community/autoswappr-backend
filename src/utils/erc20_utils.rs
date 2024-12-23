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

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;

    const TEST_RPC_URL: &str = "https://starknet-sepolia.public.blastapi.io";
    const TEST_TOKEN: Felt = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");

    async fn setup() -> TokenAllowanceChecker<JsonRpcClient<HttpTransport>> {
        let provider = JsonRpcClient::new(HttpTransport::new(Url::from_str(TEST_RPC_URL).unwrap()));
        TokenAllowanceChecker::new(Arc::new(provider))
    }

    #[tokio::test]
    async fn test_zero_allowance() {
        let checker = setup().await;
        let token = TEST_TOKEN;
        let owner = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
        let spender = Felt::ZERO;

        let allowance = checker.get_allowance(token, owner, spender).await;
        assert!(allowance.is_err());
    }

    #[tokio::test]
    async fn test_invalid_token_address() {
        let checker = setup().await;
        let token = Felt::ZERO;
        let owner = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
        let spender = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");

        let allowance = checker.get_allowance(token, owner, spender).await;
        assert!(allowance.is_err());
    }

    #[tokio::test]
    async fn test_invalid_owner_address() {
        let checker = setup().await;
        let token = TEST_TOKEN;
        let owner = Felt::ZERO;
        let spender = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");

        let allowance = checker.get_allowance(token, owner, spender).await;
        assert!(allowance.is_err());
    }

    #[tokio::test]
    async fn test_invalid_spender_address() {
        let checker = setup().await;
        let token = TEST_TOKEN;
        let owner = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
        let spender = Felt::ZERO;

        let allowance = checker.get_allowance(token, owner, spender).await;
        assert!(allowance.is_err());
    }
}