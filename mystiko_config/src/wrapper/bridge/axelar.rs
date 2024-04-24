use crate::RawAxelarBridgeConfig;
use anyhow::Result;
use mystiko_types::BridgeType;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct AxelarBridgeConfig {
    raw: Arc<RawAxelarBridgeConfig>,
}

impl AxelarBridgeConfig {
    pub fn new(raw: Arc<RawAxelarBridgeConfig>) -> Self {
        AxelarBridgeConfig { raw }
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
