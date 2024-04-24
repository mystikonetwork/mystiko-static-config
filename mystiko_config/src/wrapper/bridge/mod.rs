mod axelar;
mod celer;
mod layer_zero;
mod poly;
mod tbridge;

pub use axelar::*;
pub use celer::*;
pub use layer_zero::*;
pub use poly::*;
pub use tbridge::*;

use crate::RawBridgeConfig;
use anyhow::Result;
use mystiko_types::BridgeType;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum BridgeConfig {
    Axelar(AxelarBridgeConfig),
    Celer(CelerBridgeConfig),
    LayerZero(LayerZeroBridgeConfig),
    Poly(PolyBridgeConfig),
    TBridge(TBridgeConfig),
}

impl BridgeConfig {
    pub fn new(raw: Arc<RawBridgeConfig>) -> Self {
        match raw.as_ref() {
            RawBridgeConfig::Axelar(axelar_raw) => {
                BridgeConfig::Axelar(AxelarBridgeConfig::new(axelar_raw.clone()))
            }
            RawBridgeConfig::Celer(celer_raw) => {
                BridgeConfig::Celer(CelerBridgeConfig::new(celer_raw.clone()))
            }
            RawBridgeConfig::LayerZero(layer_zero_raw) => {
                BridgeConfig::LayerZero(LayerZeroBridgeConfig::new(layer_zero_raw.clone()))
            }
            RawBridgeConfig::Poly(poly_raw) => {
                BridgeConfig::Poly(PolyBridgeConfig::new(poly_raw.clone()))
            }
            RawBridgeConfig::Tbridge(tbridge_raw) => {
                BridgeConfig::TBridge(TBridgeConfig::new(tbridge_raw.clone()))
            }
        }
    }

    pub fn name(&self) -> &str {
        match self {
            BridgeConfig::Axelar(conf) => conf.name(),
            BridgeConfig::Celer(conf) => conf.name(),
            BridgeConfig::LayerZero(conf) => conf.name(),
            BridgeConfig::Poly(conf) => conf.name(),
            BridgeConfig::TBridge(conf) => conf.name(),
        }
    }

    pub fn bridge_type(&self) -> &BridgeType {
        match self {
            BridgeConfig::Axelar(conf) => conf.bridge_type(),
            BridgeConfig::Celer(conf) => conf.bridge_type(),
            BridgeConfig::LayerZero(conf) => conf.bridge_type(),
            BridgeConfig::Poly(conf) => conf.bridge_type(),
            BridgeConfig::TBridge(conf) => conf.bridge_type(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        match self {
            BridgeConfig::Axelar(conf) => conf.validate(),
            BridgeConfig::Celer(conf) => conf.validate(),
            BridgeConfig::LayerZero(conf) => conf.validate(),
            BridgeConfig::Poly(conf) => conf.validate(),
            BridgeConfig::TBridge(conf) => conf.validate(),
        }
    }
}
