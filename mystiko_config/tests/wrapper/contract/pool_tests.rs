use mystiko_config::{
    create_raw_from_file, AssetConfig, CircuitConfig, ContractConfig, PoolContractConfig,
    RawAssetConfig, RawCircuitConfig, RawPoolContractConfig, MAIN_ASSET_ADDRESS,
};
use mystiko_types::{AssetType, BridgeType, CircuitType, ContractType};
use num_bigint::BigUint;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::fs::read_to_string;

const VALID_CONFIG_FILE: &str = "tests/files/contract/pool/valid.json";

#[tokio::test]
async fn test_create() {
    let (main_asset_config, asset_config, circuit_configs, raw_config, config) =
        setup(SetupOptions::default()).await;
    config.validate().unwrap();
    assert_eq!(config.version(), 2);
    assert_eq!(config.name(), "CommitmentPool");
    assert_eq!(config.pool_name(), "A Pool(since 07/20/2022)");
    assert_eq!(config.bridge_type(), &BridgeType::Tbridge);
    assert_eq!(
        config.address(),
        "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    );
    assert_eq!(config.contract_type(), &ContractType::Pool);
    assert_eq!(config.start_block(), 1000000);
    assert!(config.disabled());
    assert_eq!(config.disabled_at().unwrap(), 1001000);
    assert!(config.event_filter_size().is_none());
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
    assert_eq!(
        config.min_rollup_fee().unwrap(),
        BigUint::from_str("120000000000000000").unwrap()
    );
    assert_eq!(config.min_rollup_fee_number::<f64>().unwrap(), 0.12);
    assert_eq!(config.circuits_names(), &vec![String::from("circuit-1.0")]);
    assert_eq!(config.circuits().len(), circuit_configs.len());
    assert_eq!(
        config
            .circuit_by_type(&CircuitType::Rollup1)
            .unwrap()
            .name(),
        "circuit-1.0"
    );
    assert_eq!(
        config
            .circuit_by_name("circuit-1.0")
            .unwrap()
            .circuit_type(),
        &CircuitType::Rollup1
    );
    let mut raw_config1 = raw_config.as_ref().clone();
    raw_config1.event_filter_size = Some(1000);
    let config1 = PoolContractConfig::new(
        Arc::new(raw_config1),
        main_asset_config,
        asset_config,
        circuit_configs,
    );
    assert_eq!(config1.event_filter_size().unwrap(), 1000);
}

#[tokio::test]
async fn test_create_contract_config() {
    let (_, _, _, _, config) = setup(SetupOptions::default()).await;
    let config = ContractConfig::Pool(Arc::new(config));
    config.validate().unwrap();
    assert_eq!(config.version(), 2);
    assert_eq!(config.name(), "CommitmentPool");
    assert_eq!(config.bridge_type(), &BridgeType::Tbridge);
    assert_eq!(
        config.address(),
        "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    );
    assert_eq!(config.contract_type(), &ContractType::Pool);
    assert!(config.disabled());
    assert_eq!(config.start_block(), 1000000);
    assert!(config.event_filter_size().is_none());
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
    assert_eq!(
        config.min_rollup_fee().unwrap(),
        BigUint::from_str("120000000000000000").unwrap()
    );
    assert_eq!(config.min_rollup_fee_number::<f64>().unwrap(), 0.12);
}

#[tokio::test]
async fn test_validate_asset_address_mismatch() {
    let (_, _, _, _, config) = setup(SetupOptions {
        asset_address: Some(MAIN_ASSET_ADDRESS.to_string()),
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "the given asset_config's address 0x0000000000000000000000000000000000000000 \
        is different with pool config 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        asset_address 0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
}

#[tokio::test]
async fn test_validate_missing_circuit_name() {
    let (_, _, _, _, config) = setup(SetupOptions {
        missing_circuit_name: true,
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "circuit config is missing for circuit_name circuit-1.0"
    );
}

#[tokio::test]
async fn test_validate_missing_circuit_type() {
    let (_, _, _, _, config) = setup(SetupOptions {
        missing_circuit_type: true,
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "circuit config is missing for circuit_type Rollup2"
    );
}

#[tokio::test]
async fn test_validate_duplicate_circuit_types() {
    let (_, _, _, _, config) = setup(SetupOptions {
        duplicate_circuit: true,
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "multiple circuit configs of circuit_type Rollup2"
    );
}

#[tokio::test]
async fn test_validate_asset_type_mismatch() {
    let (_, asset_config, circuit_configs, raw_config, config) = setup(SetupOptions {
        mismatch_asset_type: true,
        ..SetupOptions::default()
    })
    .await;
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "pool contract 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        asset_address should be None when asset_type is MAIN"
    );
    let mut raw_config1 = raw_config.as_ref().clone();
    raw_config1.asset_address = None;
    let mut raw_main_asset_config1 = create_raw_main_asset_config().await;
    raw_main_asset_config1.asset_type = AssetType::Erc20;
    let main_asset_config1 = Arc::new(AssetConfig::new(Arc::new(raw_main_asset_config1)));
    let config1 = PoolContractConfig::new(
        Arc::new(raw_config1),
        main_asset_config1,
        asset_config.clone(),
        circuit_configs.clone(),
    );
    assert_eq!(
        config1.validate().err().unwrap().to_string(),
        "pool contract 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        asset_address should NOT be None when asset_type is not MAIN"
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
        "pool contract 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        disabled_at 999999 is less than start_block 1000000"
    );
}

#[derive(Default)]
struct SetupOptions {
    asset_address: Option<String>,
    mismatch_asset_type: bool,
    duplicate_circuit: bool,
    missing_circuit_type: bool,
    missing_circuit_name: bool,
    disabled_at: Option<u64>,
}

async fn setup(
    options: SetupOptions,
) -> (
    Arc<AssetConfig>,
    Arc<AssetConfig>,
    Vec<Arc<CircuitConfig>>,
    Arc<RawPoolContractConfig>,
    PoolContractConfig,
) {
    let mut raw_config = create_raw_from_file::<RawPoolContractConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    if let Some(disabled_at) = options.disabled_at {
        raw_config.disabled_at = Some(disabled_at);
    }
    let raw_config = Arc::new(raw_config);
    let main_asset_config = Arc::new(AssetConfig::new(Arc::new(
        create_raw_main_asset_config().await,
    )));
    let mut raw_asset_config = create_raw_asset_config(
        &options
            .asset_address
            .unwrap_or(raw_config.asset_address.as_ref().unwrap().to_string()),
    )
    .await;
    if options.mismatch_asset_type {
        raw_asset_config.asset_type = AssetType::Main;
    }
    let asset_config = Arc::new(AssetConfig::new(Arc::new(raw_asset_config)));
    let mut raw_circuit_configs = create_raw_circuit_configs().await;
    if options.missing_circuit_name {
        let circuit_config = raw_circuit_configs.get_mut(0).unwrap();
        circuit_config.name = String::from("no_existing_name");
    }
    if options.missing_circuit_type {
        raw_circuit_configs.remove(1);
    }
    if options.duplicate_circuit {
        let mut circuit_config = raw_circuit_configs.get(1).unwrap().clone();
        circuit_config.name = String::from("duplicate_circuit_type");
        raw_circuit_configs.push(circuit_config);
    }
    let circuit_configs: Vec<Arc<CircuitConfig>> = raw_circuit_configs
        .into_iter()
        .map(|r| Arc::new(CircuitConfig::new(Arc::new(r))))
        .collect();
    let config = PoolContractConfig::new(
        raw_config.clone(),
        main_asset_config.clone(),
        asset_config.clone(),
        circuit_configs.clone(),
    );
    (
        main_asset_config,
        asset_config,
        circuit_configs,
        raw_config,
        config,
    )
}

async fn create_raw_main_asset_config() -> RawAssetConfig {
    create_raw_from_file::<RawAssetConfig>("tests/files/contract/pool/main_asset.json")
        .await
        .unwrap()
}

async fn create_raw_asset_config(address: &str) -> RawAssetConfig {
    let contents = read_to_string(PathBuf::from("tests/files/contract/pool/asset.json"))
        .await
        .unwrap();
    serde_json::from_str::<RawAssetConfig>(&contents.replace("__ADDRESS__", address)).unwrap()
}

async fn create_raw_circuit_configs() -> Vec<RawCircuitConfig> {
    let contents = read_to_string(PathBuf::from("tests/files/contract/pool/circuits.json"))
        .await
        .unwrap();
    serde_json::from_str::<Vec<RawCircuitConfig>>(&contents).unwrap()
}
