# rbx-ds-cloud &emsp; ![icon](https://user-images.githubusercontent.com/74603055/183870065-2fe503dd-07b6-4d4a-9fa5-bc02fe580cbc.jpg)

[![Crate](https://img.shields.io/crates/v/rbx-ds-cloud.svg)](https://crates.io/crates/rbx-ds-cloud)
[![Docs](https://docs.rs/rbx-ds-cloud/badge.svg)](https://docs.rs/rbx-ds-cloud)
[![CI](https://github.com/RefinedDev/rbx-ds-cloud/actions/workflows/ci.yaml/badge.svg)](https://github.com/RefinedDev/rbx-ds-cloud/actions/workflows/ci.yaml)
[![Release](https://github.com/RefinedDev/rbx-ds-cloud/actions/workflows/release.yaml/badge.svg)](https://github.com/RefinedDev/rbx-ds-cloud/actions/workflows/release.yaml)

`rbx-ds-cloud` is a CLI and Library for interacting with [Roblox's Datastore Open Cloud API](https://create.roblox.com/docs/open-cloud/data-store-api)

## Use case

- This tool is good for Debugging your game's datastores without being on Roblox Studio
- Good for instantaneous changes to the game's datastores
- Continuous Delivery

## Install

## Installing the Library

- If you would like to use `rbx-ds-cloud` in a Rust project, just add `rbx_ds_cloud` to the `Cargo.toml` dependancy list of that project

    ```toml
    [dependencies]
    rbx_ds_cloud = "0.1.3"
    ```

- Or, you can use the `cargo add` command

    ```sh
    cargo add rbx_ds_cloud
    ```

## Installing the Binary

- You can run the following command to install the binary.

    ```sh
    cargo install rbx_ds_cloud
    ```

    It will be installed on the `$HOME/.cargo/bin` path by default, you may change it in your system's enviorment variables.

- You can download the repository and build from the source

    ```sh
    git clone https://github.com/RefinedDev/rbx-ds-cloud.git
    cd rbx-ds-cloud

    cargo build --release
    ```

- Or, download and unzip the tool for your OS from the repository's [releases](https://github.com/RefinedDev/rbx-ds-cloud/releases) page, you then have to add the binary's path to your system's environment variables

## CLI Documentation

<https://refineddev.github.io/rbx-ds-cloud/>
