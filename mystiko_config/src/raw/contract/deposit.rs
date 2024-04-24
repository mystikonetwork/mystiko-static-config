use mystiko_types::{BridgeType, ContractType};
use mystiko_validator::validate::{is_ethereum_address, is_number_string};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use typed_builder::TypedBuilder;
use validator::{Validate, ValidationError};

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawDepositContractConfig {
    #[validate(range(min = 1))]
    pub version: u32,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(custom = "is_ethereum_address")]
    pub address: String,

    pub bridge_type: BridgeType,

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

    #[validate(custom = "is_ethereum_address")]
    pub pool_address: String,

    #[validate(range(min = 1))]
    #[builder(default)]
    pub disabled_at: Option<u64>,

    #[validate(range(min = 1))]
    #[builder(default = None)]
    pub peer_chain_id: Option<u64>,

    #[validate(custom = "is_ethereum_address")]
    #[builder(default = None)]
    pub peer_contract_address: Option<String>,

    #[validate(custom = "is_number_string::<true>")]
    pub min_amount: String,

    #[validate(custom = "is_number_string::<true>")]
    pub max_amount: String,

    #[validate(custom = "is_number_string::<true>")]
    #[serde(default = "default_min_bridge_fee")]
    #[builder(default = default_min_bridge_fee())]
    pub min_bridge_fee: String,

    #[validate(custom = "is_number_string::<true>")]
    #[serde(default = "default_min_executor_fee")]
    #[builder(default = default_min_executor_fee())]
    pub min_executor_fee: String,

    #[validate(custom = "is_ethereum_address")]
    #[builder(default = None)]
    pub bridge_fee_asset_address: Option<String>,

    #[validate(custom = "is_ethereum_address")]
    #[builder(default = None)]
    pub executor_fee_asset_address: Option<String>,

    #[validate(range(min = 0))]
    #[builder(default = 0)]
    #[serde(default = "default_service_fee")]
    pub service_fee: u32,

    #[validate(range(min = 1))]
    #[builder(default = 1000000)]
    #[serde(default = "default_service_fee_divider")]
    pub service_fee_divider: u32,
}

impl Hash for RawDepositContractConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state)
    }
}

fn validate_contract_type(t: &ContractType) -> Result<(), ValidationError> {
    if *t == ContractType::Deposit {
        return Ok(());
    }
    Err(ValidationError::new("contract type error"))
}

fn default_contract_type() -> ContractType {
    ContractType::Deposit
}

fn default_min_bridge_fee() -> String {
    "0".to_string()
}

fn default_min_executor_fee() -> String {
    "0".to_string()
}

fn default_service_fee() -> u32 {
    0
}

fn default_service_fee_divider() -> u32 {
    1000000
}
