use mystiko_config::{create_raw, create_raw_from_file, RawPackerConfig};
use mystiko_types::{PackerChecksum, PackerCompression};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use validator::Validate;

fn default_config() -> RawPackerConfig {
    create_raw::<RawPackerConfig>(
        RawPackerConfig::builder()
            .url("https://static.mystiko.network/packer/v1")
            .build(),
    )
    .unwrap()
}

fn full_config() -> RawPackerConfig {
    create_raw::<RawPackerConfig>(
        RawPackerConfig::builder()
            .url("https://static.mystiko.network/packer/v2")
            .client_timeout_ms(10000u64)
            .version(2u32)
            .checksum(PackerChecksum::Sha512)
            .compression(PackerCompression::Zstd)
            .build(),
    )
    .unwrap()
}

#[test]
fn test_invalid_url() {
    let mut config = default_config();
    config.url = String::from("not a valid url");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_client_time_out_ms() {
    let mut config = default_config();
    config.client_timeout_ms = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_version() {
    let mut config = default_config();
    config.version = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_hash() {
    let config1 = RawPackerConfig::builder()
        .url("https://static.mystiko.network/packer/v1")
        .build();
    let config2 = RawPackerConfig::builder()
        .url("https://static.mystiko.network/packer/v2")
        .build();
    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();
    config1.hash(&mut hasher1);
    config2.hash(&mut hasher2);
    assert_ne!(hasher1.finish(), hasher2.finish());
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = create_raw_from_file::<RawPackerConfig>("tests/files/packer/valid.json")
        .await
        .unwrap();
    assert_eq!(file_config, default_config());
}

#[tokio::test]
async fn test_import_full_json_file() {
    let file_config = create_raw_from_file::<RawPackerConfig>("tests/files/packer/full.json")
        .await
        .unwrap();
    assert_eq!(file_config, full_config());
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        create_raw_from_file::<RawPackerConfig>("tests/files/packer/invalid.json").await;
    assert!(file_config.is_err());
}
