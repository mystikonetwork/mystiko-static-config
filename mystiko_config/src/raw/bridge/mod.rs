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

use mystiko_types::BridgeType;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use validator::{Validate, ValidationErrors};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum RawBridgeConfig {
    Axelar(Arc<RawAxelarBridgeConfig>),
    Celer(Arc<RawCelerBridgeConfig>),
    LayerZero(Arc<RawLayerZeroBridgeConfig>),
    Poly(Arc<RawPolyBridgeConfig>),
    Tbridge(Arc<RawTBridgeConfig>),
}

impl RawBridgeConfig {
    pub fn bridge_type(&self) -> &BridgeType {
        match self {
            RawBridgeConfig::Axelar(conf) => &conf.bridge_type,
            RawBridgeConfig::Celer(conf) => &conf.bridge_type,
            RawBridgeConfig::LayerZero(conf) => &conf.bridge_type,
            RawBridgeConfig::Poly(conf) => &conf.bridge_type,
            RawBridgeConfig::Tbridge(conf) => &conf.bridge_type,
        }
    }
}

impl Validate for RawBridgeConfig {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            RawBridgeConfig::Axelar(conf) => conf.validate(),
            RawBridgeConfig::Celer(conf) => conf.validate(),
            RawBridgeConfig::LayerZero(conf) => conf.validate(),
            RawBridgeConfig::Poly(conf) => conf.validate(),
            RawBridgeConfig::Tbridge(conf) => conf.validate(),
        }
    }
}

impl Hash for RawBridgeConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            RawBridgeConfig::Axelar(conf) => conf.hash(state),
            RawBridgeConfig::Celer(conf) => conf.hash(state),
            RawBridgeConfig::LayerZero(conf) => conf.hash(state),
            RawBridgeConfig::Poly(conf) => conf.hash(state),
            RawBridgeConfig::Tbridge(conf) => conf.hash(state),
        }
    }
}
