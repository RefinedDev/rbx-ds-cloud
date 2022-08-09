// CLI will print the responses here

use chrono::DateTime;
use rbx_ds_cloud::api::{error_types::Error, response_structs::*};
use reqwest::{header::HeaderMap, Response, StatusCode};

async fn next_page_cursor_check(cursor: Option<String>, func: String) {
    // Double checking if there's a NextPageCursor in the response, since the value is un-defined as mentioned by the documentation, it cannot be None
    if let Some(n) = cursor {
        if !n.is_empty() {
            let msg = format!("All of the {} were not able to be printed.\nNext time use this NextPageCursor value below in the commandline with the '--cursor' or '-c' flag to get the rest of the Datastores\n<<  {}  >>",func, n);
            println!("\n{}", msg);
        }
    }
}

pub async fn list_data_stores_respond(json_data: ListDataStoresResponse) {
    // "Pretty printing" the result
    for (i, v) in json_data.datastores.iter().enumerate() {
        let formatted_time = DateTime::parse_from_rfc3339(v.createdTime.as_str())
            .unwrap()
            .date_naive(); // Convert the weird Date returned from Roblox to an actual read-able Date

        println!(
            "--------------------------\n{}. Name( '{}' ) - DateCreated( {} )",
            i + 1,
            v.name,
            formatted_time
        );
    }

    next_page_cursor_check(json_data.nextPageCursor, "Datastores".to_string()).await;
}

pub async fn list_entries_response(json_data: ListEntriesResponse) {
    // "Pretty printing" the result
    for (i, v) in json_data.keys.iter().enumerate() {
        println!(
            "--------------------------\n{}. Key( {} ) - Scope( {} )",
            i + 1,
            v.key,
            v.scope
        );
    }

    next_page_cursor_check(json_data.nextPageCursor, "Keys".to_string()).await;
}

pub async fn get_entry_response(res: Response) -> Result<(), Error> {
    let body = res.text().await?;
    println!("{}\nData: {}", "-".repeat(body.len()), body);
    Ok(())
}

pub async fn set_entry_response(status_code: StatusCode) {
    println!("\nData was set successfully! (Code: {})", status_code);
}

pub async fn delete_entry_response(status_code: StatusCode) {
    println!("\nEntry was deleted successfully!(Code: {})\n\nPS: The entry has only been marked as 'deleted', according to Roblox's documentation,\na 'tombstone' version for it has been created. Entries are deleted permanently after 30 days.\nMore Info, visit: https://create.roblox.com/docs/open-cloud/data-store-api#delete-entry", status_code);
}

pub async fn increment_entry_response(status_code: StatusCode) {
    println!(
        "\nData has been incremented successfully! (Code: {})",
        status_code
    );
}

pub async fn list_entries_version_response(json_data: ListEntryVersionResponse) {
    println!("{:#?}", json_data.versions);
    next_page_cursor_check(json_data.nextPageCursor, "ListEntryVersions".to_string()).await;
}

pub async fn get_entry_version_response(header_map: &HeaderMap) {
    println!("{:#?}", header_map);
}
