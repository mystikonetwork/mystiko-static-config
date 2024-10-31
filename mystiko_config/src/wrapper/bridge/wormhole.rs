use crate::RawWormholeBridgeConfig;
use anyhow::Result;
use mystiko_types::BridgeType;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct WormholeBridgeConfig {
    raw: Arc<RawWormholeBridgeConfig>,
}

impl WormholeBridgeConfig {
    pub fn new(raw: Arc<RawWormholeBridgeConfig>) -> Self {
        WormholeBridgeConfig { raw }
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
