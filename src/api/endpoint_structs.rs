//! Structs used as Arguments for the Roblox API

use clap::{Args, ValueEnum};

#[derive(Args, Debug)]
/// List all Datastores in the specified "universe" or game
pub struct ListDataStores {
    /// The value of DataModel.GameId, which is visible in the URL on the universe's Configure page.
    #[clap(short, long, value_parser)]
    pub universe_id: u64,

    /// API key of Roblox Open Cloud
    #[clap(short, long, value_parser)]
    pub api_key: String,

    /// (INTEGER) Maximum number of items to return
    #[clap(short, long, value_parser)]
    pub limit: u64,

    /// (Optional) Return only data stores with this prefix
    #[clap(short, long, value_parser)]
    pub prefix: Option<String>,

    /// (Optional) Cursor for the next set of data
    #[clap(short, long, value_parser)]
    pub cursor: Option<String>,
}

#[derive(Args, Debug)]
/// Returns a list of entry keys within a data store.
pub struct ListEntries {
    /// The value of DataModel.GameId, which is visible in the URL on the universe's Configure page.
    #[clap(short, long, value_parser)]
    pub universe_id: u64,

    /// API key of Roblox Open Cloud
    #[clap(short, long, value_parser)]
    pub api_key: String,

    /// Name of the data store
    #[clap(short, long, value_parser)]
    pub datastore_name: String,

    /// If true, return keys from all scopes
    #[clap(short = 'o', long, value_parser)]
    pub all_scopes: Option<bool>,

    /// (Optional) If "None", defaults to global, similar to Lua API
    #[clap(short, long, value_parser)]
    pub scope: Option<String>,

    /// (Optional) Return only keys with this prefix
    #[clap(short, long, value_parser)]
    pub prefix: Option<String>,

    /// (INTEGER) Maximum number of items to return
    #[clap(short, long, value_parser)]
    pub limit: u64,

    /// (Optional) Provide to request the next set of data
    #[clap(short, long, value_parser)]
    pub cursor: Option<String>,
}

#[derive(Args, Debug)]
/// Returns the value associated with an entry.
pub struct GetEntry {
    /// The value of DataModel.GameId, which is visible in the URL on the universe's Configure page.
    #[clap(short, long, value_parser)]
    pub universe_id: u64,

    /// API key of Roblox Open Cloud
    #[clap(short, long, value_parser)]
    pub api_key: String,

    /// Name of the data store
    #[clap(short, long, value_parser)]
    pub datastore_name: String,

    /// (Optional) If "None", defaults to global, similar to Lua API
    #[clap(short, long, value_parser)]
    pub scope: Option<String>,

    /// The key which identifies the entry
    #[clap(short, long, value_parser)]
    pub key: String,
}

#[derive(Args, Debug)]
/// Returns the versions and metadata of an Entry of a datastore
pub struct ListEntryVersions {
    /// The value of DataModel.GameId, which is visible in the URL on the universe's Configure page.
    #[clap(short, long, value_parser)]
    pub universe_id: u64,

    /// API key of Roblox Open Cloud
    #[clap(short, long, value_parser)]
    pub api_key: String,

    /// Name of the data store
    #[clap(short, long, value_parser)]
    pub datastore_name: String,

    /// (Optional) If "None", defaults to global, similar to Lua API
    #[clap(short, long, value_parser)]
    pub scope: Option<String>,

    /// The key which identifies the entry
    #[clap(short, long, value_parser)]
    pub key: String,

    /// (Optional) Start time constraint (ISO UTC Datetime)
    #[clap(short = 't', long, value_parser)]
    pub start_time: Option<String>,

    /// (Optional) End time constraint (ISO UTC Datetime)
    #[clap(short = 'e', long, value_parser)]
    pub end_time: Option<String>,

    /// Sort order
    #[clap(short = 'o', long, value_enum)]
    pub sort_order: SortOrder,

    /// (INTEGER) Maximum number of items to return
    #[clap(short, long, value_parser)]
    pub limit: u64,

    /// (Optional) Cursor for the next set of data
    #[clap(short, long, value_parser)]
    pub cursor: Option<String>,
}

#[derive(Args, Debug)]
/// Returns the metadata of a specific version of an entry.
pub struct GetEntryVersion {
    /// The value of DataModel.GameId, which is visible in the URL on the universe's Configure page.
    #[clap(short, long, value_parser)]
    pub universe_id: u64,

    /// API key of Roblox Open Cloud
    #[clap(short, long, value_parser)]
    pub api_key: String,

    /// Name of the data store
    #[clap(short, long, value_parser)]
    pub datastore_name: String,

    /// (Optional) If "None", defaults to global, similar to Lua API
    #[clap(short, long, value_parser)]
    pub scope: Option<String>,

    /// The key which identifies the entry
    #[clap(short, long, value_parser)]
    pub key: String,

    /// The version of the key
    #[clap(short, long, value_parser)]
    pub version_id: String,
}

#[derive(Args, Debug)]
/// Sets the value, metadata and user IDs associated with an entry.
pub struct SetEntry {
    /// The value of DataModel.GameId, which is visible in the URL on the universe's Configure page.
    #[clap(short, long, value_parser)]
    pub universe_id: u64,

    /// API key of Roblox Open Cloud
    #[clap(short, long, value_parser)]
    pub api_key: String,

    /// Name of the data store
    #[clap(short, long, value_parser)]
    pub datastore_name: String,

    /// (Optional) If "None", defaults to global, similar to Lua API
    #[clap(short, long, value_parser)]
    pub scope: Option<String>,

    /// The key which identifies the entry
    #[clap(short, long, value_parser)]
    pub key: String,

    /// (Optional) Only update if current version matches this
    #[clap(short = 'i', long, value_parser)]
    pub match_version: Option<String>,

    /// (Optional) Only create the entry if it does not exist
    #[clap(short, long, value_parser)]
    pub exclusive_create: Option<bool>,

    /// JSON-stringified or stringify-able data (Limit: 4MB)
    #[clap(short = 'D', long, value_parser)]
    pub data: String,

    /// (Optional) Associated UserID (can be multiple)
    #[clap(short = 'U', long, value_parser)]
    pub user_ids: Option<Vec<u64>>,

    /// (Optional) JSON-stringified attributes data
    #[clap(short = 't', long, value_parser)]
    pub attributes: Option<String>,
}

#[derive(Args, Debug)]
/// Increments the value for an entry by a given amount, or create a new entry with that amount.
pub struct IncrementEntry {
    /// The value of DataModel.GameId, which is visible in the URL on the universe's Configure page.
    #[clap(short, long, value_parser)]
    pub universe_id: u64,

    /// API key of Roblox Open Cloud
    #[clap(short, long, value_parser)]
    pub api_key: String,

    /// Name of the data store
    #[clap(short, long, value_parser)]
    pub datastore_name: String,

    /// (Optional) If "None", defaults to global, similar to Lua API
    #[clap(short, long, value_parser)]
    pub scope: Option<String>,

    /// The key which identifies the entry
    #[clap(short, long, value_parser)]
    pub key: String,

    /// The amount by which the entry should be incremented
    #[clap(short, long, value_parser)]
    pub increment_by: f64,

    /// (Optional) Comma-separated list of Roblox user IDs
    #[clap(short = 'U', long, value_parser)]
    pub user_ids: Option<Vec<u64>>,

    /// (Optional) JSON-stringified attributes data
    #[clap(short = 't', long, value_parser)]
    pub attributes: Option<String>,
}

#[derive(Args, Debug)]
/// Marks the entry as deleted by creating a tombstone version. Entries are deleted permanently after 30 days.
pub struct DeleteEntry {
    /// The value of DataModel.GameId, which is visible in the URL on the universe's Configure page.
    #[clap(short, long, value_parser)]
    pub universe_id: u64,

    /// API key of Roblox Open Cloud
    #[clap(short, long, value_parser)]
    pub api_key: String,

    /// Name of the data store
    #[clap(short, long, value_parser)]
    pub datastore_name: String,

    /// (Optional) If "None", defaults to global, similar to Lua API
    #[clap(short, long, value_parser)]
    pub scope: Option<String>,

    /// The key which identifies the entry
    #[clap(short, long, value_parser)]
    pub key: String,
}

#[derive(Debug, Clone, ValueEnum)]
/// An Enum for the ListEntryVersions API
pub enum SortOrder {
    Ascending,
    Descending,
}
