use mystiko_relayer_config::raw::create_raw_from_file;
use mystiko_relayer_config::raw::gas_cost::RawGasCostConfig;
use mystiko_relayer_config::raw::transaction_info::RawTransactionInfoConfig;
use mystiko_relayer_config::wrapper::transaction_info::{
    Erc20GasCost, MainGasCost, TransactionInfoConfig,
};
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/transaction_info.valid.json";
const VALID_GAS_COST_CONFIG_FILE: &str = "tests/files/gas_cost.valid.json";

async fn create_main_gas_cost() -> MainGasCost {
    let raw_config = create_raw_from_file::<RawGasCostConfig>(VALID_GAS_COST_CONFIG_FILE)
        .await
        .unwrap();
    let config = MainGasCost::new(Arc::new(raw_config));
    config.validate().unwrap();
    config
}

async fn create_erc20_gas_cost() -> Erc20GasCost {
    let raw_config = create_raw_from_file::<RawGasCostConfig>(VALID_GAS_COST_CONFIG_FILE)
        .await
        .unwrap();
    let config = Erc20GasCost::new(Arc::new(raw_config));
    config.validate().unwrap();
    config
}

#[tokio::test]
async fn test_create_main_gas_cost() {
    let _ = create_main_gas_cost().await;
}

#[tokio::test]
async fn test_create_erc20_gas_cost() {
    let _ = create_erc20_gas_cost().await;
}

#[tokio::test]
async fn test_create_transaction_info() {
    let raw_config = create_raw_from_file::<RawTransactionInfoConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = TransactionInfoConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    let main_gas_cost = create_main_gas_cost().await;
    let erc20_gas_cost = create_erc20_gas_cost().await;
    assert_eq!(config.main_gas_cost(), &main_gas_cost);
    assert_eq!(config.erc20_gas_cost(), &erc20_gas_cost);
}

#[tokio::test]
async fn test_gas_cost_config() {
    let config0 = create_erc20_gas_cost().await;
    let gas_cost_config0 = config0.gas_cost_config();
    let config1 = create_main_gas_cost().await;
    let gas_cost_config1 = config1.gas_cost_config();
    assert_eq!(gas_cost_config0, gas_cost_config1);
    assert_eq!(gas_cost_config1.transaction1x0, 500704);
    assert_eq!(gas_cost_config1.transaction1x1, 619966);
    assert_eq!(gas_cost_config1.transaction1x2, 705128);
    assert_eq!(gas_cost_config1.transaction2x0, 598799);
    assert_eq!(gas_cost_config1.transaction2x1, 708389);
    assert_eq!(gas_cost_config1.transaction2x2, 803183);
}
