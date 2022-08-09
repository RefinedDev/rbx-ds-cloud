use rbx_ds_cloud::api::datastore_api::increment_entry;
use rbx_ds_cloud::api::endpoint_structs::IncrementEntry;

use reqwest::Response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "API_KEY".to_string();
    let universe_id: u64 = 0;

    let datastore_name: String = "DATASTORE_NAME".to_string();
    let key: String = "ENTRY_KEY".to_string();
    let increment_by: f64 = 0.0;

    let scope: Option<String> = None; // None = default to global
    let user_ids: Option<Vec<u64>> = None;
    let attributes: Option<String> = None;

    let increment_struct = IncrementEntry {
        api_key,
        universe_id,
        datastore_name,
        key,
        increment_by,
        scope,
        user_ids,
        attributes,
    };

    let res: Response = increment_entry(&increment_struct).await?;
    let status_code = res.status();

    println!(
        "Data has been incremented successfully! (Code: {})",
        status_code
    );

    Ok(())
}
