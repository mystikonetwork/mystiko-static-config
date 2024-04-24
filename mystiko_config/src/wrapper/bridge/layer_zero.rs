use crate::RawLayerZeroBridgeConfig;
use anyhow::Result;
use mystiko_types::BridgeType;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct LayerZeroBridgeConfig {
    raw: Arc<RawLayerZeroBridgeConfig>,
}

impl LayerZeroBridgeConfig {
    pub fn new(raw: Arc<RawLayerZeroBridgeConfig>) -> Self {
        LayerZeroBridgeConfig { raw }
    }

    pub fn name(&self) -> &str {
        &self.raw.name
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.raw.bridge_type
    }

    pub fn validate(&self) -> Result<()> {
        Ok(self.raw.validate()?)
    }
}
