use std::sync::Arc;

use validator::Validate;

use crate::RawSequencerConfig;

#[derive(Clone, Debug)]
pub struct SequencerConfig {
    raw: Arc<RawSequencerConfig>,
}

impl SequencerConfig {
    pub fn new(raw: Arc<RawSequencerConfig>) -> Self {
        SequencerConfig { raw }
    }

    pub fn host(&self) -> &str {
        &self.raw.host
    }

    pub fn port(&self) -> Option<u16> {
        self.raw.port
    }

    pub fn is_ssl(&self) -> bool {
        self.raw.is_ssl
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        Ok(self.raw.validate()?)
    }
}
