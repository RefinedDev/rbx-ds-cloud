//! Some of the responses from the Roblox API are deserialized into these structs

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ListDataStoreIndex {
    pub name: String,
    pub createdTime: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ListDataStoresResponse {
    pub datastores: Vec<ListDataStoreIndex>,
    pub nextPageCursor: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ListEntriesIndex {
    pub scope: String,
    pub key: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ListEntriesResponse {
    pub keys: Vec<ListEntriesIndex>,
    pub nextPageCursor: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
#[allow(non_snake_case)]
pub struct EntryVersion {
    pub version: String,
    pub deleted: bool,
    pub contentLength: u64,
    pub createdTime: String,
    pub objectCreatedTime: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ListEntryVersionResponse {
    pub versions: Vec<EntryVersion>,
    pub nextPageCursor: Option<String>,
}
