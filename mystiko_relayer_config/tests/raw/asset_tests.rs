use mystiko_relayer_config::raw::contract::RawContractConfig;
use mystiko_relayer_config::raw::{create_raw, create_raw_from_file};
use mystiko_types::AssetType;
use rust_decimal::Decimal;
use validator::Validate;

fn default_config() -> RawContractConfig {
    create_raw::<RawContractConfig>(
        RawContractConfig::builder()
            .asset_type(AssetType::Main)
            .asset_symbol("ETH".to_string())
            .relayer_fee_of_ten_thousandth(25)
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
fn test_invalid_asset_symbol() {
    let mut config = default_config();
    config.asset_symbol = "".to_string();
    assert!(config.validate().is_err());
}

#[test]
fn test_default_minimum_gas_fee() {
    let config = default_config();
    assert_eq!(config.minimum_gas_fee, Decimal::ZERO);
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = create_raw_from_file::<RawContractConfig>("tests/files/contract.valid.json")
        .await
        .unwrap();
    assert_eq!(file_config, default_config());
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        create_raw_from_file::<RawContractConfig>("tests/files/contract.invalid.json").await;
    assert!(file_config.is_err());
}
