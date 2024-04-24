use mystiko_relayer_config::raw::chain::RawChainConfig;
use mystiko_relayer_config::raw::contract::RawContractConfig;
use mystiko_relayer_config::raw::create_raw_from_file;
use mystiko_relayer_config::raw::transaction_info::RawTransactionInfoConfig;
use mystiko_relayer_config::wrapper::chain::ChainConfig;
use mystiko_relayer_config::wrapper::contract::ContractConfig;
use mystiko_relayer_config::wrapper::transaction_info::TransactionInfoConfig;
use mystiko_types::{AssetType, CircuitType};
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/chain.valid.json";
const VALID_GAS_COST_CONFIG_FILE: &str = "tests/files/transaction_info.valid.json";
const VALID_CONTRACT_CONFIG_FILE: &str = "tests/files/contract.valid.json";

async fn test_create_transaction_info() -> TransactionInfoConfig {
    let raw_config = create_raw_from_file::<RawTransactionInfoConfig>(VALID_GAS_COST_CONFIG_FILE)
        .await
        .unwrap();
    let config = TransactionInfoConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    config
}

async fn test_create_contract() -> ContractConfig {
    let raw_config = create_raw_from_file::<RawContractConfig>(VALID_CONTRACT_CONFIG_FILE)
        .await
        .unwrap();
    let config = ContractConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    config
}

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawChainConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = ChainConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.name(), "Ethereum Goerli");
    assert_eq!(config.chain_id(), 5);
    assert_eq!(config.asset_symbol(), "ETH");
    assert_eq!(
        config.relayer_contract_address(),
        "0x45B22A8CefDfF00989882CAE48Ad06D57938Efcc"
    );
    assert_eq!(config.contracts().len(), 3);
    assert_eq!(
        config.transaction_info(),
        &test_create_transaction_info().await
    );

    let contract_option = config.find_contract("ETH");
    assert!(contract_option.is_some());
    assert_eq!(contract_option.unwrap(), &test_create_contract().await);
    assert!(config.find_contract("BNB").is_none());

    assert_eq!(
        config
            .find_gas_cost(&AssetType::Main, &CircuitType::Transaction1x0)
            .unwrap(),
        500704
    );
    assert_eq!(
        config
            .find_gas_cost(&AssetType::Main, &CircuitType::Transaction1x1)
            .unwrap(),
        619966
    );
    assert_eq!(
        config
            .find_gas_cost(&AssetType::Main, &CircuitType::Transaction1x2)
            .unwrap(),
        705128
    );
    assert_eq!(
        config
            .find_gas_cost(&AssetType::Erc20, &CircuitType::Transaction2x0)
            .unwrap(),
        598799
    );
    assert_eq!(
        config
            .find_gas_cost(&AssetType::Erc20, &CircuitType::Transaction2x1)
            .unwrap(),
        708389
    );
    assert_eq!(
        config
            .find_gas_cost(&AssetType::Erc20, &CircuitType::Transaction2x2)
            .unwrap(),
        803183
    );
    assert_eq!(
        config
            .find_gas_cost(&AssetType::Erc20, &CircuitType::Rollup1)
            .unwrap_err()
            .to_string(),
        String::from("unsupported circuit type Rollup1")
    )
}
