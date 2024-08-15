use crate::RawScreeningConfig;
use anyhow::Result;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct ScreeningConfig {
    raw: Arc<RawScreeningConfig>,
}

impl ScreeningConfig {
    pub fn new(raw: Arc<RawScreeningConfig>) -> Self {
        ScreeningConfig { raw }
    }

    pub fn url(&self) -> &str {
        &self.raw.url
    }

    pub fn version(&self) -> u32 {
        self.raw.version
    }

    pub fn client_timeout_ms(&self) -> u64 {
        self.raw.client_timeout_ms
    }

    pub fn validate(&self) -> Result<()> {
        Ok(self.raw.validate()?)
    }
}
