use mystiko_types::{BridgeType, ContractType};
use mystiko_validator::validate::{
    is_ethereum_address, is_number_string, string_vec_each_not_empty,
};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use typed_builder::TypedBuilder;
use validator::{Validate, ValidationError};

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawPoolContractConfig {
    #[validate(range(min = 1))]
    pub version: u32,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(custom = "is_ethereum_address")]
    pub address: String,

    #[validate(range(min = 1))]
    #[builder(default = None)]
    pub disabled_at: Option<u64>,

    #[serde(rename = "type")]
    #[serde(default = "default_contract_type")]
    #[validate(custom = "validate_contract_type")]
    #[builder(default = default_contract_type())]
    pub contract_type: ContractType,

    #[validate(range(min = 1))]
    pub start_block: u64,

    #[validate(range(min = 1))]
    #[builder(default = None)]
    pub event_filter_size: Option<u64>,

    #[validate(length(min = 1))]
    pub pool_name: String,

    pub bridge_type: BridgeType,

    #[validate(custom = "is_ethereum_address")]
    #[builder(default = None)]
    pub asset_address: Option<String>,

    #[validate(custom = "is_number_string::<true>")]
    #[serde(default = "default_min_rollup_fee")]
    #[builder(default = default_min_rollup_fee())]
    pub min_rollup_fee: String,

    #[validate(custom = "string_vec_each_not_empty")]
    #[serde(default)]
    #[builder(default = vec ! [])]
    pub circuits: Vec<String>,
}

impl Hash for RawPoolContractConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state)
    }
}

fn validate_contract_type(t: &ContractType) -> Result<(), ValidationError> {
    if *t == ContractType::Pool {
        return Ok(());
    }
    Err(ValidationError::new("contract type error"))
}

fn default_min_rollup_fee() -> String {
    "0".to_string()
}

fn default_contract_type() -> ContractType {
    ContractType::Pool
}
