use mystiko_config::{create_raw_from_file, CelerBridgeConfig, RawCelerBridgeConfig};
use mystiko_types::BridgeType;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/bridge/celer/valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawCelerBridgeConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = CelerBridgeConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.name(), "Celer Bridge");
    assert_eq!(config.bridge_type(), &BridgeType::Celer);
}
