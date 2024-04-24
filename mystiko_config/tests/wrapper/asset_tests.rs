use mystiko_config::{create_raw_from_file, AssetConfig, RawAssetConfig};
use mystiko_types::AssetType;
use num_bigint::BigUint;
use std::str::FromStr;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/asset/valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawAssetConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = AssetConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.asset_type(), &AssetType::Erc20);
    assert_eq!(config.asset_symbol(), "MTT");
    assert_eq!(config.asset_decimals(), 16);
    assert_eq!(
        config.asset_address(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert_eq!(
        config.recommended_amounts().unwrap(),
        vec![
            BigUint::from_str("10000000000000000").unwrap(),
            BigUint::from_str("100000000000000000").unwrap(),
        ]
    );
    assert_eq!(
        config.recommended_amounts_number::<u32>().unwrap(),
        vec![1, 10]
    );
}
