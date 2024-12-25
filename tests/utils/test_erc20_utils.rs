use autoswappr_backend::Configuration;
use crate::utils::erc20_utils::*;
use std::str::FromStr;
use dotenvy::dotenv;

pub fn rpc_url() -> &str {
    dotenv().ok();
    let config = Configuration::new();
    config.rpc_url.as_str()
}

const TEST_TOKEN: Felt = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");

async fn setup() -> TokenAllowanceChecker<JsonRpcClient<HttpTransport>> {
    let rpc_url = rpc_url();

    let provider = JsonRpcClient::new(HttpTransport::new(Url::from_str(rpc_url).unwrap()));
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

#[tokio::test]
async fn test_valid_allowance() {
    let checker = setup().await;
    let token = TEST_TOKEN;

    let owner = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
    let spender = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");

    let allowance = checker.get_allowance(token, owner, spender).await;
    assert!(allowance.is_ok());
}

#[tokio::test]
async fn test_non_erc20_token() {
    let checker = setup().await;

    // use a non-ERC20 token
    let token = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
    let owner = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
    let spender = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");

    let allowance = checker.get_allowance(token, owner, spender).await;
    assert!(allowance.is_err());
}

#[tokio::test]
async fn test_multi_owners_same_spender() {
    let checker = setup().await;
    let token = TEST_TOKEN;

    let owner1 = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
    let owner2 = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
    let spender = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");

    let allowance1 = checker.get_allowance(token, owner1, spender).await;
    let allowance2 = checker.get_allowance(token, owner2, spender).await;

    assert!(allowance1.is_ok());
    assert!(allowance2.is_ok());
}

#[tokio::test]
async fn test_same_owner_multi_spenders() {
    let checker = setup().await;
    let token = TEST_TOKEN;

    let owner = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
    let spender1 = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
    let spender2 = felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");

    let allowance1 = checker.get_allowance(token, owner, spender1).await;
    let allowance2 = checker.get_allowance(token, owner, spender2).await;

    assert!(allowance1.is_ok());
    assert!(allowance2.is_ok());
}
