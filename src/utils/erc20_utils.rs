// ref: https://www.gakonst.com/ethers-rs/getting-started/connect_to_an_ethereum_node.html
use ethers::{
    contract::{ ContractError, abigen },
    core::types::{Address, U256},
    prelude::*,
};
use std::sync::Arc;

abigen!(
    IERC20,
    r#"[
        function allowance(address owner, address spender) external view returns (uint256)
    ]"#
);

pub struct TokenAllowanceChecker<M> {
    provider: Arc<M>,
}

impl<M: Middleware> TokenAllowanceChecker<M> {
    pub fn new(provider: Arc<M>) -> Self {
        Self { provider }
    }

    pub async fn get_allowance(
        &self,
        token_address: Address,
        owner: Address,
        spender: Address,
    ) -> Result<U256, ContractError<M>> {
        if token_address == Address::zero() {
            return Err(ContractError::from(ProviderError::CustomError("(Invalid token address)".to_string())));
        }
        if owner == Address::zero() {
            return Err(ContractError::from(ProviderError::CustomError("(Invalid owner address)".to_string())));
        }
        if spender == Address::zero() {
            return Err(ContractError::from(ProviderError::CustomError("(Invalid spender address)".to_string())));
        }

        let contract = IERC20::new(token_address, Arc::clone(&self.provider));

        let allowance = contract.allowance(owner, spender).call().await?;

        Ok(allowance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::providers::{Http, Provider};
    use std::str::FromStr;

    const TEST_RPC_URL: &str = "https://sepolia.drpc.org";
    const TEST_TOKEN: &str = "0x779877A7B0D9E8603169DdbD7836e478b4624789";

    async fn setup() -> TokenAllowanceChecker<Provider<Http>> {
        let provider = Provider::<Http>::try_from(TEST_RPC_URL).expect("Failed to create provider");
        TokenAllowanceChecker::new(Arc::new(provider))
    }

    #[tokio::test]
    async fn test_zero_allowance() {
        let checker = setup().await;
        let token = Address::from_str(TEST_TOKEN).unwrap();
        let owner = Address::random();
        let spender = Address::random();

        let allowance = checker.get_allowance(token, owner, spender).await.unwrap();

        assert_eq!(allowance, U256::zero());
    }

    #[tokio::test]
    async fn test_invalid_token_address() {
        let checker = setup().await;
        let owner = Address::random();
        let spender = Address::random();

        let result = checker.get_allowance(Address::zero(), owner, spender).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_owner_address() {
        let checker = setup().await;
        let token = Address::from_str(TEST_TOKEN).unwrap();
        let spender = Address::random();

        let result = checker.get_allowance(token, Address::zero(), spender).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_spender_address() {
        let checker = setup().await;
        let token = Address::from_str(TEST_TOKEN).unwrap();
        let owner = Address::random();

        let result = checker.get_allowance(token, owner, Address::zero()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_partial_allowance() {
        let checker = setup().await;
        let token = Address::from_str(TEST_TOKEN).unwrap();
        let owner = Address::random();
        let spender = Address::random();

        let allowance = checker.get_allowance(token, owner, spender).await.unwrap();
        assert!(allowance > U256::zero());
    }

    #[tokio::test]
    async fn test_max_allowance() {
        let checker = setup().await;
        let token = Address::from_str(TEST_TOKEN).unwrap();
        let owner = Address::random();
        let spender = Address::random();

        let allowance = checker.get_allowance(token, owner, spender).await.unwrap();
        assert_eq!(allowance, U256::MAX);
    }
}
