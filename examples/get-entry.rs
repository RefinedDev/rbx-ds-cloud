use rbx_ds_cloud::api::datastore_api::get_entry;
use rbx_ds_cloud::api::endpoint_structs::GetEntry;

use reqwest::Response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "API_KEY".to_string();
    let universe_id: u64 = 0;

    let datastore_name: String = "DATASTORE_NAME".to_string();
    let key: String = "ENTRY_KEY".to_string();
    let scope: Option<String> = None; // None = default to global

    let entry_struct = GetEntry {
        api_key,
        universe_id,
        datastore_name,
        key,
        scope,
    };

    let res: Response = get_entry(&entry_struct).await?;

    let body = res.text().await?;
    println!("{}", body);

    Ok(())
}
