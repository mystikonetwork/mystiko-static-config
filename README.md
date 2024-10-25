# Mystiko Static Config

[![build status](https://github.com/mystikonetwork/mystiko-static-config/actions/workflows/build.yml/badge.svg)](https://github.com/mystikonetwork/mystiko-static-config/actions/workflows/build.yml)

This repository contains the static configuration files for the [Mystiko](https://mystiko.network) core protocol and gas
relayer.
It also contains the code to parse the configuration files in Rust, and the code to upload the configuration files to
AWS S3.

* [mystiko_config](./mystiko_config): Rust library to parse the configuration files.
* [mystiko_config_uploader](./mystiko_config_uploader): Rust binary to upload the configuration files to AWS S3.
* [mystiko_relayer_abi](./mystiko_relayer_abi): Contract ABI of Mystiko gas relayer in Rust.
* [mystiko_relayer_config](./mystiko_relayer_config): Rust library to parse the gas relayer configuration files.
* [mystiko_types](./mystiko_types): Rust library to define common types used in Mystiko.
* [mystiko_validator](./mystiko_validator): Rust library to validate the configuration files.
