use mystiko_config::{create_raw_from_file, RawWormholeBridgeConfig, WormholeBridgeConfig};
use mystiko_types::BridgeType;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/bridge/wormhole/valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawWormholeBridgeConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = WormholeBridgeConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.name(), "Wormhole Bridge");
    assert_eq!(config.bridge_type(), &BridgeType::Wormhole);
}
