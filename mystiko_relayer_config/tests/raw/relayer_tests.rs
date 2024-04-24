use mystiko_relayer_config::raw::chain::RawChainConfig;
use mystiko_relayer_config::raw::contract::RawContractConfig;
use mystiko_relayer_config::raw::gas_cost::RawGasCostConfig;
use mystiko_relayer_config::raw::relayer::RawRelayerConfig;
use mystiko_relayer_config::raw::transaction_info::RawTransactionInfoConfig;
use mystiko_relayer_config::raw::{create_raw, create_raw_from_file};
use mystiko_types::AssetType;
use std::sync::Arc;
use validator::Validate;

fn default_chain_config() -> RawChainConfig {
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
                        .asset_decimals(6)
                        .relayer_fee_of_ten_thousandth(25)
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

fn default_config() -> RawRelayerConfig {
    create_raw::<RawRelayerConfig>(
        RawRelayerConfig::builder()
            .version("0.0.1".to_string())
            .git_revision(Some("3f25038".parse().unwrap()))
            .chains(vec![Arc::new(default_chain_config())])
            .build(),
    )
    .unwrap()
}

#[test]
fn test_invalid_version_0() {
    let mut config = default_config();
    config.version = "".to_string();
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_version_1() {
    let mut config = default_config();
    config.version = "wrong version".to_string();
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = create_raw_from_file::<RawRelayerConfig>("tests/files/relayer.valid.json")
        .await
        .unwrap();
    assert_eq!(file_config, default_config());
}

#[tokio::test]
async fn test_import_invalid_json_file_0() {
    let file_config =
        create_raw_from_file::<RawRelayerConfig>("tests/files/relayer.invalid.0.json").await;
    assert!(file_config.is_err());
    let err_msg = file_config.unwrap_err().to_string();
    assert_eq!(
        err_msg.as_str(),
        "missing field `assetDecimals` at line 48 column 5"
    );
}

#[tokio::test]
async fn test_import_invalid_json_file_1() {
    let file_config =
        create_raw_from_file::<RawRelayerConfig>("tests/files/relayer.invalid.1.json").await;
    assert!(file_config.is_err());
    let err_msg = file_config.unwrap_err().to_string();
    assert_eq!(
        err_msg.as_str(),
        "missing field `assetType` at line 16 column 9"
    );
}

#[tokio::test]
async fn test_import_invalid_json_file_2() {
    let file_config =
        create_raw_from_file::<RawRelayerConfig>("tests/files/relayer.invalid.2.json").await;
    assert!(file_config.is_err());
    let err_msg = file_config.unwrap_err().to_string();
    assert_eq!(
        err_msg.as_str(),
        "missing field `assetDecimals` at line 16 column 9"
    );
}
