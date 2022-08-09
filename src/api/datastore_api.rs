//! All the functions interacting with the Roblox Datastore Open Cloud API
use reqwest::{Client, Response};

use crate::api::endpoint_structs::*;
use crate::api::error_types::*;

use md5::{Digest, Md5};

async fn build_endpoint_url(universe_id: &u64, endpoint: &str) -> String {
    let mut ep_url = format!(
        "https://apis.roblox.com/datastores/v1/universes/{}",
        universe_id
    );
    ep_url.push_str(endpoint);
    ep_url
}

async fn res_error_check(res: Response) -> Result<Response, Error> {
    if res.status().is_success() {
        Ok(res)
    } else {
        let rbx_error = res.json::<DataStoreErrorResponse>().await?;
        Err(Error::DataStoreAPIError(rbx_error))
    }
}

async fn vec_to_strlist(v: &Option<Vec<u64>>) -> String {
    v.as_ref()
        .iter()
        .map(|i| format!("{:#?}", i))
        .collect::<Vec<String>>()
        .join(",")
}

/// List all Datastores in the specified "universe" or game
pub async fn list_data_stores(lds_struct: &ListDataStores) -> Result<Response, Error> {
    let mut target = build_endpoint_url(&lds_struct.universe_id, "/standard-datastores").await;
    target.push_str(format!("?limit={}", &lds_struct.limit).as_str());

    if let Some(v) = &lds_struct.prefix {
        target.push_str(format!("&prefix={}", v).as_str());
    }
    if let Some(v) = &lds_struct.cursor {
        target.push_str(format!("&cursor={}", v).as_str());
    }

    let res = res_error_check(
        Client::new()
            .get(target)
            .header("x-api-key", &lds_struct.api_key)
            .send()
            .await?,
    )
    .await?;

    Ok(res)
}

/// Returns a list of entry keys within a data store.
pub async fn list_entries(ls_struct: &ListEntries) -> Result<Response, Error> {
    let mut target = build_endpoint_url(
        &ls_struct.universe_id,
        "/standard-datastores/datastore/entries",
    )
    .await;
    target.push_str(format!("?limit={}", &ls_struct.limit).as_str());
    target.push_str(format!("&datastoreName={}", &ls_struct.datastore_name).as_str());
    target.push_str(format!("&AllScopes={}", &ls_struct.all_scopes.unwrap_or(true)).as_str());

    if let Some(v) = &ls_struct.prefix {
        target.push_str(format!("&prefix={}", v).as_str());
    }
    if let Some(v) = &ls_struct.cursor {
        target.push_str(format!("&cursor={}", v).as_str());
    }
    if let Some(v) = &ls_struct.scope {
        target.push_str(format!("&scope={}", v).as_str());
    }

    let res = res_error_check(
        Client::new()
            .get(target)
            .header("x-api-key", &ls_struct.api_key)
            .send()
            .await?,
    )
    .await?;

    Ok(res)
}

/// Returns the value associated with an entry.
pub async fn get_entry(entry_struct: &GetEntry) -> Result<Response, Error> {
    let mut target = build_endpoint_url(
        &entry_struct.universe_id,
        "/standard-datastores/datastore/entries/entry",
    )
    .await;
    target.push_str(format!("?datastoreName={}", &entry_struct.datastore_name).as_str());
    target.push_str(format!("&entryKey={}", &entry_struct.key).as_str());

    if let Some(v) = &entry_struct.scope {
        target.push_str(format!("&scope={}", v).as_str());
    }

    let res = res_error_check(
        Client::new()
            .get(target)
            .header("x-api-key", &entry_struct.api_key)
            .send()
            .await?,
    )
    .await?;

    Ok(res)
}

/// Returns the versions and metadata of an Entry of a datastore
pub async fn list_entries_version(lev_struct: &ListEntryVersions) -> Result<Response, Error> {
    let mut target = build_endpoint_url(
        &lev_struct.universe_id,
        "/standard-datastores/datastore/entries/entry/versions",
    )
    .await;
    target.push_str(format!("?limit={}", &lev_struct.limit).as_str());
    target.push_str(format!("&datastoreName={}", &lev_struct.datastore_name).as_str());
    target.push_str(format!("&entryKey={}", &lev_struct.key).as_str());
    target.push_str(format!("&sortOrder={:?}", &lev_struct.sort_order).as_str());

    if let Some(v) = &lev_struct.cursor {
        target.push_str(format!("&cursor={}", v).as_str());
    }
    if let Some(v) = &lev_struct.start_time {
        target.push_str(format!("&startTime={}", v).as_str());
    }
    if let Some(v) = &lev_struct.end_time {
        target.push_str(format!("&endTime={}", v).as_str());
    }
    if let Some(v) = &lev_struct.scope {
        target.push_str(format!("&scope={}", v).as_str());
    }

    let res = res_error_check(
        Client::new()
            .get(target)
            .header("x-api-key", &lev_struct.api_key)
            .send()
            .await?,
    )
    .await?;

    Ok(res)
}

/// Returns the metadata of a specific version of an entry.
pub async fn get_entry_version(gev_struct: &GetEntryVersion) -> Result<Response, Error> {
    let mut target = build_endpoint_url(
        &gev_struct.universe_id,
        "/standard-datastores/datastore/entries/entry/versions/version",
    )
    .await;
    target.push_str(format!("?datastoreName={}", &gev_struct.datastore_name).as_str());
    target.push_str(format!("&entryKey={}", &gev_struct.key).as_str());
    target.push_str(format!("&versionId={}", &gev_struct.version_id).as_str());

    if let Some(v) = &gev_struct.scope {
        target.push_str(format!("&scope={}", v).as_str());
    }

    let res = res_error_check(
        Client::new()
            .get(target)
            .header("x-api-key", &gev_struct.api_key)
            .send()
            .await?,
    )
    .await?;

    Ok(res)
}

/// Increments the value for an entry by a given amount, or create a new entry with that amount.
pub async fn increment_entry(increment_struct: &IncrementEntry) -> Result<Response, Error> {
    let mut target = build_endpoint_url(
        &increment_struct.universe_id,
        "/standard-datastores/datastore/entries/entry/increment",
    )
    .await;
    target.push_str(format!("?datastoreName={}", &increment_struct.datastore_name).as_str());
    target.push_str(format!("&entryKey={}", &increment_struct.key).as_str());
    target.push_str(format!("&incrementBy={}", &increment_struct.increment_by).as_str());

    if let Some(v) = &increment_struct.scope {
        target.push_str(format!("&scope={}", v).as_str());
    }

    let res = res_error_check(
        Client::new()
            .post(target)
            .header("x-api-key", &increment_struct.api_key)
            .header(
                "roblox-entry-userids",
                vec_to_strlist(&increment_struct.user_ids).await,
            )
            .header(
                "roblox-entry-attributes",
                increment_struct
                    .attributes
                    .as_ref()
                    .unwrap_or(&String::from("{}")),
            )
            .send()
            .await?,
    )
    .await?;

    Ok(res)
}

/// Marks the entry as deleted by creating a tombstone version. Entries are deleted permanently after 30 days.
pub async fn delete_entry(delete_struct: &DeleteEntry) -> Result<Response, Error> {
    let mut target = build_endpoint_url(
        &delete_struct.universe_id,
        "/standard-datastores/datastore/entries/entry",
    )
    .await;

    target.push_str(format!("?datastoreName={}", &delete_struct.datastore_name).as_str());
    target.push_str(format!("&entryKey={}", &delete_struct.key).as_str());

    if let Some(v) = &delete_struct.scope {
        target.push_str(format!("&scope={}", v).as_str());
    }

    let res = res_error_check(
        Client::new()
            .delete(target)
            .header("x-api-key", &delete_struct.api_key)
            .send()
            .await?,
    )
    .await?;

    Ok(res)
}

/// Sets the value, metadata and user IDs associated with an entry.
pub async fn set_entry(set_struct: &SetEntry) -> Result<Response, Error> {
    let mut target = build_endpoint_url(
        &set_struct.universe_id,
        "/standard-datastores/datastore/entries/entry",
    )
    .await;
    target.push_str(format!("?datastoreName={}", &set_struct.datastore_name).as_str());
    target.push_str(format!("&entryKey={}", &set_struct.key).as_str());

    if let Some(v) = &set_struct.scope {
        target.push_str(format!("&scope={}", v).as_str());
    }
    if let Some(v) = &set_struct.match_version {
        target.push_str(format!("&matchVersion={}", v).as_str());
    }
    if let Some(v) = &set_struct.exclusive_create {
        target.push_str(format!("&exclusiveCreate={}", v).as_str());
    }

    // Convert String Data to Json String
    // let json_data = json!(&set_struct.data).to_string();
    // println!("{:#?}", json_data.to_string());

    // Convert data to Base64 encoded MD5 Checksum
    let mut hasher = Md5::new();
    hasher.update(&set_struct.data.as_bytes());
    let hasher = base64::encode(hasher.finalize());

    let res = res_error_check(
        Client::new()
            .post(target)
            .header("x-api-key", &set_struct.api_key)
            .header(
                "roblox-entry-userids",
                vec_to_strlist(&set_struct.user_ids).await,
            )
            .header(
                "roblox-entry-attributes",
                set_struct
                    .attributes
                    .as_ref()
                    .unwrap_or(&String::from("{}")),
            )
            .header("content-md5", hasher)
            .body(set_struct.data.clone())
            .send()
            .await?,
    )
    .await?;

    Ok(res)
}
