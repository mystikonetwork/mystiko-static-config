use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawGasCostConfig {
    #[validate(range(min = 1))]
    pub transaction1x0: u32,

    #[validate(range(min = 1))]
    pub transaction1x1: u32,

    #[validate(range(min = 1))]
    pub transaction1x2: u32,

    #[validate(range(min = 1))]
    pub transaction2x0: u32,

    #[validate(range(min = 1))]
    pub transaction2x1: u32,

    #[validate(range(min = 1))]
    pub transaction2x2: u32,
}
