use mystiko_relayer_config::raw::chain::RawChainConfig;
use mystiko_relayer_config::raw::contract::RawContractConfig;
use mystiko_relayer_config::raw::gas_cost::RawGasCostConfig;
use mystiko_relayer_config::raw::transaction_info::RawTransactionInfoConfig;
use mystiko_relayer_config::raw::{create_raw, create_raw_from_file};
use mystiko_types::AssetType;
use std::sync::Arc;
use validator::Validate;

fn default_config() -> RawChainConfig {
    let raw_gas_cost_config = RawGasCostConfig::builder()
        .transaction1x0(500704)
        .transaction1x1(619966)
        .transaction1x2(705128)
        .transaction2x0(598799)
        .transaction2x1(708389)
        .transaction2x2(803183)
        .build();
    create_raw::<RawChainConfig>(
        RawChainConfig::builder()
            .name("Ethereum Goerli".to_string())
            .chain_id(5)
            .asset_symbol("ETH".to_string())
            .relayer_contract_address("0x45B22A8CefDfF00989882CAE48Ad06D57938Efcc".to_string())
            .contracts(vec![
                Arc::new(
                    RawContractConfig::builder()
                        .asset_type(AssetType::Main)
                        .asset_symbol("ETH".to_string())
                        .asset_decimals(18)
                        .relayer_fee_of_ten_thousandth(25)
                        .build(),
                ),
                Arc::new(
                    RawContractConfig::builder()
                        .asset_type(AssetType::Erc20)
                        .asset_symbol("MTT".to_string())
                        .relayer_fee_of_ten_thousandth(25)
                        .build(),
                ),
                Arc::new(
                    RawContractConfig::builder()
                        .asset_type(AssetType::Erc20)
                        .asset_symbol("mUSD".to_string())
                        .relayer_fee_of_ten_thousandth(25)
                        .asset_decimals(6)
                        .build(),
                ),
            ])
            .transaction_info(Arc::new(
                RawTransactionInfoConfig::builder()
                    .main_gas_cost(Arc::new(raw_gas_cost_config.clone()))
                    .erc20_gas_cost(Arc::new(raw_gas_cost_config))
                    .build(),
            ))
            .build(),
    )
    .unwrap()
}

#[test]
fn test_valid_config() {
    let config = default_config();
    assert!(config.validate().is_ok());
}

#[test]
fn test_invalid_name() {
    let mut config = default_config();
    config.name = "".to_string();
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_chain_id() {
    let mut config = default_config();
    config.chain_id = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_asset_symbol() {
    let mut config = default_config();
    config.asset_symbol = "".to_string();
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_relayer_contract_address() {
    let mut config = default_config();
    config.relayer_contract_address = "".to_string();
    assert!(config.validate().is_err());
    config.relayer_contract_address = "45B22A8CefDfF00989882CAE48Ad06D57938Efcc".to_string();
    assert!(config.validate().is_err());
}

#[test]
fn test_default_values() {
    let raw_gas_cost_config = RawGasCostConfig::builder()
        .transaction1x0(500704)
        .transaction1x1(617592)
        .transaction1x2(705128)
        .transaction2x0(598799)
        .transaction2x1(708389)
        .transaction2x2(803183)
        .build();
    let raw_config = create_raw::<RawChainConfig>(
        RawChainConfig::builder()
            .name("Ethereum Goerli".to_string())
            .chain_id(5)
            .asset_symbol("ETH".to_string())
            .relayer_contract_address("0x45B22A8CefDfF00989882CAE48Ad06D57938Efcc".to_string())
            .transaction_info(Arc::new(
                RawTransactionInfoConfig::builder()
                    .main_gas_cost(Arc::new(raw_gas_cost_config.clone()))
                    .erc20_gas_cost(Arc::new(raw_gas_cost_config))
                    .build(),
            ))
            .build(),
    )
    .unwrap();
    assert!(raw_config.validate().is_ok());
    assert_eq!(raw_config.contracts.len(), 0);
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = create_raw_from_file::<RawChainConfig>("tests/files/chain.valid.json")
        .await
        .unwrap();
    assert_eq!(file_config, default_config());
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        create_raw_from_file::<RawChainConfig>("tests/files/chain.invalid.json").await;
    assert!(file_config.is_err());
}
