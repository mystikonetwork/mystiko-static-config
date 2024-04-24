use lazy_static::lazy_static;
use mystiko_config::{
    create_raw, create_raw_from_file, create_raw_from_json, RawPoolContractConfig,
};
use mystiko_types::{BridgeType, ContractType};
use validator::Validate;

fn default_config() -> RawPoolContractConfig {
    create_raw::<RawPoolContractConfig>(
        RawPoolContractConfig::builder()
            .version(2)
            .name("CommitmentPool".to_string())
            .address("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string())
            .contract_type(ContractType::Pool)
            .start_block(1000000)
            .disabled_at(Some(1001000))
            .pool_name("A Pool(since 07/20/2022)".to_string())
            .bridge_type(BridgeType::Tbridge)
            .asset_address(Some(
                "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string(),
            ))
            .min_rollup_fee("120000000000000000".to_string())
            .circuits(vec![String::from("circuit-1.0")])
            .build(),
    )
    .unwrap()
}

lazy_static! {
    static ref RAW_CONFIG: RawPoolContractConfig = default_config();
}

#[test]
fn test_default_values() {
    let raw_config = RawPoolContractConfig::builder()
        .version(2)
        .name("CommitmentPool".to_string())
        .address("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string())
        .start_block(1000000)
        .pool_name("A Pool(since 07/20/2022)".to_string())
        .bridge_type(BridgeType::Tbridge)
        .asset_address(Some(
            "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string(),
        ))
        .circuits(vec![String::from("circuit-1.0")])
        .build();
    assert!(raw_config.disabled_at.is_none());
    assert_eq!(raw_config.min_rollup_fee, "0");
    assert_eq!(raw_config.contract_type, ContractType::Pool);
}

#[test]
fn test_raw_contract_config_trait() {
    let config = &RAW_CONFIG;
    assert_eq!(2, config.version);
    assert_eq!("CommitmentPool", config.name);
    assert_eq!("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67", config.address);
    assert_eq!(ContractType::Pool, config.contract_type);
    assert_eq!(1000000, config.start_block);
    assert_eq!(None, config.event_filter_size);
}

#[test]
fn test_validate_success() {
    let mut config = default_config();
    config.asset_address = None;
    config.min_rollup_fee = "0".to_string();
    config.circuits = vec![];
    assert!(config.validate().is_ok());
}

#[test]
fn test_invalid_disabled_at() {
    let mut config = default_config();
    config.disabled_at = Some(0);
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_pool_name() {
    let mut config = default_config();
    config.pool_name = "".to_string();
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_contract_type() {
    let mut config = default_config();
    config.contract_type = ContractType::Deposit;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_min_rollup_fee_0() {
    let mut config = default_config();
    config.min_rollup_fee = String::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_min_rollup_fee_1() {
    let mut config = default_config();
    config.min_rollup_fee = String::from("0xdeadbeef");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_min_rollup_fee_2() {
    let mut config = default_config();
    config.min_rollup_fee = String::from("-1");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_min_rollup_fee_3() {
    let mut config = default_config();
    config.min_rollup_fee = String::from("1.2");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_circuits() {
    let mut config = default_config();
    config.circuits = vec![String::from("")];
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config =
        create_raw_from_file::<RawPoolContractConfig>("tests/files/contract/pool/valid.json")
            .await
            .unwrap();
    assert_eq!(file_config, default_config());
    assert_eq!(file_config.contract_type, ContractType::Pool);
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        create_raw_from_file::<RawPoolContractConfig>("tests/files/contract/pool/invalid.json")
            .await;
    assert!(file_config.is_err());
}

#[test]
fn test_import_valid_json_str() {
    let json_str = r#"
            {
              "version": 2,
              "name": "CommitmentPool",
              "poolName": "A Pool(since 07/20/2022)",
              "bridgeType": "tbridge",
              "address": "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67",
              "startBlock": 1000000,
              "disabledAt": 1001000,
              "assetAddress": "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a",
              "minRollupFee": "120000000000000000",
              "circuits": ["circuit-1.0"]
            }
        "#;
    let str_config = create_raw_from_json::<RawPoolContractConfig>(json_str).unwrap();
    assert_eq!(str_config.contract_type, ContractType::Pool);
}
