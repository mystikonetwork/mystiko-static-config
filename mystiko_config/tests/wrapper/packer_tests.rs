use mystiko_config::{create_raw_from_file, PackerConfig, RawPackerConfig};
use mystiko_types::{PackerChecksum, PackerCompression};
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/packer/valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawPackerConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = PackerConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.url(), "https://static.mystiko.network/packer/v1");
    assert_eq!(config.client_timeout_ms(), 15000);
    assert_eq!(config.version(), 1);
    assert_eq!(config.checksum(), &PackerChecksum::Sha512);
    assert_eq!(config.compression(), &PackerCompression::Zstd);
}
