use crate::RawPackerConfig;
use anyhow::Result;
use mystiko_types::{PackerChecksum, PackerCompression};
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct PackerConfig {
    raw: Arc<RawPackerConfig>,
}

impl PackerConfig {
    pub fn new(raw: Arc<RawPackerConfig>) -> Self {
        PackerConfig { raw }
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

    pub fn checksum(&self) -> &PackerChecksum {
        &self.raw.checksum
    }

    pub fn compression(&self) -> &PackerCompression {
        &self.raw.compression
    }

    pub fn validate(&self) -> Result<()> {
        Ok(self.raw.validate()?)
    }
}
