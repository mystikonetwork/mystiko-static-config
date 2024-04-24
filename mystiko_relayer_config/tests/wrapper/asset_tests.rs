use mystiko_relayer_config::raw::contract::RawContractConfig;
use mystiko_relayer_config::raw::create_raw_from_file;
use mystiko_relayer_config::wrapper::contract::ContractConfig;
use mystiko_types::AssetType;
use rust_decimal::Decimal;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/contract.valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawContractConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = ContractConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.asset_decimals(), 18);
    assert_eq!(config.asset_type(), &AssetType::Main);
    assert_eq!(config.asset_symbol(), "ETH");
    assert_eq!(config.relayer_fee_of_ten_thousandth(), 25);
    assert_eq!(config.minimum_gas_fee(), &Decimal::ZERO);
}
