use mystiko_config::{
    create_raw_from_file, AssetConfig, CircuitConfig, ContractConfig, DepositContractConfig,
    PoolContractConfig, RawAssetConfig, RawCircuitConfig, RawDepositContractConfig,
    RawPoolContractConfig, MAIN_ASSET_ADDRESS,
};
use mystiko_types::{AssetType, BridgeType, ContractType};
use num_bigint::BigUint;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::fs::read_to_string;

const VALID_CONFIG_FILE: &str = "tests/files/contract/deposit/valid.json";

#[tokio::test]
async fn test_create() {
    let (_, _, _, _, config) = setup(SetupOptions::default()).await;
    config.validate().unwrap();
    assert_eq!(config.version(), 2);
    assert_eq!(config.name(), "MystikoWithPolyERC20");
    assert_eq!(
        config.address(),
        "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    );
    assert_eq!(config.contract_type(), &ContractType::Deposit);
    assert_eq!(config.start_block(), 1000000);
    assert!(config.event_filter_size().is_none());
    assert_eq!(config.bridge_type(), &BridgeType::Tbridge);
    assert_eq!(
        config.pool_contract_address(),
        "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
    );
    assert!(config.disabled());
    assert_eq!(config.disabled_at().unwrap(), 1001000);
    assert_eq!(
        config.min_amount().unwrap(),
        BigUint::from_str("10000000000000000").unwrap()
    );
    assert_eq!(config.min_amount_number::<f64>().unwrap(), 0.01);
    assert_eq!(
        config.max_amount().unwrap(),
        BigUint::from_str("100000000000000000").unwrap()
    );
    assert_eq!(config.max_amount_number::<f64>().unwrap(), 0.1);
    assert_eq!(
        config.min_bridge_fee().unwrap(),
        BigUint::from_str("20000000000000000").unwrap()
    );
    assert_eq!(config.min_bridge_fee_number::<f64>().unwrap(), 0.02);
    assert_eq!(
        config.min_executor_fee().unwrap(),
        BigUint::from_str("30000000000000000").unwrap()
    );
    assert_eq!(config.min_executor_fee_number::<f64>().unwrap(), 0.03);
    assert_eq!(
        config.min_rollup_fee().unwrap(),
        BigUint::from_str("120000000000000000").unwrap()
    );
    assert_eq!(config.min_rollup_fee_number::<f64>().unwrap(), 0.12);
    assert_eq!(config.service_fee(), 2);
    assert_eq!(config.service_fee_divider(), 1000);
    assert_eq!(
        config.asset().asset_address(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert_eq!(
        config.asset_address().unwrap(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert_eq!(config.asset_type(), &AssetType::Erc20);
    assert_eq!(config.asset_decimals(), 18);
    assert_eq!(config.asset_symbol(), "MTT");
    assert_eq!(
        config.recommended_amounts().unwrap(),
        vec![
            BigUint::from_str("1000000000000000000").unwrap(),
            BigUint::from_str("10000000000000000000").unwrap(),
        ]
    );
    assert_eq!(
        config.recommended_amounts_number::<u32>().unwrap(),
        vec![1, 10]
    );
    assert_eq!(config.peer_chain_id().unwrap(), 97);
    assert_eq!(
        config.peer_contract_address().unwrap(),
        "0x98bF2d9e3bA2A8515E660BD4104432ce3e2D7547"
    );
    assert_eq!(
        config.bridge_fee_asset_address().unwrap(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert_eq!(
        config.bridge_fee_asset().asset_address(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert_eq!(
        config.executor_fee_asset_address().unwrap(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert_eq!(
        config.executor_fee_asset().asset_address(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert!(!config.circuits().is_empty());
}

#[tokio::test]
async fn test_create_contract_config() {
    let (_, _, _, _, config) = setup(SetupOptions::default()).await;
    let config = ContractConfig::Deposit(Arc::new(config));
    config.validate().unwrap();
    assert_eq!(config.version(), 2);
    assert_eq!(config.name(), "MystikoWithPolyERC20");
    assert_eq!(
        config.address(),
        "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    );
    assert_eq!(config.contract_type(), &ContractType::Deposit);
    assert!(config.disabled());
    assert_eq!(config.disabled_at().unwrap(), 1001000);
    assert_eq!(config.start_block(), 1000000);
    assert!(config.event_filter_size().is_none());
    assert_eq!(config.bridge_type(), &BridgeType::Tbridge);
    assert_eq!(
        config.min_rollup_fee().unwrap(),
        BigUint::from_str("120000000000000000").unwrap()
    );
    assert_eq!(config.min_rollup_fee_number::<f64>().unwrap(), 0.12);
    assert_eq!(
        config.asset().asset_address(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert_eq!(
        config.asset_address().unwrap(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert_eq!(config.asset_type(), &AssetType::Erc20);
    assert_eq!(config.asset_decimals(), 18);
    assert_eq!(config.asset_symbol(), "MTT");
    assert_eq!(
        config.recommended_amounts().unwrap(),
        vec![
            BigUint::from_str("1000000000000000000").unwrap(),
            BigUint::from_str("10000000000000000000").unwrap(),
        ]
    );
    assert_eq!(
        config.recommended_amounts_number::<u32>().unwrap(),
        vec![1, 10]
    );
}

#[tokio::test]
async fn test_create_with_filter_size() {
    let (_, _, _, _, config) = setup(SetupOptions {
        event_filter_size: Some(100000),
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(config.event_filter_size().unwrap(), 100000);
}

#[tokio::test]
async fn test_validate_min_max_amount() {
    let (_, _, _, _, config) = setup(SetupOptions {
        incorrect_min_max: true,
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "deposit contract 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        min_amount is greater than max_amount"
    );
}

#[tokio::test]
async fn test_validate_bridge_asset_address_mismatch() {
    let (_, _, _, _, config) = setup(SetupOptions {
        bridge_fee_asset_address: Some(String::from("0x388c818ca8b9251b393131c08a736a67ccb19297")),
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "deposit contract 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        configured bridge_fee_asset_address 0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a \
        is different with the given bridge_fee_asset_config \
        asset_address 0x388c818ca8b9251b393131c08a736a67ccb19297"
    );
}

#[tokio::test]
async fn test_validate_executor_asset_address_mismatch() {
    let (_, _, _, _, config) = setup(SetupOptions {
        executor_fee_asset_address: Some(String::from(
            "0x388c818ca8b9251b393131c08a736a67ccb19297",
        )),
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "deposit contract 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        configured executor_fee_asset_address 0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a \
        is different with the given executor_fee_asset_config \
        asset_address 0x388c818ca8b9251b393131c08a736a67ccb19297"
    );
}

#[tokio::test]
async fn test_validate_loop_peer_chain_config() {
    let (_, _, _, _, config) = setup(SetupOptions {
        bridge_type: Some(BridgeType::Loop),
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "deposit contract 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        peer_contract_address and peer_contract_address \
        should be None when bridge_type is LOOP"
    );
}

#[tokio::test]
async fn test_validate_non_loop_no_peer_chain_config() {
    let (_, _, _, _, config1) = setup(SetupOptions {
        no_peer_chain_id: true,
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config1.validate().err().unwrap().to_string(),
        "deposit contract 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        peer_contract_address and peer_contract_address \
        should NOT be None when bridge_type is NOT LOOP"
    );
    let (_, _, _, _, config2) = setup(SetupOptions {
        no_peer_contract_address: true,
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config2.validate().err().unwrap().to_string(),
        "deposit contract 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        peer_contract_address and peer_contract_address \
        should NOT be None when bridge_type is NOT LOOP"
    );
}

#[tokio::test]
async fn test_validate_pool_contract_address_mismatch() {
    let (_, _, _, _, config) = setup(SetupOptions {
        pool_contract_address: Some(String::from("0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326")),
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "mismatched pool contract address 0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326 \
        vs 0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d for deposit contract config \
        at 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    );
}

#[tokio::test]
async fn test_validate_wrong_bridge_type() {
    let (_, _, _, _, config) = setup(SetupOptions {
        bridge_type: Some(BridgeType::Axelar),
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "mismatched pool contract bridge_type \
        Axelar vs Tbridge for deposit contract config \
        at 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    );
}

#[tokio::test]
async fn test_validate_too_small_disabled_at() {
    let (_, _, _, _, config) = setup(SetupOptions {
        disabled_at: Some(999999),
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "deposit contract 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        disabled_at 999999 is less than start_block 1000000"
    );
}

#[tokio::test]
async fn test_validate_pool_contract_disabled() {
    let (_, _, _, _, config) = setup(SetupOptions {
        pool_contract_disabled_at: Some(1000001),
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "deposit contract 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        pool contract 0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d is disabled"
    );
}

#[derive(Default)]
struct SetupOptions {
    bridge_type: Option<BridgeType>,
    pool_contract_address: Option<String>,
    no_peer_chain_id: bool,
    no_peer_contract_address: bool,
    event_filter_size: Option<u64>,
    incorrect_min_max: bool,
    bridge_fee_asset_address: Option<String>,
    executor_fee_asset_address: Option<String>,
    disabled_at: Option<u64>,
    pool_contract_disabled_at: Option<u64>,
}

async fn setup(
    options: SetupOptions,
) -> (
    Arc<AssetConfig>,
    Arc<AssetConfig>,
    Arc<PoolContractConfig>,
    Arc<RawDepositContractConfig>,
    DepositContractConfig,
) {
    let mut raw_pool_contract_config = create_raw_pool_contract_config().await;
    if let Some(pool_contract_disabled_at) = options.pool_contract_disabled_at {
        raw_pool_contract_config.disabled_at = Some(pool_contract_disabled_at);
    }
    let raw_pool_contract_config = Arc::new(raw_pool_contract_config);
    let main_asset_config = Arc::new(AssetConfig::new(Arc::new(
        create_raw_main_asset_config().await,
    )));
    let asset_address = if let Some(address) = &raw_pool_contract_config.asset_address {
        address
    } else {
        MAIN_ASSET_ADDRESS
    };
    let asset_config = Arc::new(AssetConfig::new(Arc::new(
        create_raw_asset_config(asset_address).await,
    )));
    let circuit_configs: Vec<Arc<CircuitConfig>> = create_raw_circuit_configs()
        .await
        .into_iter()
        .map(|r| Arc::new(CircuitConfig::new(Arc::new(r))))
        .collect();
    let pool_contract_config = Arc::new(PoolContractConfig::new(
        raw_pool_contract_config,
        main_asset_config.clone(),
        asset_config.clone(),
        circuit_configs,
    ));
    let mut raw_config = create_raw_deposit_contract_config().await;
    let bridge_fee_asset_address =
        if let Some(bridge_fee_asset_address) = &options.bridge_fee_asset_address {
            Some(bridge_fee_asset_address.clone())
        } else {
            raw_config.bridge_fee_asset_address.clone()
        };
    let executor_fee_asset_address =
        if let Some(executor_fee_asset_address) = &options.executor_fee_asset_address {
            Some(executor_fee_asset_address.clone())
        } else {
            raw_config.executor_fee_asset_address.clone()
        };
    let bridge_fee_asset = if let Some(bridge_fee_asset_address) = &bridge_fee_asset_address {
        Arc::new(AssetConfig::new(Arc::new(
            create_raw_asset_config(bridge_fee_asset_address).await,
        )))
    } else {
        main_asset_config.clone()
    };
    let executor_fee_asset = if let Some(executor_fee_asset_address) = &executor_fee_asset_address {
        Arc::new(AssetConfig::new(Arc::new(
            create_raw_asset_config(executor_fee_asset_address).await,
        )))
    } else {
        asset_config.clone()
    };
    if let Some(pool_contract_address) = &options.pool_contract_address {
        raw_config.pool_address = pool_contract_address.clone();
    }
    if let Some(bridge_type) = &options.bridge_type {
        raw_config.bridge_type = bridge_type.clone();
    }
    if options.no_peer_chain_id {
        raw_config.peer_chain_id = None;
    }
    if options.no_peer_contract_address {
        raw_config.peer_contract_address = None;
    }
    if let Some(event_filter_size) = options.event_filter_size {
        raw_config.event_filter_size = Some(event_filter_size);
    }
    if options.incorrect_min_max {
        raw_config.min_amount = String::from("1000000000000000000");
        raw_config.max_amount = String::from("100000000000000000");
    }
    if options.pool_contract_disabled_at.is_some() {
        raw_config.disabled_at = None;
    } else if let Some(disabled_at) = options.disabled_at {
        raw_config.disabled_at = Some(disabled_at);
    }
    let raw_config_arc = Arc::new(raw_config);
    let config = DepositContractConfig::new(
        raw_config_arc.clone(),
        bridge_fee_asset.clone(),
        executor_fee_asset.clone(),
        pool_contract_config.clone(),
    );
    (
        bridge_fee_asset,
        executor_fee_asset,
        pool_contract_config,
        raw_config_arc,
        config,
    )
}

async fn create_raw_deposit_contract_config() -> RawDepositContractConfig {
    create_raw_from_file::<RawDepositContractConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap()
}

async fn create_raw_pool_contract_config() -> RawPoolContractConfig {
    create_raw_from_file::<RawPoolContractConfig>("tests/files/contract/deposit/pool.json")
        .await
        .unwrap()
}

async fn create_raw_main_asset_config() -> RawAssetConfig {
    create_raw_from_file::<RawAssetConfig>("tests/files/contract/deposit/main_asset.json")
        .await
        .unwrap()
}

async fn create_raw_asset_config(address: &str) -> RawAssetConfig {
    let contents = read_to_string(PathBuf::from("tests/files/contract/deposit/asset.json"))
        .await
        .unwrap();
    serde_json::from_str::<RawAssetConfig>(&contents.replace("__ADDRESS__", address)).unwrap()
}

async fn create_raw_circuit_configs() -> Vec<RawCircuitConfig> {
    let contents = read_to_string(PathBuf::from("tests/files/contract/deposit/circuits.json"))
        .await
        .unwrap();
    serde_json::from_str::<Vec<RawCircuitConfig>>(&contents).unwrap()
}
