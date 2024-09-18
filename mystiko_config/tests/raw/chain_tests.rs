use lazy_static::lazy_static;
use mystiko_config::{
    create_raw, create_raw_from_file, RawAssetConfig, RawChainConfig, RawDepositContractConfig,
    RawPoolContractConfig, RawProviderConfig, EXPLORER_DEFAULT_PREFIX,
};
use mystiko_types::{AssetType, BridgeType, ContractType, ProviderType, TransactionType};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use validator::Validate;

fn init_provider_config() -> RawProviderConfig {
    create_raw::<RawProviderConfig>(
        RawProviderConfig::builder()
            .url("wss://goerli.infura.io/ws/v3/9aa3d95b3bc440fa88ea12eaa4456161".to_string())
            .timeout_ms(5000)
            .max_try_count(5)
            .build(),
    )
    .unwrap()
}

fn init_deposit_contract_config() -> RawDepositContractConfig {
    let raw_deposit_contract_config = RawDepositContractConfig::builder()
        .version(2)
        .name("MystikoWithPolyERC20".to_string())
        .address("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string())
        .contract_type(ContractType::Deposit)
        .start_block(1000000)
        .bridge_type(BridgeType::Tbridge)
        .pool_address("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string())
        .disabled_at(Some(1001000))
        .peer_chain_id(Some(97))
        .peer_contract_address(Some(
            "0x98bF2d9e3bA2A8515E660BD4104432ce3e2D7547".to_string(),
        ))
        .min_amount("10000000000000000".to_string())
        .max_amount("100000000000000000".to_string())
        .min_bridge_fee("20000000000000000".to_string())
        .min_executor_fee("30000000000000000".to_string())
        .build();
    create_raw::<RawDepositContractConfig>(raw_deposit_contract_config).unwrap()
}

fn init_pool_contract_config() -> RawPoolContractConfig {
    create_raw::<RawPoolContractConfig>(
        RawPoolContractConfig::builder()
            .version(2)
            .name("CommitmentPool".to_string())
            .address("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string())
            .contract_type(ContractType::Pool)
            .start_block(1000000)
            .pool_name("A Pool(since 07/20/2022)".to_string())
            .bridge_type(BridgeType::Tbridge)
            .asset_address(Some(String::from(
                "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a",
            )))
            .min_rollup_fee("40000000000000000".to_string())
            .circuits(vec![String::from("circuit-1.0")])
            .build(),
    )
    .unwrap()
}

fn init_assets_config() -> RawAssetConfig {
    create_raw::<RawAssetConfig>(
        RawAssetConfig::builder()
            .asset_type(AssetType::Erc20)
            .asset_symbol("MTT".to_string())
            .asset_symbol_alias(vec![])
            .asset_decimals(16)
            .asset_address("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string())
            .build(),
    )
    .unwrap()
}

fn default_config() -> RawChainConfig {
    let provider_config = init_provider_config();
    let deposit_contract_config = init_deposit_contract_config();
    let pool_contract_config = init_pool_contract_config();
    let asset_config = init_assets_config();
    let raw_chain_config = RawChainConfig::builder()
        .chain_id(5)
        .name("Ethereum Goerli".to_string())
        .asset_symbol("ETH".to_string())
        .asset_symbol_alias(vec![])
        .asset_decimals(18)
        .recommended_amounts(vec![
            "1000000000000000000".to_string(),
            "10000000000000000000".to_string(),
        ])
        .explorer_url("https://goerli.etherscan.io".to_string())
        .explorer_api_url("https://api-goerli.etherscan.io".to_string())
        .explorer_prefix("/tx/%tx%".to_string())
        .event_delay_blocks(200)
        .event_filter_size(1000)
        .sequencer_fetch_size(20000)
        .providers(vec![Arc::new(provider_config)])
        .provider_type(ProviderType::Quorum)
        .provider_quorum_percentage(80)
        .signer_endpoint("https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161".to_string())
        .transaction_type(TransactionType::Legacy)
        .deposit_contracts(vec![Arc::new(deposit_contract_config)])
        .pool_contracts(vec![Arc::new(pool_contract_config)])
        .assets(vec![Arc::new(asset_config)])
        .packer_granularities(vec![2000, 4000, 8000, 16000])
        .safe_confirmations(Some(10_u64))
        .build();
    create_raw::<RawChainConfig>(raw_chain_config).unwrap()
}

lazy_static! {
    static ref RAW_CONFIG: RawChainConfig = default_config();
}

#[test]
fn test_default_values() {
    let provider_config = init_provider_config();
    let deposit_contract_config = init_deposit_contract_config();
    let pool_contract_config = init_pool_contract_config();
    let asset_config = init_assets_config();
    let raw_config = RawChainConfig::builder()
        .chain_id(5)
        .name("Ethereum Goerli".to_string())
        .asset_symbol("ETH".to_string())
        .asset_symbol_alias(vec![])
        .asset_decimals(18)
        .recommended_amounts(vec![
            "1000000000000000000".to_string(),
            "10000000000000000000".to_string(),
        ])
        .explorer_url("https://goerli.etherscan.io".to_string())
        .explorer_api_url("https://api-goerli.etherscan.io".to_string())
        .providers(vec![Arc::new(provider_config)])
        .signer_endpoint("https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161".to_string())
        .deposit_contracts(vec![Arc::new(deposit_contract_config)])
        .pool_contracts(vec![Arc::new(pool_contract_config)])
        .assets(vec![Arc::new(asset_config)])
        .packer_granularities(vec![2000, 4000, 8000, 16000])
        .build();
    assert_eq!(raw_config.event_filter_size, 200000);
    assert_eq!(raw_config.sequencer_fetch_size, 500000);
    assert_eq!(raw_config.explorer_prefix, EXPLORER_DEFAULT_PREFIX);
}

#[test]
fn test_hash() {
    let config1 = &RAW_CONFIG;
    let mut hasher = DefaultHasher::new();
    config1.hash(&mut hasher);
    let hash1 = hasher.finish();

    hasher = DefaultHasher::new();
    let mut config2 = default_config();
    config2.chain_id = 97;
    config2.hash(&mut hasher);
    let hash2 = hasher.finish();

    assert_ne!(hash1, hash2);
}

#[test]
fn test_invalid_name() {
    let mut config = default_config();
    config.name = String::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_asset_symbol() {
    let mut config = default_config();
    config.asset_symbol = String::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_asset_decimals() {
    let mut config = default_config();
    config.asset_decimals = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_recommended_amounts_0() {
    let mut config = default_config();
    config.recommended_amounts = vec![String::from("")];
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_recommended_amounts_1() {
    let mut config = default_config();
    config.recommended_amounts = vec![String::from("abcd")];
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_recommended_amounts_2() {
    let mut config = default_config();
    config.recommended_amounts = vec![String::from("1"), String::from("1")];
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_explore_url_0() {
    let mut config = default_config();
    config.explorer_url = String::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_explore_url_1() {
    let mut config = default_config();
    config.explorer_url = String::from("wrong url");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_explore_api_url_0() {
    let mut config = default_config();
    config.explorer_api_url = String::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_explore_api_url_1() {
    let mut config = default_config();
    config.explorer_api_url = String::from("wrong url");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_explore_prefix_0() {
    let mut config = default_config();
    config.explorer_prefix = String::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_explore_prefix_1() {
    let mut config = default_config();
    config.explorer_prefix = String::from("wrong prefix");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_providers_0() {
    let mut config = default_config();
    config.providers = vec![];
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_providers_1() {
    let mut config = default_config();
    let mut provider_config = (*config.providers.remove(0)).clone();
    provider_config.url = String::from("wrong url");
    config.providers.insert(0, Arc::new(provider_config));
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_provider_quorum_percentage() {
    let mut config = default_config();
    config.provider_quorum_percentage = 10;
    assert!(config.validate().is_err());
    config.provider_quorum_percentage = 101;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_signer_endpoint_0() {
    let mut config = default_config();
    config.signer_endpoint = String::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_signer_endpoint_1() {
    let mut config = default_config();
    config.signer_endpoint = String::from("wrong url");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_signer_endpoint_2() {
    let mut config = default_config();
    config.signer_endpoint = String::from("wrong_schema://127.0.0.1");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_event_filter_size() {
    let mut config = default_config();
    config.event_filter_size = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_sequencer_fetch_size() {
    let mut config = default_config();
    config.sequencer_fetch_size = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_pool_contracts_0() {
    let mut config = default_config();
    config
        .pool_contracts
        .push(Arc::new(init_pool_contract_config()));
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_pool_contracts_1() {
    let mut config = default_config();
    let mut pool_contract = (*config.pool_contracts.remove(0)).clone();
    pool_contract.asset_address = Some(String::from("0xdeadbeef"));
    config.pool_contracts.insert(0, Arc::new(pool_contract));
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_deposit_contracts() {
    let mut config = default_config();
    config
        .deposit_contracts
        .push(Arc::new(init_deposit_contract_config()));
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_assets() {
    let mut config = default_config();
    config.assets.push(Arc::new(init_assets_config()));
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_granularities() {
    let mut config = default_config();
    config.packer_granularities = vec![];
    assert!(config.validate().is_err());
    config.packer_granularities = vec![1000u64, 1000u64];
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = create_raw_from_file::<RawChainConfig>("tests/files/chain/valid.json")
        .await
        .unwrap();
    assert_eq!(file_config, default_config())
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        create_raw_from_file::<RawChainConfig>("tests/files/chain/invalid.json").await;
    assert!(file_config.is_err());
}
