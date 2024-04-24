use crate::RawPolyBridgeConfig;
use anyhow::Result;
use mystiko_types::BridgeType;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct PolyBridgeConfig {
    raw: Arc<RawPolyBridgeConfig>,
}

impl PolyBridgeConfig {
    pub fn new(raw: Arc<RawPolyBridgeConfig>) -> Self {
        PolyBridgeConfig { raw }
    }

    pub fn name(&self) -> &str {
        &self.raw.name
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.raw.bridge_type
    }

    pub fn explorer_url(&self) -> &str {
        &self.raw.explorer_url
    }

    pub fn explorer_prefix(&self) -> &str {
        &self.raw.explorer_prefix
    }

    pub fn api_url(&self) -> &str {
        &self.raw.api_url
    }

    pub fn api_prefix(&self) -> &str {
        &self.raw.api_prefix
    }

    pub fn validate(&self) -> Result<()> {
        Ok(self.raw.validate()?)
    }
}
