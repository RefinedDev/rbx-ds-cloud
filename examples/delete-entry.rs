use rbx_ds_cloud::api::datastore_api::delete_entry;
use rbx_ds_cloud::api::endpoint_structs::DeleteEntry;

use reqwest::Response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "API_KEY".to_string();
    let universe_id: u64 = 0;

    let datastore_name: String = "DATASTORE_NAME".to_string();
    let key: String = "ENTRY_KEY".to_string();
    let scope: Option<String> = None; // None = default to global

    let delete_struct = DeleteEntry {
        api_key,
        universe_id,
        datastore_name,
        key,
        scope,
    };

    let res: Response = delete_entry(&delete_struct).await?;
    let status = res.status();
    println!("Entry was deleted successfully!(Code: {})\n\nPS: The entry has only been marked as 'deleted', according to Roblox's documentation,\na 'tombstone' version for it has been created. Entries are deleted permanently after 30 days.\nMore Info, visit: https://create.roblox.com/docs/open-cloud/data-store-api#delete-entry", status);

    Ok(())
}
