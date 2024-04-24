use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub enum BridgeType {
    #[default]
    Loop,
    Poly,
    Tbridge,
    Celer,
    LayerZero,
    Axelar,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ContractType {
    Deposit,
    Pool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum AssetType {
    #[default]
    Erc20,
    Main,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Copy)]
#[serde(rename_all = "lowercase")]
pub enum CircuitType {
    Rollup1,
    Rollup2,
    Rollup4,
    Rollup8,
    Rollup16,
    Transaction1x0,
    Transaction1x1,
    Transaction1x2,
    Transaction2x0,
    Transaction2x1,
    Transaction2x2,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ProviderType {
    #[default]
    Failover,
    Quorum,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum NetworkType {
    #[default]
    Testnet,
    Mainnet,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum PackerChecksum {
    #[default]
    Sha512,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum PackerCompression {
    #[default]
    Zstd,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum TransactionType {
    Legacy,
    #[default]
    Eip1559,
    Eip2930,
}

impl CircuitType {
    pub fn all() -> Vec<CircuitType> {
        vec![
            CircuitType::Rollup1,
            CircuitType::Rollup2,
            CircuitType::Rollup4,
            CircuitType::Rollup8,
            CircuitType::Rollup16,
            CircuitType::Transaction1x0,
            CircuitType::Transaction1x1,
            CircuitType::Transaction1x2,
            CircuitType::Transaction2x0,
            CircuitType::Transaction2x1,
            CircuitType::Transaction2x2,
        ]
    }
}
