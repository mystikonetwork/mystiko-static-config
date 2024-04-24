use mystiko_config::{create_raw, create_raw_from_file, RawSequencerConfig};

fn default_config() -> RawSequencerConfig {
    create_raw::<RawSequencerConfig>(RawSequencerConfig::builder().host("example.com").build())
        .unwrap()
}

fn full_config() -> RawSequencerConfig {
    create_raw::<RawSequencerConfig>(
        RawSequencerConfig::builder()
            .host("example.com")
            .port(443)
            .is_ssl(true)
            .build(),
    )
    .unwrap()
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config =
        create_raw_from_file::<RawSequencerConfig>("tests/files/sequencer/valid.json")
            .await
            .unwrap();
    assert_eq!(file_config, default_config());
}

#[tokio::test]
async fn test_import_full_json_file() {
    let file_config = create_raw_from_file::<RawSequencerConfig>("tests/files/sequencer/full.json")
        .await
        .unwrap();
    assert_eq!(file_config, full_config());
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        create_raw_from_file::<RawSequencerConfig>("tests/files/sequencer/invalid.json").await;
    assert!(file_config.is_err());
}
