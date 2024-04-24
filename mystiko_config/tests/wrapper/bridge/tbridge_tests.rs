use mystiko_config::{create_raw_from_file, RawTBridgeConfig, TBridgeConfig};
use mystiko_types::BridgeType;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/bridge/tbridge/valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawTBridgeConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = TBridgeConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.name(), "Mystiko Testnet Bridge");
    assert_eq!(config.bridge_type(), &BridgeType::Tbridge);
}
