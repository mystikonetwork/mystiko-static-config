use mystiko_relayer_config::raw::gas_cost::RawGasCostConfig;
use mystiko_relayer_config::raw::transaction_info::RawTransactionInfoConfig;
use mystiko_relayer_config::raw::{create_raw, create_raw_from_file};
use std::sync::Arc;

fn default_gas_cost_config() -> RawGasCostConfig {
    create_raw::<RawGasCostConfig>(
        RawGasCostConfig::builder()
            .transaction1x0(500704)
            .transaction1x1(619966)
            .transaction1x2(705128)
            .transaction2x0(598799)
            .transaction2x1(708389)
            .transaction2x2(803183)
            .build(),
    )
    .unwrap()
}

fn default_config() -> RawTransactionInfoConfig {
    create_raw::<RawTransactionInfoConfig>(
        RawTransactionInfoConfig::builder()
            .main_gas_cost(Arc::new(default_gas_cost_config()))
            .erc20_gas_cost(Arc::new(default_gas_cost_config()))
            .build(),
    )
    .unwrap()
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config =
        create_raw_from_file::<RawTransactionInfoConfig>("tests/files/transaction_info.valid.json")
            .await
            .unwrap();
    assert_eq!(file_config, default_config());
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config = create_raw_from_file::<RawTransactionInfoConfig>(
        "tests/files/transaction_info.invalid.json",
    )
    .await;
    assert!(file_config.is_err());
}
