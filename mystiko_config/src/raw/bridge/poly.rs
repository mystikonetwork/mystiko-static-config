use crate::EXPLORER_DEFAULT_PREFIX;
use mystiko_types::BridgeType;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use typed_builder::TypedBuilder;
use validator::{Validate, ValidationError};

#[derive(
    TypedBuilder, Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct RawPolyBridgeConfig {
    #[validate(length(min = 1))]
    pub name: String,

    #[serde(rename = "type")]
    #[serde(default = "default_bridge_type")]
    #[builder(default = default_bridge_type())]
    #[validate(custom = "validate_bridge_type")]
    pub bridge_type: BridgeType,

    #[validate(url)]
    pub explorer_url: String,

    #[validate(contains = "%tx%")]
    #[serde(default = "default_explorer_prefix")]
    #[builder(default = default_explorer_prefix())]
    pub explorer_prefix: String,

    #[validate(url)]
    pub api_url: String,

    #[validate(contains = "%tx%")]
    pub api_prefix: String,
}

fn default_bridge_type() -> BridgeType {
    BridgeType::Poly
}

fn default_explorer_prefix() -> String {
    EXPLORER_DEFAULT_PREFIX.to_string()
}

fn validate_bridge_type(t: &BridgeType) -> Result<(), ValidationError> {
    if *t == BridgeType::Poly {
        return Ok(());
    }
    Err(ValidationError::new("bridge type error"))
}
