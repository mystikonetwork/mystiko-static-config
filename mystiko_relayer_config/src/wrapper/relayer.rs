use crate::raw::chain::RawChainConfig;
use crate::raw::create_raw_from_json;
use crate::raw::relayer::RawRelayerConfig;
use crate::wrapper::chain::ChainConfig;
use anyhow::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct RelayerConfig {
    raw: RawRelayerConfig,
    chain_configs: Vec<Arc<ChainConfig>>,
    default_chain_configs: HashMap<u64, Arc<ChainConfig>>,
}

#[derive(TypedBuilder, Clone, Debug)]
pub struct RemoteOptions {
    #[builder(default, setter(strip_option))]
    pub base_url: Option<String>,
    #[builder(default, setter(strip_option))]
    pub git_revision: Option<String>,
    #[builder(default = false)]
    pub is_testnet: bool,
    #[builder(default = false)]
    pub is_staging: bool,
}

const DEFAULT_REMOTE_BASE_URL: &str = "https://static.mystiko.network/relayer_config";

impl RelayerConfig {
    fn new(raw: RawRelayerConfig) -> Result<Self> {
        let chain_configs = initialize_chain_configs(&raw.chains)?;
        let default_chain_configs = initialize_default_chain_configs(&chain_configs)?;
        Ok(RelayerConfig {
            raw,
            chain_configs,
            default_chain_configs,
        })
    }

    pub fn from_raw(raw: RawRelayerConfig) -> Result<Self> {
        let config = RelayerConfig::new(raw)?;
        config.validate()?;
        Ok(config)
    }

    pub fn from_json_str(json: &str) -> Result<Self> {
        RelayerConfig::from_raw(create_raw_from_json::<RawRelayerConfig>(json)?)
    }

    #[cfg(feature = "fs")]
    pub async fn from_json_file(json_file: &str) -> Result<Self> {
        RelayerConfig::from_raw(
            crate::raw::create_raw_from_file::<RawRelayerConfig>(json_file).await?,
        )
    }

    pub async fn from_remote(options: &RemoteOptions) -> Result<Self> {
        let base_url = options
            .base_url
            .as_deref()
            .unwrap_or(DEFAULT_REMOTE_BASE_URL);
        let environment = if options.is_staging {
            "staging"
        } else {
            "production"
        };
        let network = if options.is_testnet {
            "testnet"
        } else {
            "mainnet"
        };
        let url = if let Some(git_revision) = &options.git_revision {
            format!(
                "{}/{}/{}/{}/config.json",
                base_url, environment, network, git_revision
            )
        } else {
            format!("{}/{}/{}/latest.json", base_url, environment, network)
        };
        let response = reqwest::get(&url).await?;
        if response.status().is_success() {
            let content = response.text().await?;
            RelayerConfig::from_json_str(&content)
        } else {
            Err(Error::msg(format!(
                "Failed to fetch config from {}: status code {}",
                &url,
                response.status()
            )))
        }
    }

    pub async fn from_remote_default_mainnet() -> Result<Self> {
        RelayerConfig::from_remote(&RemoteOptions::builder().build()).await
    }

    pub async fn from_remote_default_testnet() -> Result<Self> {
        RelayerConfig::from_remote(&RemoteOptions::builder().is_testnet(true).build()).await
    }

    pub fn version(&self) -> &str {
        &self.raw.version
    }

    pub fn find_chain_config(&self, chain_id: u64) -> Option<&ChainConfig> {
        self.default_chain_configs
            .get(&chain_id)
            .map(|c| c.as_ref())
    }

    pub fn chains(&self) -> Vec<&ChainConfig> {
        self.chain_configs.iter().map(|c| c.as_ref()).collect()
    }

    pub fn validate(&self) -> Result<()> {
        self.raw.validate()?;
        for chain_config in self.chains() {
            chain_config.validate()?;
        }
        Ok(())
    }
}

fn initialize_chain_configs(
    raw_chain_configs: &[Arc<RawChainConfig>],
) -> Result<Vec<Arc<ChainConfig>>> {
    let mut chain_configs: Vec<Arc<ChainConfig>> = Vec::new();
    for raw_chain_config in raw_chain_configs {
        chain_configs.push(Arc::new(ChainConfig::new(raw_chain_config.clone())));
    }
    Ok(chain_configs)
}

fn initialize_default_chain_configs(
    chain_configs: &[Arc<ChainConfig>],
) -> Result<HashMap<u64, Arc<ChainConfig>>> {
    let mut configs: HashMap<u64, Arc<ChainConfig>> = HashMap::new();
    for chain_config in chain_configs.iter() {
        if configs.contains_key(&chain_config.chain_id()) {
            return Err(Error::msg(format!(
                "duplicate default chain config for chain_id {:?}",
                chain_config.chain_id()
            )));
        }
        configs.insert(chain_config.chain_id(), chain_config.clone());
    }
    Ok(configs)
}
