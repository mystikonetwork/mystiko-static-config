use lazy_static::lazy_static;
use mystiko_config::{
    create_raw, create_raw_from_file, create_raw_from_json, RawCelerBridgeConfig,
};
use mystiko_types::BridgeType;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use validator::Validate;

fn default_config() -> RawCelerBridgeConfig {
    create_raw::<RawCelerBridgeConfig>(
        RawCelerBridgeConfig::builder()
            .name("Celer Bridge".to_string())
            .build(),
    )
    .unwrap()
}

lazy_static! {
    static ref RAW_CONFIG: RawCelerBridgeConfig = default_config();
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
    assert_eq!("Celer Bridge", &config.name);
}

#[test]
fn test_invalid_type() {
    let mut config = default_config();
    config.bridge_type = BridgeType::Tbridge;
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config =
        create_raw_from_file::<RawCelerBridgeConfig>("tests/files/bridge/celer/valid.json")
            .await
            .unwrap();
    assert_eq!(file_config, default_config());
    assert_eq!(BridgeType::Celer, file_config.bridge_type);
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        create_raw_from_file::<RawCelerBridgeConfig>("tests/files/bridge/celer/invalid.json").await;
    assert!(file_config.is_err());
}

#[test]
fn test_import_valid_json_str() {
    let json_str = r#"{
            "name": "Celer Bridge"
        }"#;
    let str_config = create_raw_from_json::<RawCelerBridgeConfig>(json_str).unwrap();
    assert_eq!(str_config.bridge_type, BridgeType::Celer);
}
