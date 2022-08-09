use rbx_ds_cloud::api::datastore_api::set_entry;
use rbx_ds_cloud::api::endpoint_structs::SetEntry;

use reqwest::Response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "API_KEY".to_string();
    let universe_id: u64 = 0;

    let datastore_name: String = "DATASTORE_NAME".to_string();
    let key: String = "ENTRY_KEY".to_string();
    let data: String = "".to_string(); // should be json_stringify-able (upto 4MB)

    let scope: Option<String> = None; // None = default to global
                                      // see docs for info
    let match_version: Option<String> = None;
    let exclusive_create: Option<bool> = None;
    let user_ids: Option<Vec<u64>> = None;
    let attributes: Option<String> = None;

    let set_struct = SetEntry {
        universe_id,
        api_key,
        datastore_name,
        scope,
        key,
        match_version,
        exclusive_create,
        data,
        user_ids,
        attributes,
    };

    let res: Response = set_entry(&set_struct).await?;
    let status = res.status();

    println!("Success: {}", status);

    Ok(())
}
