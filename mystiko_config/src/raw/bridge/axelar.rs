use mystiko_types::BridgeType;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use typed_builder::TypedBuilder;
use validator::{Validate, ValidationError};

#[derive(
    TypedBuilder, Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct RawAxelarBridgeConfig {
    #[validate(length(min = 1))]
    pub name: String,

    #[serde(rename = "type")]
    #[serde(default = "default_bridge_type")]
    #[builder(default = default_bridge_type())]
    #[validate(custom = "validate_bridge_type")]
    pub bridge_type: BridgeType,
}

fn default_bridge_type() -> BridgeType {
    BridgeType::Axelar
}

fn validate_bridge_type(t: &BridgeType) -> Result<(), ValidationError> {
    if *t == BridgeType::Axelar {
        return Ok(());
    }
    Err(ValidationError::new("bridge type error"))
}
