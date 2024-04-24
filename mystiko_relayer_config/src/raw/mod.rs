pub mod chain;
pub mod contract;
pub mod gas_cost;
pub mod relayer;
pub mod transaction_info;

use anyhow::bail;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_str;
use std::fmt::Debug;
use validator::Validate;

pub fn validate_raw<T>(raw: &T) -> anyhow::Result<()>
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

pub fn create_raw<T>(plain: T) -> anyhow::Result<T>
where
    T: Validate + Serialize + DeserializeOwned + Debug,
{
    match validate_raw(&plain) {
        Ok(_) => Ok(plain),
        Err(err) => Err(err),
    }
}

pub fn create_raw_from_json<T>(json_str: &str) -> anyhow::Result<T>
where
    T: Validate + Serialize + DeserializeOwned + Debug,
{
    let object: T = from_str(json_str)?;
    create_raw::<T>(object)
}

#[cfg(feature = "fs")]
pub async fn create_raw_from_file<T>(json_file: &str) -> anyhow::Result<T>
where
    T: Validate + Serialize + DeserializeOwned + Debug,
{
    let contents = tokio::fs::read_to_string(std::path::PathBuf::from(json_file)).await?;
    let result: T = from_str(&contents)?;
    create_raw::<T>(result)
}
