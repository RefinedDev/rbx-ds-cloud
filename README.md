# rbx-ds-cloud &emsp; ![logo](docs/imgs/icon.png)

[![Crate](https://img.shields.io/crates/v/rbx-ds-cloud.svg)](https://crates.io/crates/rbx-ds-cloud)
[![Docs](https://docs.rs/rbx-ds-cloud/badge.svg)](https://docs.rs/rbx-ds-cloud)
[![CI](https://github.com/RefinedDev/rbx-ds-cloud/actions/workflows/ci.yaml/badge.svg)](https://github.com/RefinedDev/rbx-ds-cloud/actions/workflows/ci.yaml)
[![Release](https://github.com/RefinedDev/rbx-ds-cloud/actions/workflows/release.yaml/badge.svg)](https://github.com/RefinedDev/rbx-ds-cloud/actions/workflows/release.yaml)

`rbx-ds-cloud` is a CLI and Library for interacting with [Roblox's Datastore Open Cloud API](https://create.roblox.com/docs/open-cloud/data-store-api)

## Install

## Installing the Binary

- You MUST have [Rust](https://www.rust-lang.org/tools/install) installed

1. You can run the following command to install the binary.

    ```sh
    cargo install rbx_ds_cloud
    ```

    It will be installed on the `$HOME/.cargo/bin` path by default, you may change it in your system's enviorment variables.

2. You can download the repository and build from the source

    ```sh
    git clone https://github.com/RefinedDev/rbx-ds-cloud.git
    cd rbx-ds-cloud

    cargo build --release
    ```

3. Or, download and unzip the tool for your OS from the [releases](https://github.com/RefinedDev/rbx-ds-cloud/releases) page

## Installing the Library

- If you would like to use `rbx-ds-cloud` in a Rust project, just add `rbx_ds_cloud` to the `Cargo.toml` dependancy list of that project

    ```toml
    [dependencies]
    rbx_ds_cloud = "0.1.0"
    ```

- Or, you can use the `cargo add` command

```sh
cargo add rbx_ds_cloud
```

## Use case

This tool is good for Debugging your game's datastores without being on Roblox Studio

## Documentation

<https://refineddev.github.io/rbx-ds-cloud/>