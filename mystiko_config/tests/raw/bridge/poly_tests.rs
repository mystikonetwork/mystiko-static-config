use lazy_static::lazy_static;
use mystiko_config::{
    create_raw, create_raw_from_file, create_raw_from_json, RawPolyBridgeConfig,
    EXPLORER_DEFAULT_PREFIX,
};
use mystiko_types::BridgeType;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use validator::Validate;

fn default_config() -> RawPolyBridgeConfig {
    create_raw::<RawPolyBridgeConfig>(
        RawPolyBridgeConfig::builder()
            .name("Poly Bridge".to_string())
            .explorer_url("https://explorer.poly.network".to_string())
            .explorer_prefix("/tx/%tx%".to_string())
            .api_url("https://explorer.poly.network".to_string())
            .api_prefix("/testnet/api/v1/getcrosstx?txhash=%tx%".to_string())
            .build(),
    )
    .unwrap()
}

lazy_static! {
    static ref RAW_CONFIG: RawPolyBridgeConfig = default_config();
}

#[test]
fn test_default_values() {
    let raw_config = RawPolyBridgeConfig::builder()
        .name("Poly Bridge".to_string())
        .explorer_url("https://explorer.poly.network".to_string())
        .api_url("https://explorer.poly.network".to_string())
        .api_prefix("/testnet/api/v1/getcrosstx?txhash=%tx%".to_string())
        .build();
    assert_eq!(raw_config.bridge_type, BridgeType::Poly);
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
    config2.bridge_type = BridgeType::Tbridge;
    config2.hash(&mut hasher);
    let hash2 = hasher.finish();

    assert_ne!(hash1, hash2);
}

#[test]
fn test_name() {
    let config = &RAW_CONFIG;
    assert_eq!("Poly Bridge", &config.name);
}

#[test]
fn test_validate_success() {
    let config = &RAW_CONFIG;
    assert_eq!(config.bridge_type, BridgeType::Poly);
    assert_eq!(config.explorer_prefix, EXPLORER_DEFAULT_PREFIX);
}

#[test]
fn test_invalid_type() {
    let mut config = default_config();
    config.bridge_type = BridgeType::Tbridge;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_explorer_url_0() {
    let mut config = default_config();
    config.explorer_url = String::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_explorer_url_1() {
    let mut config = default_config();
    config.explorer_url = String::from("wrong url");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_explorer_prefix_0() {
    let mut config = default_config();
    config.explorer_prefix = String::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_explorer_prefix_1() {
    let mut config = default_config();
    config.explorer_prefix = String::from("wrong prefix");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_api_url_0() {
    let mut config = default_config();
    config.api_url = String::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_api_url_1() {
    let mut config = default_config();
    config.api_url = String::from("wrong url");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_api_prefix_0() {
    let mut config = default_config();
    config.api_prefix = String::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_api_prefix_1() {
    let mut config = default_config();
    config.api_prefix = String::from("wrong prefix");
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config =
        create_raw_from_file::<RawPolyBridgeConfig>("tests/files/bridge/poly/valid.json")
            .await
            .unwrap();
    assert_eq!(file_config, default_config())
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        create_raw_from_file::<RawPolyBridgeConfig>("tests/files/bridge/poly/invalid.json").await;
    assert!(file_config.is_err());
}

#[tokio::test]
async fn test_import_valid_json_str() {
    let json_str = r#"{
            "name": "Poly Bridge",
            "explorerUrl": "https://explorer.poly.network",
            "explorerPrefix": "/tx/%tx%",
            "apiUrl": "https://explorer.poly.network",
            "apiPrefix": "/testnet/api/v1/getcrosstx?txhash=%tx%"
        }"#;
    let str_config = create_raw_from_json::<RawPolyBridgeConfig>(json_str).unwrap();
    assert_eq!(str_config.bridge_type, BridgeType::Poly);
}
