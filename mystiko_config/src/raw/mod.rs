mod asset;
mod bridge;
mod chain;
mod circuit;
mod contract;
mod mystiko;
mod packer;
mod provider;
mod sequencer;

pub use asset::*;
pub use bridge::*;
pub use chain::*;
pub use circuit::*;
pub use contract::*;
pub use mystiko::*;
pub use packer::*;
pub use provider::*;
pub use sequencer::*;

use ::validator::Validate;
use anyhow::{bail, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_str;
use std::fmt::Debug;

pub fn validate_raw<T>(raw: &T) -> Result<()>
where
    T: Validate + Debug,
{
    let validate = raw.validate();
    match validate {
        Ok(_) => Ok(()),
        Err(validation) => bail!(
            "failed to validate config object: \n {:?}",
            validation.errors()
        ),
    }
}

pub fn create_raw<T>(plain: T) -> Result<T>
where
    T: Validate + Serialize + DeserializeOwned + Debug,
{
    validate_raw(&plain)?;
    Ok(plain)
}

pub fn create_raw_from_json<T>(json_str: &str) -> Result<T>
where
    T: Validate + Serialize + DeserializeOwned + Debug,
{
    let object: T = from_str(json_str)?;
    create_raw::<T>(object)
}

#[cfg(feature = "fs")]
pub async fn create_raw_from_file<T>(json_file: &str) -> Result<T>
where
    T: Validate + Serialize + DeserializeOwned + Debug,
{
    let contents = tokio::fs::read_to_string(std::path::PathBuf::from(json_file)).await?;
    let result: T = from_str(&contents)?;
    create_raw::<T>(result)
}
