use rbx_ds_cloud::api::datastore_api::list_data_stores;
use rbx_ds_cloud::api::endpoint_structs::ListDataStores;
use rbx_ds_cloud::api::response_structs::ListDataStoresResponse;

use reqwest::Response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error> > {
    let api_key = "API_KEY".to_string();
    let universe_id: u64 = 0;

    let limit: u64 = 10; // the amount of datastores to be returned from a single request, max is 11 per request.
    // optional args are none cuz i dont need 'em
    let prefix: Option<String> = None; // returns datastores specified by the prefix, 
    let cursor: Option<String> = None; // cursor for the next set of data, read the documentation for more info

    let lds_struct = ListDataStores { 
        universe_id,
        api_key,
        limit,
        prefix,
        cursor,
    };

    let res: Response = list_data_stores(&lds_struct).await?;
    let json_response = res.json::<ListDataStoresResponse>().await?;
    println!("{:#?}", json_response);

    Ok(())
}