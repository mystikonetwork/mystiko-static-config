use mystiko_config::{create_raw_from_file, LayerZeroBridgeConfig, RawLayerZeroBridgeConfig};
use mystiko_types::BridgeType;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/bridge/layer_zero/valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawLayerZeroBridgeConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = LayerZeroBridgeConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.name(), "LayerZero Bridge");
    assert_eq!(config.bridge_type(), &BridgeType::LayerZero);
}
