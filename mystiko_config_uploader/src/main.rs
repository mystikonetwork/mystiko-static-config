extern crate anyhow;
extern crate clap;
extern crate env_logger;
extern crate log;
extern crate mystiko_config;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate serde_json;
extern crate tokio;

use anyhow::Result;
use clap::{Args, Parser, Subcommand, ValueEnum};
use mystiko_config::MystikoConfig;
use mystiko_relayer_config::wrapper::relayer::RelayerConfig;
use rusoto_core::Region;
use rusoto_s3::{HeadObjectRequest, PutObjectRequest, S3Client, S3};
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs::read_to_string;

const DEFAULT_BUCKET: &str = "static.mystiko.network";
const DEFAULT_REGION: &str = "us-east-1";

#[derive(ValueEnum, Clone)]
enum ConfigType {
    Core,
    Relayer,
}

#[derive(Args)]
struct UploadArgs {
    path: String,
    git_revision: String,
    #[arg(long = "type", value_enum, default_value_t = ConfigType::Core)]
    config_type: ConfigType,
    #[arg(short, long, default_value_t = String::from(DEFAULT_BUCKET))]
    bucket: String,
    #[arg(short, long, default_value_t = String::from(DEFAULT_REGION))]
    region: String,
    #[arg(short, long)]
    testnet: bool,
    #[arg(short, long)]
    latest: bool,
    #[arg(short, long)]
    overwrite: bool,
    #[arg(short, long)]
    production: bool,
}

#[derive(Parser)]
#[command(about = "Tool for validating/uploading Mystiko config")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Validate {
        path: String,
        #[arg(long = "type", value_enum, default_value_t = ConfigType::Core)]
        config_type: ConfigType,
    },
    Upload(UploadArgs),
}

async fn validate_config_string(
    config_string: &str,
    config_type: &ConfigType,
    git_revision: Option<String>,
) -> Result<String> {
    let config_str = match config_type {
        ConfigType::Core => {
            let mut config_value: serde_json::Value = serde_json::from_str(config_string)?;
            if let Some(git_revision) = git_revision {
                config_value["gitRevision"] = serde_json::Value::String(git_revision);
            }
            MystikoConfig::from_raw(serde_json::from_value(config_value.clone())?)?;
            serde_json::to_string(&config_value)?
        }
        ConfigType::Relayer => {
            let mut config_value: serde_json::Value = serde_json::from_str(config_string)?;
            if let Some(git_revision) = git_revision {
                config_value["gitRevision"] = serde_json::Value::String(git_revision);
            }
            RelayerConfig::from_raw(serde_json::from_value(config_value.clone())?)?;
            serde_json::to_string(&config_value)?
        }
    };
    log::info!("Config is valid");
    Ok(config_str)
}

async fn validate_config(
    path: &str,
    config_type: &ConfigType,
    git_revision: Option<String>,
) -> Result<String> {
    let config_str = read_to_string(PathBuf::from(path)).await?;
    validate_config_string(&config_str, config_type, git_revision).await
}

async fn upload_config(args: &UploadArgs) -> Result<String> {
    let config_str = validate_config(
        &args.path,
        &args.config_type,
        Some(args.git_revision.clone()),
    )
    .await?;
    let region = Region::from_str(&args.region)?;
    let s3_client = S3Client::new(region);
    let content_type = Some(String::from("application/json"));
    let acl = Some(String::from("public-read"));
    let config_content: Vec<u8> = config_str.clone().into();
    let config_key = format!(
        "{}/{}/{}/{}/config.json",
        match args.config_type {
            ConfigType::Core => {
                "config"
            }
            ConfigType::Relayer => {
                "relayer_config"
            }
        },
        if args.production {
            "production"
        } else {
            "staging"
        },
        if args.testnet { "testnet" } else { "mainnet" },
        args.git_revision
    );
    let config_exists = s3_client
        .head_object(HeadObjectRequest {
            bucket: args.bucket.clone(),
            key: config_key.clone(),
            ..HeadObjectRequest::default()
        })
        .await
        .is_ok();
    if !config_exists || args.overwrite {
        let request = PutObjectRequest {
            bucket: args.bucket.clone(),
            key: config_key.clone(),
            body: Some(config_content.clone().into()),
            content_type: content_type.clone(),
            acl: acl.clone(),
            ..PutObjectRequest::default()
        };
        s3_client.put_object(request).await?;
        log::info!("Uploaded config to s3://{}/{}", &args.bucket, config_key);
        if args.latest {
            let latest_config_key = format!(
                "{}/{}/{}/latest.json",
                match args.config_type {
                    ConfigType::Core => {
                        "config"
                    }
                    ConfigType::Relayer => {
                        "relayer_config"
                    }
                },
                if args.production {
                    "production"
                } else {
                    "staging"
                },
                if args.testnet { "testnet" } else { "mainnet" }
            );
            let latest_request = PutObjectRequest {
                bucket: args.bucket.clone(),
                key: latest_config_key.clone(),
                body: Some(config_content.into()),
                cache_control: Some(String::from("no-store")),
                content_type,
                acl,
                ..PutObjectRequest::default()
            };
            s3_client.put_object(latest_request).await?;
            log::info!(
                "Uploaded config to s3://{}/{}",
                &args.bucket,
                latest_config_key
            );
        }
    } else {
        log::warn!("Config already exists, skipping upload");
    }
    Ok(config_str)
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Validate { path, config_type } => {
            validate_config(path, config_type, None).await?
        }
        Commands::Upload(args) => upload_config(args).await?,
    };
    Ok(())
}
