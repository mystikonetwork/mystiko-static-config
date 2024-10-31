use mystiko_config::{
    create_raw_from_file, BridgeConfig, RawAxelarBridgeConfig, RawBridgeConfig,
    RawCelerBridgeConfig, RawLayerZeroBridgeConfig, RawPolyBridgeConfig, RawTBridgeConfig,
    RawWormholeBridgeConfig,
};
use mystiko_types::BridgeType;
use std::sync::Arc;

const AXELAR_CONFIG_FILE: &str = "tests/files/bridge/axelar/valid.json";
const CELER_CONFIG_FILE: &str = "tests/files/bridge/celer/valid.json";
const LAYER_ZERO_CONFIG_FILE: &str = "tests/files/bridge/layer_zero/valid.json";
const POLY_CONFIG_FILE: &str = "tests/files/bridge/poly/valid.json";
const TBRIDGE_CONFIG_FILE: &str = "tests/files/bridge/tbridge/valid.json";
const WORMHOLE_CONFIG_FILE: &str = "tests/files/bridge/wormhole/valid.json";

#[tokio::test]
async fn test_create() {
    let raw_axelar_config = create_raw_from_file::<RawAxelarBridgeConfig>(AXELAR_CONFIG_FILE)
        .await
        .unwrap();
    let raw_celer_config = create_raw_from_file::<RawCelerBridgeConfig>(CELER_CONFIG_FILE)
        .await
        .unwrap();
    let raw_layer_zero_config =
        create_raw_from_file::<RawLayerZeroBridgeConfig>(LAYER_ZERO_CONFIG_FILE)
            .await
            .unwrap();
    let raw_poly_config = create_raw_from_file::<RawPolyBridgeConfig>(POLY_CONFIG_FILE)
        .await
        .unwrap();
    let raw_tbridge_config = create_raw_from_file::<RawTBridgeConfig>(TBRIDGE_CONFIG_FILE)
        .await
        .unwrap();
    let raw_wormhole_config = create_raw_from_file::<RawWormholeBridgeConfig>(WORMHOLE_CONFIG_FILE)
        .await
        .unwrap();
    let bridge_config1 = BridgeConfig::new(Arc::new(RawBridgeConfig::Axelar(Arc::new(
        raw_axelar_config,
    ))));
    let bridge_config2 =
        BridgeConfig::new(Arc::new(RawBridgeConfig::Celer(Arc::new(raw_celer_config))));
    let bridge_config3 = BridgeConfig::new(Arc::new(RawBridgeConfig::LayerZero(Arc::new(
        raw_layer_zero_config,
    ))));
    let bridge_config4 =
        BridgeConfig::new(Arc::new(RawBridgeConfig::Poly(Arc::new(raw_poly_config))));
    let bridge_config5 = BridgeConfig::new(Arc::new(RawBridgeConfig::Tbridge(Arc::new(
        raw_tbridge_config,
    ))));
    let bridge_config6 = BridgeConfig::new(Arc::new(RawBridgeConfig::Wormhole(Arc::new(
        raw_wormhole_config,
    ))));
    bridge_config1.validate().unwrap();
    bridge_config2.validate().unwrap();
    bridge_config3.validate().unwrap();
    bridge_config4.validate().unwrap();
    bridge_config5.validate().unwrap();
    bridge_config6.validate().unwrap();
    assert_eq!(bridge_config1.name(), "Axelar Bridge");
    assert_eq!(bridge_config2.name(), "Celer Bridge");
    assert_eq!(bridge_config3.name(), "LayerZero Bridge");
    assert_eq!(bridge_config4.name(), "Poly Bridge");
    assert_eq!(bridge_config5.name(), "Mystiko Testnet Bridge");
    assert_eq!(bridge_config6.name(), "Wormhole Bridge");
    assert_eq!(bridge_config1.bridge_type(), &BridgeType::Axelar);
    assert_eq!(bridge_config2.bridge_type(), &BridgeType::Celer);
    assert_eq!(bridge_config3.bridge_type(), &BridgeType::LayerZero);
    assert_eq!(bridge_config4.bridge_type(), &BridgeType::Poly);
    assert_eq!(bridge_config5.bridge_type(), &BridgeType::Tbridge);
    assert_eq!(bridge_config6.bridge_type(), &BridgeType::Wormhole);
}
