use mystiko_types::{PackerChecksum, PackerCompression};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct RawPackerConfig {
    #[validate(url)]
    pub url: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_client_timeout_ms")]
    #[builder(default = default_client_timeout_ms())]
    pub client_timeout_ms: u64,

    #[validate(range(min = 1))]
    #[serde(default = "default_version")]
    #[builder(default = default_version())]
    pub version: u32,

    #[serde(default)]
    #[builder(default)]
    pub checksum: PackerChecksum,

    #[serde(default)]
    #[builder(default)]
    pub compression: PackerCompression,
}

impl Hash for RawPackerConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state)
    }
}

fn default_version() -> u32 {
    1
}

fn default_client_timeout_ms() -> u64 {
    15000
}
